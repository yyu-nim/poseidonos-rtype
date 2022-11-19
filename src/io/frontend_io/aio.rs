use crate::bio::volume_io::VolumeIo;
use crate::generated::bindings::pos_io;
use crate::volume::volume_base::VolumeIoType;

pub struct AIO;

impl AIO {
    pub fn SubmitAsyncIO(&self, volIo: VolumeIo) {
        // TODO
    }

    pub fn SubmitAsyncAdmin(&self, io: pos_io/*, arrayInfo: */) {
        // TODO
    }

    pub fn SubmitFlush(&self, posIo: pos_io) {
        // TODO
    }

    pub fn CreateVolumeIo(&self, posIo: pos_io) -> VolumeIo {
        // TODO
        VolumeIo
    }
}