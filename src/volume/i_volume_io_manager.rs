use crate::include::pos_event_id::PosEventId;
use crate::volume::volume_base::VolumeIoType;

pub trait IVolumeIoManager {
    fn IncreasePendingIOCountIfNotZero(&self, volId: i32, volumeIoType: VolumeIoType, ioCountToSubmit: u32) -> Result<(), PosEventId>;
    fn DecreasePendingIOCount(&self, volId: i32, volumeIoType: VolumeIoType, ioCountCompleted: u32) -> Result<(), PosEventId>;
}