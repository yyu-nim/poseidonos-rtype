use std::fmt::{Debug, Formatter};
use std::sync::mpsc::Sender;
use crate::device::base::ublock_device::UBlockDevice;
use crate::include::i_array_device::IArrayDevice;

// TODO: Callback needs to be generic (refer to Callback.cpp)
pub type CallbackClosure = Box<dyn FnMut(&Vec<u8>)->()>;

pub struct Ubio {
    pub dir: UbioDir,
    pub dataBuffer: Option<Vec<u8>>,
    pub callback: CallbackClosure,
    pub lba: u64,
    pub uBlock: Option<Box<dyn UBlockDevice>>,
    pub arrayDev: Option<Box<dyn IArrayDevice>>,
    pub arrayId: i32,
    pub arrayName: String,
}

impl Ubio {
    pub fn new(dir: UbioDir, lba: u64, dataBuffer: Vec<u8>, callback: CallbackClosure) -> Ubio {
        Ubio {
            dir: dir,
            dataBuffer: Some(dataBuffer),
            callback: callback,
            lba: lba,
            uBlock: None,
            arrayDev: None,
            arrayId: 0,
            arrayName: "".to_string()
        }
    }
}

impl Debug for Ubio {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ubio")
            .field("dir", &self.dir)
            .field("lba", &self.lba)
            .field("arrayName", &self.arrayName)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub enum UbioDir {
    Read,
    Write,
}