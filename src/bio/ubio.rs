use crate::device::base::ublock_device::UBlockDevice;
use crate::include::i_array_device::IArrayDevice;

#[derive(Debug)]
pub struct Ubio {
    pub dir: UbioDir,
    pub dataBuffer: Option<Vec<u8>>,
    pub callback: Option<fn()->()>,
    pub lba: u64,
    pub uBlock: Option<Box<dyn UBlockDevice>>,
    pub arrayDev: Option<Box<dyn IArrayDevice>>,
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
            arrayDev: None,
            arrayId: 0,
            arrayName: "".to_string()
        }
    }
}

#[derive(Debug)]
pub enum UbioDir {
    Read,
    Write,
}