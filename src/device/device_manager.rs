use std::borrow::Borrow;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use lazy_static::lazy_static;
use log::{info, warn};
use crate::device::base::ublock_device::UBlockDevice;
use crate::device::i_io_dispatcher::IIODispatcher;
use crate::device::ufile::ufile_ssd::UfileSsd;
use crate::io_scheduler::io_dispatcher::IODispatcherSingleton;
lazy_static!{
    pub static ref DeviceManagerSingleton: DeviceManager = {
        DeviceManager::new()
    };
}

// TODO: config 파일에 넣어 명시적으로 알 수 있도록 하기
const DEF_DEVICE_DIR : &str = "/tmp/";
const DEF_DEVICE_NAME_PREFIX : &str = "UfileSsd.";
const DEF_NUM_OF_DEVICES : usize = 4;
const DEF_SIZE_OF_DEVICE : usize = 100 * 1024 * 1024; // 100 MB

pub struct DeviceManager {
    devices: Vec<Box<dyn UBlockDevice>>,
    ioDispatcher: &'static IODispatcherSingleton,
}

impl DeviceManager {
    pub fn new() -> DeviceManager {
        let io_dispatcher_singleton = IODispatcherSingleton.borrow();
        DeviceManager {
            devices: vec![],
            ioDispatcher: io_dispatcher_singleton,
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

    fn _PrepareIOWorker(&self) {
        self.ioDispatcher.lock().unwrap().AddIOWorker();
    }

    fn _InitScan(&mut self) {
        // 현재 pos-rtype에는 UfileSsd 한 종류의 UBlockDevice만 존재하므로,
        // "DeviceDriver" 추상화 레이어 없이 곧바로 UBlockDevice 만들어서,
        // "devices"에 추가해도 좋을 것.
        let mut _devs : Vec<Box<dyn UBlockDevice>> = Vec::new();
        if let Ok(r) = PathBuf::from(DEF_DEVICE_DIR).read_dir() {
            for entry in r {
                let dir_entry = entry.unwrap();
                let file = dir_entry.file_name();
                if file.to_str().unwrap().starts_with(DEF_DEVICE_NAME_PREFIX) {
                    let file_size = dir_entry.metadata().unwrap().size();
                    let device = UfileSsd::new(dir_entry.path(), file_size as usize);
                    _devs.push(device.boxed());
                }
            }
        } else {
            warn!("Failed to list files from {:?}", DEF_DEVICE_DIR);
        }

        if _devs.len() == 0 {
            // 혹시 디바이스가 하나도 스캔되지 않았으면, default 설정으로
            // 100 MB file * 4 개를 생성한다.
            info!("No device has been scanned. Creating {} new devices...", DEF_NUM_OF_DEVICES);
            for i in 0..DEF_NUM_OF_DEVICES {
                let device_file = format!("{}{}{}", DEF_DEVICE_DIR, DEF_DEVICE_NAME_PREFIX, i);
                info!("Creating UBlockDevice at {}...", device_file);
                let device = UfileSsd::new(PathBuf::from(device_file), DEF_SIZE_OF_DEVICE);
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
    use crate::device::device_manager::{DEF_DEVICE_DIR, DEF_DEVICE_NAME_PREFIX, DEF_NUM_OF_DEVICES, DeviceManager};
    use crate::device::ufile::ufile_ssd::UfileSsd;

    fn setup() {
        // set up the logger for the test context
        env_logger::builder().is_test(true).try_init();

        // clean up device files
        cleanup_device_files();
    }

    fn cleanup_device_files() {
        let device_dir = PathBuf::from(DEF_DEVICE_DIR).read_dir();
        if let Ok(d) = device_dir {
            for entry in d {
                let file_path = entry.unwrap().path();
                let file_name = file_path.file_name().unwrap().to_str().unwrap();
                if file_name.starts_with(DEF_DEVICE_NAME_PREFIX) {
                    info!("Removing a test file at {:?}", file_path);
                    fs::remove_file(file_path).unwrap();
                }
            }
        }
    }

    fn create_device_file(dev_num: u32) {
        let device_dir = PathBuf::from(DEF_DEVICE_DIR);
        let device_file = device_dir.join(format!("{}{}", DEF_DEVICE_NAME_PREFIX, dev_num));
        let mut dev = UfileSsd::new(device_file, 1024 * 1024);
        dev.Open();
        dev.Close();
    }

    #[test]
    fn test_if_scanning_with_default_conf_works() {
        setup();

        // Given: an initialized device manager
        let mut dm = DeviceManager::new();
        dm.Initialize();

        // When 1: the device manager scans devices
        dm.ScanDevs();

        // Then 1: the device manager should have scanned the default number of devices
        assert_eq!(DEF_NUM_OF_DEVICES, dm.devices.len());

        // When 2: the device manager scans again,
        dm.ScanDevs();

        // Then 2: the device manager should have the same number of devices,
        assert_eq!(DEF_NUM_OF_DEVICES, dm.devices.len());
    }

    #[test]
    fn test_if_rescanning_works_for_the_varying_number_of_devices() {
        setup();

        // Given: "three" UBlockDevices
        for dev_num in 0..3 {
            create_device_file(dev_num);
        }
        let mut dm = DeviceManager::new();
        dm.Initialize();
        dm.ScanDevs();
        assert_eq!(3, dm.devices.len());

        // When: a new UBlockDevice is added and Device Manager performs "Rescan"
        create_device_file(4 /* new device's number */);
        dm.ScanDevs();

        // Then: Device Manager should be aware of "four" UBlockDevices
        assert_eq!(4, dm.devices.len());
    }

}