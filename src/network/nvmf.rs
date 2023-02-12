use log::info;
use crate::array_models::interface::i_array_info::ArrayInfo;
use crate::array_models::interface::i_mount_sequence::IMountSequence;
use crate::sys_event::volume_event::{VolumeArrayInfo, VolumeEvent, VolumeEventBase, VolumeEventPerf};
use crate::sys_event::volume_event_publisher::VolumeEventPublisherSingleton;

pub struct Nvmf {
    arrayInfo: ArrayInfo,
}

impl Nvmf {

    pub fn new(arrayInfo: ArrayInfo) -> Nvmf {
        // TODO
        info!("Creating NVMf for {} with idx {}", arrayInfo.name, arrayInfo.index);
        let nvmf = Nvmf {
            arrayInfo,
        };
        //let boxed : Box<dyn VolumeEvent> = Box::new(nvmf);
        //VolumeEventPublisherSingleton.RegisterSubscriber(&boxed, arrayName.clone(), arrayId as i32);
        nvmf
    }

}

impl VolumeEvent for Nvmf {
    fn Tag(&self) -> String {
        self.arrayInfo.index.to_string()
    }

    fn VolumeCreated(&self, volEventBase: VolumeEventBase, volEventPerf: VolumeEventPerf, volArrayInfo: VolumeArrayInfo) -> i32 {
        // TODO
        info!("VolumeCreated: {}", self.arrayInfo.name);
        0
    }

    fn VolumeUpdated(&self, volEventBase: VolumeEventBase, volEventPerf: VolumeEventPerf, volArrayInfo: VolumeArrayInfo) -> i32 {
        // TODO
        info!("VolumeUpdated: {}", self.arrayInfo.name);
        0
    }

    fn VolumeDeleted(&self, volEventBase: VolumeEventBase, volArrayInfo: VolumeArrayInfo) -> i32 {
        // TODO
        info!("VolumeDeleted: {}", self.arrayInfo.name);
        0
    }

    fn VolumeMounted(&self, volEventBase: VolumeEventBase, volEventPerf: VolumeEventPerf, volArrayInfo: VolumeArrayInfo) -> i32 {
        // TODO
        info!("VolumeMounted: {}", self.arrayInfo.name);
        0
    }

    fn VolumeUnmounted(&self, volEventBase: VolumeEventBase, volArrayInfo: VolumeArrayInfo) -> i32 {
        // TODO
        info!("VolumeUnmounted: {}", self.arrayInfo.name);
        0
    }

    fn VolumeLoaded(&self, volEventBase: VolumeEventBase, volEventPerf: VolumeEventPerf, volArrayInfo: VolumeArrayInfo) -> i32 {
        // TODO
        info!("VolumeLoaded: {}", self.arrayInfo.name);
        0
    }

    fn VolumeDetached(&self, volList: Vec<i32>, volArrayInfo: VolumeArrayInfo) -> i32 {
        // TODO
        info!("VolumeDetached: {}", self.arrayInfo.name);
        0
    }
}

impl IMountSequence for Nvmf {
    fn Init(&mut self) -> i32 {
        // TODO
        info!("Init: {}", self.arrayInfo.name);
        0
    }

    fn Dispose(&self) {
        // TODO
        info!("Dispose: {}", self.arrayInfo.name);
    }

    fn Shutdown(&self) {
        // TODO
        info!("Shutdown: {}", self.arrayInfo.name);
    }

    fn Flush(&self) {
        // TODO
        info!("Flush: {}", self.arrayInfo.name);
    }
}