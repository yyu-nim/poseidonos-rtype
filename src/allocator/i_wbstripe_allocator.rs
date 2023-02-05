use std::collections::HashMap;
use crate::allocator::stripe::stripe::Stripe;
use crate::include::address_type::{BlkAddr, StripeAddr, StripeId, VirtualBlkAddr};

use mockall::*;
use mockall::predicate::*;

#[automock]
pub trait IWBStripeAllocator : Send + Sync {
    fn GetStripe(&self, wb_lsid: StripeId) -> Stripe;
    fn FreeWBStripeId(&self, lsid: StripeId);
    fn ReferLsidCnt(&self, lsa: StripeAddr) -> bool;
    fn DereferLsidCnt(&self, lsa: StripeAddr, block_count: u32);
    fn ReconstructActiveStripe(&self, volume_id: u32, wb_lsid: StripeId, tail_vsa: VirtualBlkAddr,
                               revmap_infos: HashMap<u64, BlkAddr>);
    fn FinishStripe(&self, wb_lsid: StripeId, tail: VirtualBlkAddr);
    fn LoadPendingStripesToWriteBuffer(&self) -> i32;
    fn FlushAllPendingStripes(&self) -> i32;
    fn FlushAllPendingStripesInVolume(&self, volume_id: i32) -> i32;
    //fn FlushAllPendingStripesInVolume(&self, volume_id: i32, flush_io: FlushIo) -> i32;
    fn GetUserStripeId(&self, vsid: StripeId) -> StripeId;
}
