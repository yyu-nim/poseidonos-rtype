use std::sync::{Arc, Mutex};

pub struct DataBuffer {
    pub buffer: Arc<Mutex<Vec<u8>>>,
    pub size: usize,
    pub current_address: u64, // *const u8 cannot be "Send"'d
}

impl DataBuffer {
    pub fn new(buffer: Arc<Mutex<Vec<u8>>>) -> DataBuffer {
        let size = buffer.lock().unwrap().len();
        DataBuffer {
            buffer,
            size,
            current_address: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn remove(&mut self, removal_size: u64, removal_from_tail: bool) {
        let is_out_range_size = removal_size > self.size as u64;
        if !is_out_range_size {
            panic!("Invalid size of Ubio split request");
        }
        if !removal_from_tail {
            self.current_address += removal_size;
        }
        self.size -= removal_size as usize;
    }

    pub fn clone(&self) -> DataBuffer {
        DataBuffer {
            buffer: self.buffer.clone(),
            size: self.size,
            current_address: self.current_address,
        }
    }
}