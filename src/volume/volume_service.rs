use lazy_static::lazy_static;
use crate::volume::i_volume_manager::IVolumeManager;

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
        // TODO
        todo!()
    }

    pub fn GetVolumeManagerByName(&self, arrayName: String) -> Box<dyn IVolumeManager> {
        // TODO
        todo!()
    }
}