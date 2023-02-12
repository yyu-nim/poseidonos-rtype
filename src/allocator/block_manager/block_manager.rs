use std::sync::{Arc, Mutex};
use log::{error, warn};
use crate::allocator::address::allocator_address_info::AllocatorAddressInfo;
use crate::allocator::context_manager::allocator_ctx::allocator_ctx::AllocatorCtx;
use crate::allocator::context_manager::block_allocation_status::BlockAllocationStatus;
use crate::allocator::context_manager::context_manager::ContextManager;
use crate::allocator::i_block_allocator::IBlockAllocator;
use crate::allocator::i_wbstripe_allocator;
use crate::allocator::i_wbstripe_allocator::IWBStripeAllocator;
use crate::allocator::stripe::stripe::Stripe;
use crate::include::address_type::{BlkOffset, IsUnMapStripe, StripeId, StripeLoc, UNMAP_OFFSET, UNMAP_STRIPE, VirtualBlkAddr, VirtualBlks};
use crate::include::pos_event_id::PosEventId::ALLOCATOR_FAILED_TO_ASSIGN_STRIPE;
use crate::mapper::i_reversemap::IReverseMap;
use crate::mapper::i_stripemap::IStripeMap;
use crate::mapper::i_stripemap;
use crate::mapper_service::mapper_service::MapperServiceSingleton;
use crate::qos::qos_manager::QosManagerSingleton;

pub struct BlockManager {
    allocStatus: BlockAllocationStatus,
    allocCtx: AllocatorCtx,
    addrInfo: AllocatorAddressInfo,
    arrayId: u32,
    iStripeMap: Option<Arc<Mutex<Box<dyn IStripeMap>>>>,
    iReverseMap: Option<Arc<Mutex<Box<dyn IReverseMap>>>>,
    contextManager: ContextManager,
    iWBStripeAllocator: Arc<Mutex<Box<dyn IWBStripeAllocator>>>,
}

impl Default for BlockManager {
    fn default() -> Self {
        BlockManager {
            allocStatus: Default::default(),
            allocCtx: Default::default(),
            addrInfo: Default::default(),
            arrayId: 0,
            iStripeMap: None,
            iReverseMap: None,
            contextManager: Default::default(),
            iWBStripeAllocator: Arc::new(Mutex::new(i_wbstripe_allocator::boxed_default())),
        }
    }
}

impl IBlockAllocator for BlockManager {
    fn AllocateWriteBufferBlks(&mut self, volume_id: u32, num_blks: u32) -> Option<(VirtualBlks, StripeId)> {
        if self.allocStatus.IsUserBlockAllocationProhibited(volume_id) {
            return None;
        }

        return self._AllocateBlks(volume_id, num_blks);
    }

    fn AllocateGcDestStripe(&self, volume_id: u32) -> Stripe {
        todo!()
    }

    fn ProhibitUserBlkAlloc(&self) {
        todo!()
    }

    fn PermitUserBlkAlloc(&self) {
        todo!()
    }

    fn BlockAllocating(&self, volume_id: u32) -> bool {
        todo!()
    }

    fn UnblockAllocating(&self, volume_id: u32) {
        todo!()
    }

    fn TryRdLock(&self, volume_id: u32) -> bool {
        todo!()
    }

    fn Unlock(&self, volume_id: u32) -> bool {
        todo!()
    }

    fn boxed_clone(&self) -> Box<dyn IBlockAllocator> {
        todo!()
    }
}

impl BlockManager {

    pub fn Init(&mut self, wb_stripe_manager: Arc<Mutex<Box<dyn IWBStripeAllocator>>>) {
        self.iWBStripeAllocator = wb_stripe_manager;
        let m = MapperServiceSingleton.lock().unwrap();
        if self.iReverseMap.is_none() {
            self.iReverseMap = m.GetIReverseMap(self.arrayId);
        }
        if self.iStripeMap.is_none() {
            self.iStripeMap = m.GetIStripeMap(self.arrayId);
        }
    }

    fn _AllocateBlks(&mut self, as_tail_array_idx: u32, num_blks: u32) -> Option<(VirtualBlks, StripeId)> {
        assert_ne!(num_blks, 0);
        let tail_lock = self.allocCtx.GetActiveStripeTailLock(as_tail_array_idx);
        tail_lock.lock().unwrap();

        let cur_vsa = self.allocCtx.GetActiveStripeTail(as_tail_array_idx);
        let allocated_user_stripe_id;
        if cur_vsa.is_none() || self._IsStripeFull(cur_vsa.unwrap()) { // || IsUnMapStripe(cur_vsa.stripe_id) {
            if let Some((_wb_lsid, user_lsid)) = self._AllocateStripesAndUpdateActiveStripeTail(as_tail_array_idx) {
                allocated_user_stripe_id = user_lsid;
            }  else {
                return None;
            }
        } else {
            allocated_user_stripe_id = self.iWBStripeAllocator.GetUserStripeId(cur_vsa.unwrap().stripe_id);
        }

        let allocated_blks = self._AllocateBlocksFromActiveStripe(as_tail_array_idx, num_blks);
        Some((allocated_blks, allocated_user_stripe_id))
    }

