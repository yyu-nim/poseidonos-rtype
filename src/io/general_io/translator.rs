use std::sync::{Arc, Mutex};
use crate::array_models::interface::i_array_info::ArrayInfo;
use crate::include::address_type::{BlkAddr, LogicalBlkAddr, PhysicalBlkAddr, PhysicalEntry, StripeAddr, StripeId, StripeLoc, VirtualBlkAddr};

pub struct Translator {
    pub startRba: BlkAddr,
    pub blockCount: u32,
}

impl Translator {
    pub fn new(volume_id: u32, start_rba: BlkAddr, array_id: i32, is_read: bool) -> Translator {
        // TODO
        Translator {
            startRba: start_rba,
            blockCount: 1,
        }
    }

    pub fn new_with_vsa(vsa: VirtualBlkAddr, array_id: i32, user_lsid: StripeId, array_info: Option<ArrayInfo>) -> Translator {
        // TODO
        Translator {
            startRba: 0,
            blockCount: 1,
        }
    }

    pub fn GetLsidEntry(&self, block_index: u32) -> StripeAddr {
        // TODO
        StripeAddr {
            stripe_loc: StripeLoc::IN_USER_AREA,
            stripe_id: 0
        }
    }

    pub fn IsMapped(&self) -> bool {
        // TODO
        false
    }

    pub fn GetPba(&self) -> PhysicalBlkAddr {
        // TODO
        PhysicalBlkAddr {
            lba: 0,
            array_dev: None
        }
    }

    pub fn GetPhysicalEntries(&self, mem: Arc<Mutex<Vec<u8>>>, block_count: u32) -> Vec<PhysicalEntry> {
        // TODO
        self._CheckSingleBlock();

        Vec::new()
    }

    fn _CheckSingleBlock(&self) {
        assert_eq!(self.blockCount, 1);
    }
}