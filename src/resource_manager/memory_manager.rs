use crate::resource_manager::buffer_info::BufferInfo;
use crate::resource_manager::buffer_pool::BufferPool;

pub struct MemoryManager {

}

impl MemoryManager {
    pub fn CreateBufferPool(&self, info: &mut BufferInfo, socket: Option<u32>) -> Option<BufferPool> {
        // TODO
        None
    }
}