use std::collections::HashMap;
use crate::include::address_type::{StripeId, VirtualBlkAddr};

pub struct AllocatorCtx {
    pub activeStripeTail: HashMap::<u32, VirtualBlkAddr>,
}

impl AllocatorCtx {
    pub fn GetActiveStripeTail(&self, as_tail_array_idx: u32) -> Option<&VirtualBlkAddr> {
        self.activeStripeTail.get(&as_tail_array_idx)
    }

    pub fn SetActiveStripeTail(&mut self, as_tail_array_idx: u32, vsa: VirtualBlkAddr) {
        self.activeStripeTail.insert(as_tail_array_idx, vsa);
    }

    pub fn AllocFreeWbStripe(&self) -> Option<StripeId> {
        todo!()
    }

    pub fn GetCurrentSsdLsid(&self) -> StripeId {
        todo!()
    }

    pub fn SetCurrentSsdLsid(&self, stripe_id: Option<StripeId>) {
        todo!()
    }

    pub fn ReleaseWbStripe(&self, stripe_id: StripeId) {
        todo!()
    }
}