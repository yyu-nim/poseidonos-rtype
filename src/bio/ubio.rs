use std::fmt::{Debug, Formatter};
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use crate::bio::data_buffer::DataBuffer;
use crate::bio::volume_io::VolumeIo;
use crate::device::base::ublock_device::UBlockDevice;
use crate::event_scheduler::callback::Callback;
use crate::include::backend_event::BackendEvent;
use crate::include::i_array_device::IArrayDevice;
use crate::include::memory::{BLOCK_SIZE, SECTOR_SIZE};

pub type CallbackClosure = Box<dyn FnMut(&Vec<u8>)->()>;

pub struct Ubio {
    pub dir: UbioDir,
    pub dataBuffer: Option<DataBuffer>,
    pub callback: Option<Box<dyn Callback>>,
    pub origin: Option<Arc<Mutex<Ubio>>>,
    pub lba: u64,
    pub uBlock: Option<Box<dyn UBlockDevice>>,
    pub arrayDev: Option<Box<dyn IArrayDevice>>,
    pub arrayId: i32,
    pub arrayName: String,
    pub eventIoType: BackendEvent,
}

impl Ubio {
    pub fn new(buffer: Option<Arc<Mutex<Vec<u8>>>>, callback: Option<Box<dyn Callback>>, array_id: i32) -> Ubio {
        let data_buffer = if let Some(buffer) = buffer {
            Some(DataBuffer::new(buffer))
        } else {
            None
        };
        Ubio {
            dir: UbioDir::Read /* default in pos-cpp */,
            dataBuffer: data_buffer,
            callback: callback,
            origin: None /* default in pos-cpp */,
            lba: 0 /* default in pos-cpp */,
            uBlock: None,
            arrayDev: None,
            arrayId: array_id,
            arrayName: "Uninitialized".to_string(),
            eventIoType: BackendEvent::BackendEvent_Unknown,
        }
    }

    pub fn CheckPbaSet(&self) -> bool {
        self.arrayDev.is_some()
    }

    pub fn GetBuffer(&mut self) -> Option<DataBuffer> {
        if self.dataBuffer.is_none() {
            return None;
        }

        Some(self.dataBuffer.as_ref().unwrap().clone())
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

#[derive(Debug, Clone, Copy)]
pub enum UbioDir {
    Read,
    Write,
}

pub const BYTES_PER_UNIT: u32 = SECTOR_SIZE as u32;
pub const UNITS_PER_BLOCK: u32 = (BLOCK_SIZE / BYTES_PER_UNIT as usize) as u32;