    fn _IsStripeFull(&self, addr: &VirtualBlkAddr) -> bool {
        addr.offset == self.addrInfo.blks_per_stripe as BlkOffset
    }

    fn _IsSegmentFull(&self, stripe_id: StripeId) -> bool {
        stripe_id % self.addrInfo.stripes_per_segment == 0
    }

    fn _IsValidOffset(&self, stripe_offset: u64) -> bool {
        stripe_offset < self.addrInfo.blks_per_stripe as u64
    }

    fn _AllocateStripesAndUpdateActiveStripeTail(&mut self, as_tail_array_idx: u32)
                                                 -> Option<(StripeId, StripeId)> {

        if let Some(wb_lsid) = self.allocCtx.AllocFreeWbStripe() {
            if let Some(user_lsid) = self._AllocateSsdStripeForUser(as_tail_array_idx) {
                QosManagerSingleton.IncreaseUsedStripeCnt(self.arrayId);
                self._AssignStripe(user_lsid, wb_lsid, as_tail_array_idx);
                self.iStripeMap.unwrap().SetLSA(user_lsid, wb_lsid, StripeLoc::IN_WRITE_BUFFER_AREA);
                let cur_vsa = VirtualBlkAddr {
                    stripe_id: user_lsid,
                    offset: 0
                };
                self.allocCtx.SetActiveStripeTail(as_tail_array_idx, cur_vsa);
                return Some((wb_lsid, user_lsid));
            } else {
                self.allocCtx.ReleaseWbStripe(wb_lsid);
                return None;
            }
        } else {
            return None;
        }
    }

    fn _AllocateSsdStripeForUser(&mut self, volume_id: u32) -> Option<StripeId> {
        let alloc_ctx_lock = self.allocCtx.GetCtxLock().clone();
        alloc_ctx_lock.lock().unwrap();

        let ssd_lsid = self.allocCtx.GetCurrentSsdLsid() + 1;
        let mut opt_ssd_lsid = Some(ssd_lsid);
        if self._IsSegmentFull(ssd_lsid) {
            if self.allocStatus.IsUserBlockAllocationProhibited(volume_id) {
                opt_ssd_lsid = None;
            } else {
                opt_ssd_lsid = self._AllocateSegmentAndStripe();
            }
        }
        self.allocCtx.SetCurrentSsdLsid(opt_ssd_lsid);
        opt_ssd_lsid
    }

    fn _AllocateSegmentAndStripe(&mut self) -> Option<StripeId> {
        match self.contextManager.AllocateFreeSegment() {
            Some(segment_id) => {
                let new_stripe = segment_id * self.addrInfo.stripes_per_segment;
                Some(new_stripe)
            }
            None => None
        }
    }

    fn _AssignStripe(&self, vsid: StripeId, wb_lsid: StripeId, as_tail_array_idx: u32) {
        let stripe = self.iWBStripeAllocator.GetStripe(wb_lsid);
        let user_lsid = self.iWBStripeAllocator.GetUserStripeId(vsid);
        let stripe_assigned = stripe.Assign(vsid, wb_lsid, as_tail_array_idx);
        if !stripe_assigned {
            error!("[{}] Failed to assign a stripe for vsid {}, wbLsid {}, tailArrayIdx {}",
                     ALLOCATOR_FAILED_TO_ASSIGN_STRIPE.to_string(), vsid, wb_lsid, as_tail_array_idx);
        }
    }

