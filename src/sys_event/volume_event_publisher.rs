use lazy_static::lazy_static;
use log::info;
use crate::sys_event::volume_event::VolumeEvent;

lazy_static!{
    pub static ref VolumeEventPublisherSingleton: VolumeEventPublisher = {
        VolumeEventPublisher::new()
    };
}

pub struct VolumeEventPublisher;

impl VolumeEventPublisher {

    pub fn new() -> VolumeEventPublisher {
        VolumeEventPublisher
    }

    pub fn RegisterSubscriber(&self, subscriber: &Box<dyn VolumeEvent>, arrayName: String, arrayId: i32) {
        // TODO
        info!("TODO: Registering VolumeEvent subscriber for {}", arrayName);
    }

    pub fn RemoveSubscriber(&self, subscriber: &Box<dyn VolumeEvent>, arrayName: String, arrayId: i32) {
        // TODO
    }

    /***
    VolumeEventPublisher(void);
    virtual ~VolumeEventPublisher(void);
    void RegisterSubscriber(VolumeEvent* subscriber, std::string arrayName, int arrayId);
    void RemoveSubscriber(VolumeEvent* subscriber, std::string arrayName, int arrayId);
    virtual bool NotifyVolumeCreated(VolumeEventBase* volEventBase, VolumeEventPerf* volEventPerf, VolumeArrayInfo* volArrayInfo);
    virtual bool NotifyVolumeUpdated(VolumeEventBase* volEventBase, VolumeEventPerf* volEventPerf, VolumeArrayInfo* volArrayInfo);
    virtual bool NotifyVolumeDeleted(VolumeEventBase* volEventBase, VolumeArrayInfo* volArrayInfo);
    virtual bool NotifyVolumeMounted(VolumeEventBase* volEventBase, VolumeEventPerf* volEventPerf, VolumeArrayInfo* volArrayInfo);
    virtual bool NotifyVolumeUnmounted(VolumeEventBase* volEventBase, VolumeArrayInfo* volArrayInfo);
    virtual bool NotifyVolumeLoaded(VolumeEventBase* volEventBase, VolumeEventPerf* volEventPerf, VolumeArrayInfo* volArrayInfo);
    virtual void NotifyVolumeDetached(vector<int> volList, VolumeArrayInfo* volArrayInfo);
    */

}