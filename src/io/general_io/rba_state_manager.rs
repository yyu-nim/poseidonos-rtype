use lazy_static::lazy_static;
use crate::include::pos_event_id::PosEventId;

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

    pub fn BulkAcquireOwnership(&self, volume_id: u32, start_rba: u64, i: u32) -> Result<bool, PosEventId> {
        // TODO
        Ok(true)
    }

    pub fn BulkReleaseOwnership(&self, volume_id: u32, start_rba: u64, i: u32) -> Result<(), PosEventId> {
        // TODO
        Ok(())
    }
}