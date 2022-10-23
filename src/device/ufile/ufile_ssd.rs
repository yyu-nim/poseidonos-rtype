use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use log::{info, warn};
use crate::bio::ubio::{Ubio, UbioDir};
use crate::device::base::ublock_device::UBlockDevice;
use crate::generated::bindings::spdk_new_thread_fn;

pub struct UfileSsd {
    filePath: PathBuf,
    fileSize: usize,
    file: Option<File>,
}

impl UBlockDevice for UfileSsd {
    fn SubmitAsyncIO(&self, bio: &mut Ubio) -> i32 {
        // 사실은 SyncIO의 구현...
        let lba = bio.lba;
        match bio.dir {
            UbioDir::Read => {
                let mut buf = bio.dataBuffer.as_mut().expect("Ubio must have a read buffer!");
                self.read(lba, &mut buf);
                bio.dataBuffer = Some(buf.clone()); // could be expensive
            }
            UbioDir::Write => {
                let buf = bio.dataBuffer.as_ref().expect("Ubio must have a write buffer!");
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
        let mut file = File::options()
            .create(true)
            .read(true)
            .write(true)
            .open(self.filePath.clone())
            .expect(format!("Failed to create a device file {:?}", self.filePath).as_str());

        file.set_len(self.fileSize as u64);
        info!("Opening a file {:?} and truncating to {} bytes", self.filePath, self.fileSize);
        self.file = Some(file);
        true
    }

    fn Close(&self) -> u32 {
        match &self.file {
            None => {
                warn!("Cannot close the non-open file! {:?}", self.filePath);
            }
            Some(f) => {
                f.sync_all().expect(
                    format!("failed to sync_all for {:?}", self.filePath).as_str());
                info!("Closing {:?}", self.filePath);
            }
        }
        0
    }

    fn clone_box(&self) -> Box<dyn UBlockDevice> {
        let mut new_ufile_ssd = UfileSsd {
            filePath: self.filePath.clone(),
            fileSize: self.fileSize.clone(),
            file: None,
        };
        if self.file.is_some() {
            new_ufile_ssd.Open(); // TODO: 이런 방식의 Clone은 문제가 있을 지도. 동일 file을 각각 열어서 쓰면 consistency 문제가 있을 수 있으니.
        }
        Box::new(new_ufile_ssd)
    }
}

impl UfileSsd {
    pub fn new(filePath: PathBuf, fileSize: usize) -> UfileSsd {
        UfileSsd {
            filePath,
            fileSize,
            file: None,
        }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    fn read(&self, lba: u64, buf: &mut Vec<u8>) {
        let mut f = self.file.as_ref().unwrap();
        f.seek(SeekFrom::Start(lba * 512));
        f.read(buf);
    }

    fn write(&self, lba: u64, buf: &Vec<u8>) {
        let mut f = self.file.as_ref().unwrap();
        f.seek(SeekFrom::Start(lba * 512));
        f.write(&buf);
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use log::info;
    use crate::bio::ubio::{Ubio, UbioDir};
    use crate::device::base::ublock_device::UBlockDevice;
    use crate::device::ufile::ufile_ssd::UfileSsd;

    #[test]
    fn create_open_close_ufile_ssd() {
        let mut ssd = UfileSsd::new("/tmp/dev1".into(), 1024*1024);
        assert_eq!(true, ssd.Open());
        ssd.Close();

        let m = fs::metadata(PathBuf::from("/tmp/dev1")).unwrap();
        assert_eq!(true, m.is_file());
    }

    #[test]
    fn open_write_read_ufile_ssd() {
        let mut ssd = UfileSsd::new("/tmp/dev2".into(), 1024*1024);
        ssd.Open();
        let lba_locations = vec![0, 500, 1000];
        let expected_pattern = vec![0, 1, 2, 3, 4, 5, 6, 7];
        for lba in &lba_locations {
            let buf : Vec<u8> = expected_pattern.clone(); // 8 bytes signature
            let mut ubio = Ubio::new(UbioDir::Write, lba.clone(), buf, Box::new(|_| {}));
            ssd.SubmitAsyncIO(&mut ubio);
        }

        for lba in &lba_locations {
            let buf : Vec<u8> = vec![0; 8]; // 8 bytes buffer
            let mut ubio = Ubio::new(UbioDir::Read, lba.clone(), buf, Box::new(|_| {}));
            ssd.SubmitAsyncIO(&mut ubio);
            assert_eq!(expected_pattern, ubio.dataBuffer.unwrap());
        }
        ssd.Close();
    }
}