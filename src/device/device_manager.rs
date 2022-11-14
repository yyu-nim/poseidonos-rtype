use std::borrow::Borrow;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use lazy_static::lazy_static;
use log::{info, warn};
use crate::device::base::ublock_device::UBlockDevice;
use crate::device::i_io_dispatcher::IIODispatcher;
use crate::device::ufile::ufile_ssd::UfileSsd;
use crate::include::pos_event_id::PosEventId;
use crate::io_scheduler::io_dispatcher::IODispatcherSingleton;
lazy_static!{
    pub static ref DeviceManagerSingleton: DeviceManager = {
        DeviceManager::new(None)
    };
}

// TODO: config 파일에 넣어 명시적으로 알 수 있도록 하기
pub struct DeviceManagerConfig {
    pub dir_to_lookup: &'static str,
    pub device_prefix: &'static str,
    pub num_devices_to_create: u32,
    pub device_size_bytes: usize,
}

impl Default for DeviceManagerConfig {
    fn default() -> Self {
        DeviceManagerConfig {
            dir_to_lookup: "/tmp/",
            device_prefix: "UfileSsd.",
            num_devices_to_create: 4,
            device_size_bytes: 100 * 1024 * 1024,
        }
    }
}

impl Clone for DeviceManagerConfig {
    fn clone(&self) -> Self {
        DeviceManagerConfig {
            dir_to_lookup: self.dir_to_lookup,
            device_prefix: self.device_prefix,
            num_devices_to_create: self.num_devices_to_create,
            device_size_bytes: self.device_size_bytes,
        }
    }
}

pub struct DeviceManager {
    devices: Vec<Box<dyn UBlockDevice>>,
    ioDispatcher: &'static IODispatcherSingleton,
    config: DeviceManagerConfig,
}

impl DeviceManager {
    pub fn new(config: Option<DeviceManagerConfig>) -> DeviceManager {
        let io_dispatcher_singleton = IODispatcherSingleton.borrow();
        let config = config.unwrap_or(Default::default());
        DeviceManager {
            devices: vec![],
            ioDispatcher: io_dispatcher_singleton,
            config,
        }
    }

    pub fn Initialize(&self) {
        // 생성자에 의존성 주입하는 것으로 변경하였음. 따라서 이 fn은 no-op으로 변경.
    }

    pub fn ScanDevs(&mut self) {
        let num_devices = self.devices.len();
        if num_devices == 0 {
            info!("# of devices was 0. Performing initial scan...");
            self._PrepareIOWorker();
            self._InitScan();
            self._PrepareDevices();
            self._StartMonitoring();
        } else {
            info!("# of devices was {}. Performing re-scan...", num_devices);
            self._Rescan();
        }
    }

    pub fn IterateDevicesAndDoFunc(&self, mut func: Box<dyn FnMut(&Box<dyn UBlockDevice>)>) -> Result<(), PosEventId> {
        // POS에서는 devicesLocal copy를 두어서 critical section을
        // 줄였으나, rtype에서는 이 부분 리팩토링이 추후 예상되므로, 그 최적화는
        // 넣지 않는다.
        if self.devices.len() == 0 {
            return Err(PosEventId::DEVICEMGR_DEVICE_NOT_FOUND)
        }

        for uBlockDev in self.devices.iter() {
            func(uBlockDev);
        }

        Ok(())
    }

    pub fn Close(&self) {
        for dev in self.devices.iter() {
            dev.Close();
        }
    }

    fn _PrepareIOWorker(&self) {
        self.ioDispatcher.lock().unwrap().AddIOWorker();
    }

