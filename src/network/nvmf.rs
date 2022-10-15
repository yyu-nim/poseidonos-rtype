use log::info;
use crate::array_models::interface::i_mount_sequence::IMountSequence;
use crate::sys_event::volume_event::{VolumeArrayInfo, VolumeEvent, VolumeEventBase, VolumeEventPerf};
use crate::sys_event::volume_event_publisher::VolumeEventPublisherSingleton;

pub struct Nvmf {
    arrayName: String,
    arrayId: u32,
}

impl Nvmf {

    pub fn new(arrayName: String, arrayId: u32) -> Nvmf {
        // TODO
        info!("Creating NVMf for {} with idx {}", arrayName, arrayId);

        Nvmf {
            arrayName,
            arrayId
        }
    }

}

impl VolumeEvent for Nvmf {
    fn Tag(&self) -> String {
        self.arrayId.to_string()
    }

    fn VolumeCreated(&self, volEventBase: VolumeEventBase, volEventPerf: VolumeEventPerf, volArrayInfo: VolumeArrayInfo) -> i32 {
        // TODO
        info!("VolumeCreated: {}", self.arrayName);
        0
    }

    fn VolumeUpdated(&self, volEventBase: VolumeEventBase, volEventPerf: VolumeEventPerf, volArrayInfo: VolumeArrayInfo) -> i32 {
        // TODO
        info!("VolumeUpdated: {}", self.arrayName);
        0
    }

    fn VolumeDeleted(&self, volEventBase: VolumeEventBase, volArrayInfo: VolumeArrayInfo) -> i32 {
        // TODO
        info!("VolumeDeleted: {}", self.arrayName);
        0
    }

    fn VolumeMounted(&self, volEventBase: VolumeEventBase, volEventPerf: VolumeEventPerf, volArrayInfo: VolumeArrayInfo) -> i32 {
        // TODO
        info!("VolumeMounted: {}", self.arrayName);
        0
    }

    fn VolumeUnmounted(&self, volEventBase: VolumeEventBase, volArrayInfo: VolumeArrayInfo) -> i32 {
        // TODO
        info!("VolumeUnmounted: {}", self.arrayName);
        0
    }

    fn VolumeLoaded(&self, volEventBase: VolumeEventBase, volEventPerf: VolumeEventPerf, volArrayInfo: VolumeArrayInfo) -> i32 {
        // TODO
        info!("VolumeLoaded: {}", self.arrayName);
        0
    }

    fn VolumeDetached(&self, volList: Vec<i32>, volArrayInfo: VolumeArrayInfo) -> i32 {
        // TODO
        info!("VolumeDetached: {}", self.arrayName);
        0
    }
}

impl IMountSequence for Nvmf {
    fn Init(&self) -> i32 {
        // TODO
        info!("Init: {}", self.arrayName);
        0
    }

    fn Dispose(&self) {
        // TODO
        info!("Dispose: {}", self.arrayName);
    }

    fn Shutdown(&self) {
        // TODO
        info!("Shutdown: {}", self.arrayName);
    }

    fn Flush(&self) {
        // TODO
        info!("Flush: {}", self.arrayName);
    }
}