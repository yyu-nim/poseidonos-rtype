use crate::bio::ubio::{Ubio, UbioDir};
use crate::device::base::device_property::{DeviceClass, DeviceProperty, DeviceType};
use crate::device::base::ublock_device::UBlockDevice;
use crate::generated::bindings::spdk_new_thread_fn;
use log::{info, warn};
use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use crate::bio::data_buffer::DataBuffer;
use crate::event_scheduler::callback::Callback;

pub struct UfileSsd {
    filePath: PathBuf,
    fileSize: usize,
    file: Arc<Mutex<Option<File>>>,
    property: DeviceProperty,
}

impl UBlockDevice for UfileSsd {
    fn SubmitAsyncIO(&self, bio: Arc<Mutex<Ubio>>) -> i32 {
        // 사실은 SyncIO의 구현...
        let mut bio = bio.lock().unwrap();
        let lba = bio.lba;
        match bio.dir {
            UbioDir::Read => {
                let mut read_buffer= bio.dataBuffer.as_ref().unwrap().buffer.lock().unwrap();
                self.read(lba, &mut read_buffer);
            }
            UbioDir::Write => {
                let mut write_buffer = bio.dataBuffer.as_ref().unwrap().buffer.lock().unwrap();
                self.write(lba, &write_buffer);
            }
        }

        if bio.callback.is_some() {
            let callback_closure = bio.callback.as_deref_mut().unwrap();
            Callback::Execute(callback_closure);
        }

        0
    }

    fn CompleteIOs(&self) -> i32 {
        // SubmitAsyncIO를 sync 버전으로 구현했기 때문에 사실 CompelteIOs에서
        // 할건 없다.
        0
    }

    fn Open(&mut self) -> bool {
        let mut file = File::options()
            .create(true)
            .read(true)
            .write(true)
            .open(self.filePath.clone())
            .expect(format!("Failed to create a device file {:?}", self.filePath).as_str());

        file.set_len(self.fileSize as u64);
        info!(
            "Opening a file {:?} and truncating to {} bytes",
            self.filePath, self.fileSize
        );
        self.file = Arc::new(Mutex::new(Some(file)));
        true
    }

    fn Close(&self) -> u32 {
        let f = self.file.lock().unwrap();
        match &*f {
            Some(f_handle) => {
                f_handle
                    .sync_all()
                    .expect(format!("failed to sync_all for {:?}", self.filePath).as_str());
            }
            None => {
                warn!("Cannot close the non-open file! {:?}", self.filePath);
            }
        };

        0
    }

    fn clone_box(&self) -> Box<dyn UBlockDevice> {
        let new_ufile_ssd = UfileSsd {
            filePath: self.filePath.clone(),
            fileSize: self.fileSize.clone(),
            file: self.file.clone(),
            property: self.property.clone(),
        };
        Box::new(new_ufile_ssd)
    }

    fn GetName(&self) -> String {
        self.property.name.clone()
    }

    fn GetSN(&self) -> String {
        self.property.sn.clone()
    }

    fn SetClass(&mut self, class: DeviceClass) {
        self.property.deviceClass = Some(class);
    }
}

impl UfileSsd {
    pub fn new(filePath: PathBuf, fileSize: usize) -> UfileSsd {
        let device_name = filePath
            .file_name()
            .unwrap()
            .to_owned()
            .to_str()
            .unwrap()
            .to_owned();
        UfileSsd {
            filePath,
            fileSize,
            file: Arc::new(Mutex::new(None)),
            property: DeviceProperty::new(DeviceType::SSD, device_name, fileSize),
        }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    fn read(&self, lba: u64, buf: &mut Vec<u8>) {
        let guard = self.file.lock().unwrap();
        let mut f = (*guard).as_ref().unwrap();
        f.seek(SeekFrom::Start(lba * 512));
        f.read(buf);
    }

    fn write(&self, lba: u64, buf: &Vec<u8>) {
        let guard = self.file.lock().unwrap();
        let mut f = guard.as_ref().unwrap();
        f.seek(SeekFrom::Start(lba * 512));
        f.write(&buf);
    }
}

#[cfg(test)]
mod tests {
    use crate::bio::ubio::{Ubio, UbioDir};
    use crate::device::base::device_property::DeviceClass;
    use crate::device::base::ublock_device::UBlockDevice;
    use crate::device::ufile::ufile_ssd::UfileSsd;
    use log::info;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};
    use crossbeam::sync::Parker;
    use crate::event_scheduler::callback::Callback;
    use crate::io_scheduler::io_dispatcher::tests::WaitReadDone;

    #[test]
    fn create_open_close_ufile_ssd() {
        let mut ssd = UfileSsd::new("/tmp/dev1".into(), 1024 * 1024);
        assert_eq!(true, ssd.Open());
        ssd.Close();

        let m = fs::metadata(PathBuf::from("/tmp/dev1")).unwrap();
        assert_eq!(true, m.is_file());
    }

    #[test]
    fn open_write_read_ufile_ssd() {
        let mut ssd = UfileSsd::new("/tmp/dev2".into(), 1024 * 1024);
        ssd.Open();
        let lba_locations = vec![0, 500, 1000];
        let expected_pattern = vec![0, 1, 2, 3, 4, 5, 6, 7];
        for lba in &lba_locations {
            let buf: Arc<Mutex<Vec<u8>>> = {
                let p = expected_pattern.clone(); // 8 bytes signature
                Arc::new(Mutex::new(p))
            };
            let mut ubio = Ubio::new(Some(buf), None /* no callback */,
                                     0);
            ubio.lba = lba.clone();
            ubio.dir = UbioDir::Write;
            ssd.SubmitAsyncIO(Arc::new(Mutex::new(ubio)));
        }

        for lba in &lba_locations {
            let buf: Arc<Mutex<Vec<u8>>> = {
                let p = vec![0; 8];
                Arc::new(Mutex::new(p))
            }; // 8 bytes buffer
            let parker = crossbeam::sync::Parker::new();
            let unparker = parker.unparker().clone();
            let read_callback: Box<dyn Callback> = Box::new(WaitReadDone{
                unparker: unparker,
                done: false
            });
            let mut ubio = Ubio::new(Some(buf),
                                     Some(read_callback),
                                     0);
            ubio.lba = lba.clone();
            ubio.dir = UbioDir::Read;
            let ubio = Arc::new(Mutex::new(ubio));
            ssd.SubmitAsyncIO(ubio.clone());
            parker.park();
            let actual_pattern = {
                let inner_ubio = ubio.lock().unwrap();
                let bytes = inner_ubio
                    .dataBuffer.as_ref().unwrap().buffer.lock().unwrap();
                bytes.clone()
            };

            assert_eq!(expected_pattern, actual_pattern);
        }
        ssd.Close();
    }

    #[test]
    fn set_and_get_property() {
        let mut ssd = UfileSsd::new("/tmp/dev3".into(), 1024 * 1024);
        assert_eq!(ssd.GetName(), "dev3".to_string());
        assert_eq!(ssd.property.GetClass(), "".to_string());

        ssd.SetClass(DeviceClass::ARRAY);
        assert_eq!(ssd.property.GetClass(), DeviceClass::ARRAY.to_string());
    }
}
