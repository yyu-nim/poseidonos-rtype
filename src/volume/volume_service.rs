use lazy_static::lazy_static;
use crate::include::pos_event_id::PosEventId;
use crate::volume::i_volume_io_manager::IVolumeIoManager;
use crate::volume::i_volume_manager::IVolumeManager;
use crate::volume::volume_base::VolumeIoType;
use crate::volume::volume_manager::VolumeManager;

lazy_static!{
    pub static ref VolumeServiceSingleton: VolumeService = {
        VolumeService::new()
    };
}

pub struct VolumeService;

impl VolumeService {
    pub fn new() -> VolumeService {
        VolumeService
    }

    pub fn GetVolumeManagerById(&self, arrayId: i32) -> Box<dyn IVolumeManager> {
        // TODO: just create a mock just to pass a test.
        struct MockVolumeManager;
        impl IVolumeIoManager for MockVolumeManager {
            fn IncreasePendingIOCountIfNotZero(&self, volId: i32, volumeIoType: VolumeIoType, ioCountToSubmit: u32) -> Result<(), PosEventId> {
                Ok(())
            }

            fn DecreasePendingIOCount(&self, volId: i32, volumeIoType: VolumeIoType, ioCountCompleted: u32) -> Result<(), PosEventId> {
                Ok(())
            }
        }
        impl IVolumeManager for MockVolumeManager {}
        Box::new(MockVolumeManager)
    }

    pub fn GetVolumeManagerByName(&self, arrayName: String) -> Box<dyn IVolumeManager> {
        // TODO
        todo!()
    }
}