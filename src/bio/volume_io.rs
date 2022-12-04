use crate::bio::ubio::{CallbackClosure, Ubio, UbioDir};
use crate::event_scheduler::callback::Callback;

pub struct VolumeIo {
    pub array_id: u32,
    pub vol_id: u32,
    pub sector_rba: u64,
    pub sector_size: u64,
    pub dir: UbioDir,
    pub callback: Box<dyn Callback>, // Note that ubio has its own callback, so no worry about being overwritten.
    pub data_buffer: Vec<u8>,
}

impl VolumeIo {
    pub fn new(array_id: u32, vol_id: u32, sector_rba: u64, sector_size: u64,
               dir: UbioDir, mut dataBuffer: Vec<u8>, volume_io_callback: Box<dyn Callback>) -> VolumeIo {
        VolumeIo {
            array_id,
            vol_id,
            sector_rba,
            sector_size,
            dir,
            callback: volume_io_callback,
            data_buffer: Vec::new(),
        }
    }
}