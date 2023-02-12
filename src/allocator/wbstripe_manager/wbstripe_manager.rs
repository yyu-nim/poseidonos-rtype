use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::allocator::address::allocator_address_info::AllocatorAddressInfo;
use crate::allocator::i_wbstripe_allocator::IWBStripeAllocator;
use crate::allocator::stripe::stripe::Stripe;
use crate::event_scheduler::event_scheduler::{EventScheduler, EventSchedulerSingleton};
use crate::include::address_type::{BlkAddr, StripeAddr, StripeId, VirtualBlkAddr};
use crate::include::meta_const::CHUNK_SIZE;
use crate::mapper::i_reversemap::IReverseMap;
use crate::mapper::i_stripemap::IStripeMap;
use crate::mapper_service::mapper_service::MapperServiceSingleton;
use crate::resource_manager::buffer_info::BufferInfo;
use crate::resource_manager::buffer_pool::BufferPool;
use crate::resource_manager::memory_manager::MemoryManager;

pub struct WBStripeManager {
    iStripeMap: Option<Arc<Mutex<Box<dyn IStripeMap>>>>,
    iReverseMap: Option<Arc<Mutex<Box<dyn IReverseMap>>>>,
    //volumeManager: Option<Box<IVolumeInfoManager>>,
    eventScheduler: Option<Arc<Mutex<EventScheduler>>>,
    arrayId: u32,
    addrInfo: AllocatorAddressInfo,
    stripeBufferPool: Option<BufferPool>,
    memoryManager: Option<MemoryManager>,
    wbStripeArray: Vec<Stripe>,
}

impl Default for WBStripeManager {
    fn default() -> Self {
        WBStripeManager {
            iStripeMap: None,
            iReverseMap: None,
            eventScheduler: None,
            arrayId: 0,
            addrInfo: Default::default(),
            stripeBufferPool: None,
            memoryManager: None,
            wbStripeArray: vec![]
        }
    }
}

impl IWBStripeAllocator for WBStripeManager {
    fn GetStripe(&self, wb_lsid: StripeId) -> Stripe {
        todo!()
    }

    fn FreeWBStripeId(&self, lsid: StripeId) {
        todo!()
    }

    fn ReferLsidCnt(&self, lsa: StripeAddr) -> bool {
        todo!()
    }

    fn DereferLsidCnt(&self, lsa: StripeAddr, block_count: u32) {
        todo!()
    }

    fn ReconstructActiveStripe(&self, volume_id: u32, wb_lsid: StripeId, tail_vsa: VirtualBlkAddr, revmap_infos: HashMap<u64, BlkAddr>) {
        todo!()
    }

    fn FinishStripe(&self, wb_lsid: StripeId, tail: VirtualBlkAddr) {
        todo!()
    }

    fn LoadPendingStripesToWriteBuffer(&self) -> i32 {
        todo!()
    }

    fn FlushAllPendingStripes(&self) -> i32 {
        todo!()
    }

    fn FlushAllPendingStripesInVolume(&self, volume_id: i32) -> i32 {
        todo!()
    }

    fn GetUserStripeId(&self, vsid: StripeId) -> StripeId {
        todo!()
    }
}

impl WBStripeManager {
    pub fn Init(&mut self) {
        let mapper_service = MapperServiceSingleton.lock().unwrap();
        if self.iStripeMap.is_none() {
            self.iStripeMap = mapper_service.GetIStripeMap(self.arrayId);
        }
        // TODO
        // if self.volumeManager.is_none() {
        //
        // }
        if self.iReverseMap.is_none() {
            self.iReverseMap = mapper_service.GetIReverseMap(self.arrayId);
        }
        if self.eventScheduler.is_none() {
            self.eventScheduler = Some(EventSchedulerSingleton.clone());
        }
        let total_nvm_stripes = self.addrInfo.num_wb_stripes;
        let chunks_per_stripe = self.addrInfo.chunks_per_stripe;
        let mut info = BufferInfo {
            owner: "WBStripeManager".to_string(),
            size: CHUNK_SIZE,
            count: (total_nvm_stripes * chunks_per_stripe) as u64,
        };
        self.stripeBufferPool = self.memoryManager.unwrap().CreateBufferPool(&mut info, None);

        for stripe_cnt in 0..total_nvm_stripes {
            let stripe = Stripe::from(None,
                                      self.iReverseMap.unwrap().clone(),
                                      true, self.addrInfo.blks_per_stripe);
            self.wbStripeArray.push( stripe );
        }
    }
}