    fn _AllocateBlocksFromActiveStripe(&mut self, as_tail_array_idx: u32, num_blks: u32) -> VirtualBlks {
        let cur_vsa = self.allocCtx.GetActiveStripeTail(as_tail_array_idx)
            .expect(format!("Cannot enter here without having an active stripe tail: volume_id: {}", as_tail_array_idx).as_str());

        let mut updated_tail = cur_vsa.clone();
        let mut allocated_blks = VirtualBlks {
            start_vsa: cur_vsa.clone(),
            num_blks: 0 /* => uninitialized */
        };

        if !self._IsValidOffset(cur_vsa.offset + num_blks as u64 - 1) {
            allocated_blks.num_blks = self.addrInfo.blks_per_stripe - cur_vsa.offset as u32;
            updated_tail.offset = self.addrInfo.blks_per_stripe as BlkOffset;
        } else {
            allocated_blks.num_blks = num_blks as u32;
            updated_tail.offset = cur_vsa.offset + num_blks as u64;
        }
        self.allocCtx.SetActiveStripeTail(as_tail_array_idx, updated_tail);
        allocated_blks
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io::empty;
    use std::sync::{Arc, Mutex};
    use crate::allocator::address::allocator_address_info::AllocatorAddressInfo;
    use crate::allocator::block_manager::block_manager::BlockManager;
    use crate::allocator::context_manager::allocator_ctx::allocator_ctx::AllocatorCtx;
    use crate::allocator::context_manager::block_allocation_status::BlockAllocationStatus;
    use crate::allocator::context_manager::context_manager::ContextManager;
    use crate::mapper::i_stripemap::*;
    use crate::allocator::i_wbstripe_allocator::*;
    use crate::allocator::context_manager::block_allocation_status;
    use crate::allocator::i_block_allocator::IBlockAllocator;
    use crate::allocator::stripe::stripe::Stripe;
    use crate::include::address_type::{StripeId, VirtualBlkAddr};
    use crate::volume::volume_base::MAX_VOLUME_COUNT;

    #[test]
    fn test_none_is_returned_when_block_allocation_is_prohibited() {
        // Given: volumeId=10 's block allocation is prohibited
        let mut mock_stripe_map = MockIStripeMap::new();
        let mut mock_wb_stripe_allocator = MockIWBStripeAllocator::new();
        let mut block_allocation_status = BlockAllocationStatus {
            blkAllocProhibited: [false; MAX_VOLUME_COUNT],
            userBlkAllocProhibited: false
        };
        let volume_id = 10u32;
        block_allocation_status.blkAllocProhibited[volume_id as usize] = true;

        let block_manager = BlockManager {
            allocStatus: block_allocation_status,
            allocCtx: Default::default(),
            addrInfo: AllocatorAddressInfo::default(),
            arrayId: 0,
            iStripeMap: Some(Box::new(mock_stripe_map)),
            iReverseMap: None,
            contextManager: Default::default(),
            iWBStripeAllocator: Arc::new(Mutex::new(Box::new(mock_wb_stripe_allocator))),
        };

        let mut iBlockAllocator = Box::new(block_manager);

        // When
        let actual = iBlockAllocator.AllocateWriteBufferBlks(volume_id, 1);

        // Then
        assert!(actual.is_none());
    }

    #[test]
    fn test_none_is_returned_when_userblock_allocation_is_prohibited() {
        // Given: userBlkAllocProhibited is true
        let mut mock_stripe_map = MockIStripeMap::new();
        let mut mock_wb_stripe_allocator = MockIWBStripeAllocator::new();
        let mut block_allocation_status = BlockAllocationStatus {
            blkAllocProhibited: [false; MAX_VOLUME_COUNT],
            userBlkAllocProhibited: true
        };
        let block_manager = BlockManager {
            allocStatus: block_allocation_status,
            allocCtx: Default::default(),
            addrInfo: AllocatorAddressInfo::default(),
            arrayId: 0,
            iStripeMap: Box::new(mock_stripe_map),
            contextManager: Default::default(),
            iWBStripeAllocator: Box::new(mock_wb_stripe_allocator),
        };
        let mut iBlockAllocator = Box::new(block_manager);

        // When
        let actual = iBlockAllocator.AllocateWriteBufferBlks(0/* don't care */, 1 /* don't care */);

        // Then
        assert!( actual.is_none() );
    }

    #[test]
    fn test_allocator_allocates_blocks_from_existing_userstripe_containing_enough_remaining_blocks() {
        // Given: an empty stripe 123 (tail offset = 0)
        let mut mock_stripe_map = MockIStripeMap::new();
        let mut mock_wb_stripe_allocator = MockIWBStripeAllocator::new();
        let expected_stripe_id: StripeId = 123;
        mock_wb_stripe_allocator.expect_GetUserStripeId()
            .times(1)
            .return_const(expected_stripe_id);

        let mut allocCtx = AllocatorCtx::default();
        allocCtx.SetActiveStripeTail(0, VirtualBlkAddr {
            stripe_id: 123,
            offset: 0 /* stripe이 텅 비어 있으므로, "enough remaining blocks" 가 된다. */
        });
        let block_manager = BlockManager {
            allocStatus: BlockAllocationStatus::default(),
            allocCtx,
            addrInfo: {
                let mut addr_info = AllocatorAddressInfo::default();
                addr_info.blks_per_stripe = 128;
                addr_info
            },
            arrayId: 0,
            iStripeMap: Box::new(mock_stripe_map),
            contextManager: ContextManager::default(),
            iWBStripeAllocator: Box::new(mock_wb_stripe_allocator),
        };
        let mut iBlockAllocator = Box::new(block_manager);

        // When: allocate 10 blocks
        let actual = iBlockAllocator.AllocateWriteBufferBlks(0, 10);

        // Then: 10 blocks should be allocated from stripe 123
        let (allocated_blks, allocated_user_stripe_id) = actual.unwrap();
        assert_eq!(expected_stripe_id, allocated_blks.start_vsa.stripe_id);
        assert_eq!(expected_stripe_id, allocated_user_stripe_id);
        assert_eq!(10, allocated_blks.num_blks);
    }

    #[test]
    fn test_allocator_allocates_blocks_from_existing_userstripe_containing_fewer_remaining_blocks_than_needed() {
        // Given: a half-full stripe 123 (tail offset = 64)
        let mut mock_stripe_map = MockIStripeMap::new();
        let mut mock_wb_stripe_allocator = MockIWBStripeAllocator::new();
        let expected_stripe_id: StripeId = 123;
        mock_wb_stripe_allocator.expect_GetUserStripeId()
            .times(1)
            .return_const(expected_stripe_id);

        let mut allocCtx = AllocatorCtx::default();
        allocCtx.SetActiveStripeTail(0, VirtualBlkAddr {
            stripe_id: 123,
            offset: 64 /* stripe을 half-full로 만든다 */
        });
        let block_manager = BlockManager {
            allocStatus: BlockAllocationStatus::default(),
            allocCtx,
            addrInfo: {
                let mut addr_info = AllocatorAddressInfo::default();
                addr_info.blks_per_stripe = 128;
                addr_info
            },
            arrayId: 0,
            iStripeMap: Box::new(mock_stripe_map),
            contextManager: ContextManager::default(),
            iWBStripeAllocator: Box::new(mock_wb_stripe_allocator),
        };
        let mut iBlockAllocator = Box::new(block_manager);

        // When: allocate 72 blocks to make the stripe overflow
        let actual = iBlockAllocator.AllocateWriteBufferBlks(0, 72);

        // Then: the requested blocks are partially allocated
        let (allocated_blks, allocated_user_stripe_id) = actual.unwrap();
        assert_eq!(128 - 64, allocated_blks.num_blks);
        assert_eq!(123, allocated_blks.start_vsa.stripe_id);
        assert_eq!(123, allocated_user_stripe_id);
    }

    #[test]
    fn test_allocator_allocates_blocks_from_new_userstripe_from_new_segment() {
        // Given: a full stripe
        let expected_stripe: Stripe = Stripe {};
        let mut mock_wb_stripe_allocator = MockIWBStripeAllocator::new();
        let expected_allocated_stripe_id: StripeId = 123;
        mock_wb_stripe_allocator.expect_GetStripe()
            .return_const(expected_stripe);
        mock_wb_stripe_allocator.expect_GetUserStripeId()
            .times(1)
            .return_const(expected_allocated_stripe_id);
        let mut mock_stripe_map = MockIStripeMap::new();
        mock_stripe_map.expect_SetLSA()
            .times(1)
            .return_const(0 /* success */);

        let addr_info = {
            let mut addr_info = AllocatorAddressInfo::default();
            addr_info.blks_per_stripe = 128;
            addr_info.num_wb_stripes = 500;
            addr_info.stripes_per_segment = 1024;
            addr_info
        };
        let mut allocCtx = AllocatorCtx::new(&addr_info);
        allocCtx.Init();

        let current_lsid = expected_allocated_stripe_id - 1;
        allocCtx.SetActiveStripeTail(0, VirtualBlkAddr {
            stripe_id: current_lsid,
            offset: 128 /* stripe을 full로 만든다 */
        });
        allocCtx.SetCurrentSsdLsid(Some(current_lsid));

        let block_manager = BlockManager {
            allocStatus: BlockAllocationStatus::default(),
            allocCtx,
            addrInfo: addr_info,
            arrayId: 0,
            iStripeMap: Box::new(mock_stripe_map),
            contextManager: ContextManager::default(),
            iWBStripeAllocator: Box::new(mock_wb_stripe_allocator),
        };
        let mut iBlockAllocator = Box::new(block_manager);

        // When: allocate 1 block
        let actual = iBlockAllocator.AllocateWriteBufferBlks(0, 1);

        // Then: the allocated block should be in the "expected_stripe_id" stripe
        let (virtual_blks, stripe_id) = actual.unwrap();
        assert_eq!(expected_allocated_stripe_id, stripe_id);
        assert_eq!(expected_allocated_stripe_id, virtual_blks.start_vsa.stripe_id);
        assert_eq!(1, virtual_blks.num_blks);
    }

}