use crate::device::base::ublock_device::UBlockDevice;

pub struct Ubio {
    pub dir: UbioDir,
    pub dataBuffer: Option<Vec<u8>>,
    pub callback: Option<fn()->()>,
    pub lba: u64,
    pub uBlock: Option<Box<dyn UBlockDevice>>,
    pub arrayId: i32,
    pub arrayName: String,
}

impl Ubio {
    pub fn new(dir: UbioDir, lba: u64, dataBuffer: Vec<u8>) -> Ubio {
        Ubio {
            dir: dir,
            dataBuffer: Some(dataBuffer),
            callback: None,
            lba: lba,
            uBlock: None,
            arrayId: 0,
            arrayName: "".to_string()
        }
    }
}

pub enum UbioDir {
    Read,
    Write,
}