    fn _InitScan(&mut self) {
        // 현재 pos-rtype에는 UfileSsd 한 종류의 UBlockDevice만 존재하므로,
        // "DeviceDriver" 추상화 레이어 없이 곧바로 UBlockDevice 만들어서,
        // "devices"에 추가해도 좋을 것.
        let mut _devs : Vec<Box<dyn UBlockDevice>> = Vec::new();
        if let Ok(r) = PathBuf::from(self.config.dir_to_lookup).read_dir() {
            for entry in r {
                let dir_entry = entry.unwrap();
                let file = dir_entry.file_name();
                if file.to_str().unwrap().starts_with(self.config.device_prefix) {
                    let file_size = dir_entry.metadata().unwrap().size();
                    let device = UfileSsd::new(dir_entry.path(), file_size as usize);
                    _devs.push(device.boxed());
                }
            }
        } else {
            warn!("Failed to list files from {:?}", self.config.dir_to_lookup);
        }

        if _devs.len() == 0 {
            // 혹시 디바이스가 하나도 스캔되지 않았으면, default 설정으로
            // 100 MB file * 4 개를 생성한다.
            info!("No device has been scanned. Creating {} new devices...", self.config.num_devices_to_create);
            for i in 0..self.config.num_devices_to_create {
                let device_file = format!("{}{}{}", self.config.dir_to_lookup, self.config.device_prefix, i);
                info!("Creating UBlockDevice at {}...", device_file);
                let device = UfileSsd::new(PathBuf::from(device_file), self.config.device_size_bytes);
                _devs.push(device.boxed());
            }
        }

        // move the resulting devices list to the member variable
        info!("Scanned {} devices...", _devs.len());
        self.devices = _devs;
    }
    fn _PrepareDevices(&mut self) {
        // IODispatcher::AddDeviceForReactor() eventually calls "dev->Open()"
        // Hence, we can do something similar in the current context
        for dev in &mut self.devices {
            dev.Open();
        }
    }
    fn _StartMonitoring(&self) {
        // TODO
    }
    fn _Rescan(&mut self) {
        // 일단 혹시 열려 있을지 모르는 device들을 닫고
        for dev in &mut self.devices {
            dev.Close();
        }

        // 현재는 InitScan과 Rescan의 차이는 없음.
        self._InitScan();
        self._PrepareDevices();
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::path::PathBuf;
    use log::{info, LevelFilter};
    use crate::device::base::ublock_device::UBlockDevice;
    use crate::device::device_manager::{DeviceManager, DeviceManagerConfig};
    use crate::device::ufile::ufile_ssd::UfileSsd;

    fn setup(dm_config: &DeviceManagerConfig) {
        // set up the logger for the test context
        env_logger::builder().is_test(true).try_init();

        // clean up device files
        cleanup_device_files(dm_config);
    }

    fn cleanup_device_files(dm_config: &DeviceManagerConfig) {
        let device_dir = PathBuf::from(dm_config.dir_to_lookup).read_dir();
        if let Ok(d) = device_dir {
            for entry in d {
                let file_path = entry.unwrap().path();
                let file_name = file_path.file_name().unwrap().to_str().unwrap();
                if file_name.starts_with(dm_config.device_prefix) {
                    info!("Removing a test file at {:?}", file_path);
                    fs::remove_file(file_path).unwrap();
                }
            }
        }
    }

    fn create_device_file(dev_num: u32, dm_config: &DeviceManagerConfig) {
        let device_dir = PathBuf::from(dm_config.dir_to_lookup);
        let device_file = device_dir.join(format!("{}{}", dm_config.device_prefix, dev_num));
        let mut dev = UfileSsd::new(device_file, 1024 * 1024);
        dev.Open();
        dev.Close();
    }

    #[test]
    fn test_if_scanning_with_default_conf_works() {
        let dm_config = DeviceManagerConfig {
            dir_to_lookup: "/tmp/",
            device_prefix: "TestUfileSsd.",
            num_devices_to_create: 4,
            device_size_bytes: 1 * 1024 * 1024,
        };
        setup(&dm_config);

        // Given: an initialized device manager
        let mut dm = DeviceManager::new(Some(dm_config.clone()));
        dm.Initialize();

        // When 1: the device manager scans devices
        dm.ScanDevs();

        // Then 1: the device manager should have scanned the default number of devices
        assert_eq!(dm_config.num_devices_to_create, dm.devices.len() as u32);

        // When 2: the device manager scans again,
        dm.ScanDevs();

        // Then 2: the device manager should have the same number of devices,
        assert_eq!(dm_config.num_devices_to_create, dm.devices.len() as u32);
    }

    #[test]
    fn test_if_rescanning_works_for_the_varying_number_of_devices() {
        let dm_config = DeviceManagerConfig {
            dir_to_lookup: "/tmp/",
            device_prefix: "TestUfileSsd.",
            num_devices_to_create: 3,
            device_size_bytes: 1 * 1024 * 1024,
        };
        setup(&dm_config);

        // Given: "three" UBlockDevices
        for dev_num in 0..3 {
            create_device_file(dev_num, &dm_config);
        }
        let mut dm = DeviceManager::new(Some(dm_config.clone()));
        dm.Initialize();
        dm.ScanDevs();
        assert_eq!(3, dm.devices.len());

        // When: a new UBlockDevice is added and Device Manager performs "Rescan"
        create_device_file(4 /* new device's number */, &dm_config);
        dm.ScanDevs();

        // Then: Device Manager should be aware of "four" UBlockDevices
        assert_eq!(4, dm.devices.len());
    }

}