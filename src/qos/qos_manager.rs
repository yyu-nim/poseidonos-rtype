use lazy_static::lazy_static;
use crate::bio::volume_io::VolumeIo;
lazy_static!{
    pub static ref QosManagerSingleton : QosManager = {
        QosManager
    };
}

pub struct QosManager;
impl QosManager {

    pub fn Initialize(&self) {
        // TODO
    }

    pub fn InitializeSpdkManager(&self) {
        // TODO
    }

    pub fn IsFeQosEnabled(&self) -> bool {
        // TODO
        false
    }

    pub fn HandlePosIoSubmission(&self, /*aioSubmission*/ volumeIo: VolumeIo) {
        // TODO

    }

    pub fn IncreaseUsedStripeCnt(&self, array_id: u32) {
        // TODO
    }
}