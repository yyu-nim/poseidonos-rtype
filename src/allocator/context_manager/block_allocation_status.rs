use crate::volume::volume_base::MAX_VOLUME_COUNT;

pub struct BlockAllocationStatus {
    pub blkAllocProhibited: [bool; MAX_VOLUME_COUNT],
    pub userBlkAllocProhibited: bool,
}

impl BlockAllocationStatus {
    pub fn IsUserBlockAllocationProhibited(&self, volume_id: u32) -> bool {
        if self.blkAllocProhibited[volume_id as usize] {
            true
        } else {
            self.userBlkAllocProhibited
        }
    }
}

impl Default for BlockAllocationStatus {
    fn default() -> Self {
        BlockAllocationStatus {
            blkAllocProhibited: [false; MAX_VOLUME_COUNT],
            userBlkAllocProhibited: false
        }
    }
}