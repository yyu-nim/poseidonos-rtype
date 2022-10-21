use log::info;
use crate::array_models::interface::i_array_info::ArrayInfo;
use crate::array_models::interface::i_mount_sequence::IMountSequence;

pub struct Allocator;

impl Allocator {
    pub fn new(arrayInfo: ArrayInfo) -> Allocator {
        info!("Creating Allocator for {}", arrayInfo.name);
        Allocator
    }
}

impl IMountSequence for Allocator {
    fn Init(&self) -> i32 {
        info!("TODO: Init() for Allocator...");
        0
    }

    fn Dispose(&self) {
        info!("TODO: Dispose() for Allocator...");
    }

    fn Shutdown(&self) {
        info!("TODO: Shutdown() for Allocator...");
    }

    fn Flush(&self) {
        info!("TODO: Flush() for Allocator...");
    }
}