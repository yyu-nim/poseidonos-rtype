use lazy_static::lazy_static;

pub struct RBAStateManager {

}

lazy_static!{
    pub static ref RBAStateManagerSingleton: RBAStateManager = {
        RBAStateManager::new()
    };
}

impl RBAStateManager {
    pub fn new() -> RBAStateManager {
        RBAStateManager {

        }
    }

    pub fn AcquireOwnershipRbaList(&self) {

    }

    pub fn ReleaseOwnershipRbaList(&self) {

    }

    pub fn BulkAcquireOwnership(&self, volume_id: u32, start_rba: u64, i: u32) -> bool {
        // TODO
        true
    }

    pub fn BulkReleaseOwnership(&self, volume_id: u32, start_rba: u64, i: u32) {

    }
}