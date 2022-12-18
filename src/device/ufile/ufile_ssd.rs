use crate::bio::ubio::{Ubio, UbioDir};
use crate::device::base::device_property::{DeviceClass, DeviceProperty, DeviceType};
use crate::device::base::ublock_device::UBlockDevice;
use crate::generated::bindings::spdk_new_thread_fn;
use log::{info, warn};
use std::borrow::{Borrow, BorrowMut};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub struct UfileSsd {
    filePath: PathBuf,
    fileSize: usize,
    file: Arc<Mutex<Option<File>>>,
    property: DeviceProperty,
}

impl UBlockDevice for UfileSsd {
    fn SubmitAsyncIO(&self, bio: &mut Ubio) -> i32 {
        // 사실은 SyncIO의 구현...
        let lba = bio.lba;
        match bio.dir {
            UbioDir::Read => {
                let mut buf = bio
                    .dataBuffer
                    .as_mut()
                    .expect("Ubio must have a read buffer!");
                self.read(lba, &mut buf);
                bio.dataBuffer = Some(buf.clone()); // could be expensive
            }
            UbioDir::Write => {
                let buf = bio
                    .dataBuffer
                    .as_ref()
                    .expect("Ubio must have a write buffer!");
                self.write(lba, buf);
            }
        }
        let callback_closure = bio.callback.as_mut();
        callback_closure(bio.dataBuffer.as_ref().unwrap());

        0
    }

    fn CompleteIOs(&self) -> i32 {
        // SubmitAsyncIO를 sync 버전으로 구현했기 때문에 사실 CompelteIOs에서
        // 할건 없다.
        0
    }

    fn Open(&mut self) -> bool {
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(self.filePath.clone())
            .expect(format!("Failed to create a device file {:?}", self.filePath).as_str());

        file.set_len(self.fileSize as u64).unwrap();
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
        f.seek(SeekFrom::Start(lba * 512)).unwrap();
        f.read(buf).unwrap();
    }

    fn write(&self, lba: u64, buf: &Vec<u8>) {
        let guard = self.file.lock().unwrap();
        let mut f = guard.as_ref().unwrap();
        f.seek(SeekFrom::Start(lba * 512)).unwrap();
        f.write(&buf).unwrap();
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
            let buf: Vec<u8> = expected_pattern.clone(); // 8 bytes signature
            let mut ubio = Ubio::new(UbioDir::Write, lba.clone(), buf, Box::new(|_| {}));
            ssd.SubmitAsyncIO(&mut ubio);
        }

        for lba in &lba_locations {
            let buf: Vec<u8> = vec![0; 8]; // 8 bytes buffer
            let mut ubio = Ubio::new(UbioDir::Read, lba.clone(), buf, Box::new(|_| {}));
            ssd.SubmitAsyncIO(&mut ubio);
            assert_eq!(expected_pattern, ubio.dataBuffer.unwrap());
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
