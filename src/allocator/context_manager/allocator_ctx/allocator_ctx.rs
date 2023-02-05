use std::alloc::alloc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU16, AtomicU64};
use std::thread::current;
use crate::allocator::address::allocator_address_info::AllocatorAddressInfo;
use crate::allocator::include::allocator_const::{ACTIVE_STRIPE_TAIL_ARRAYLEN, AllocatorCtxHeader};
use crate::include::address_type::{StripeId, UNMAP_VSA, VirtualBlkAddr};
use crate::include::meta_const::STRIPES_PER_SEGMENT;
use crate::lib::bitmap::BitMapMutex;

pub struct AllocatorCtx {
    activeStripeTail: HashMap::<u32, VirtualBlkAddr>,
    allocWbLsidBitmap: BitMapMutex,
    currentSsdLsid: StripeId,
    pub(crate) allocCtxLock: Arc<Mutex<u32>>,
    activeStripeTailLock: HashMap::<u32, Arc<Mutex<u32>>>,
    ctxStoredVersion: AtomicU64,
    ctxDirtyVersion: AtomicU64,
    ctxHeader: AllocatorCtxHeader,
    initialized: bool,
    num_wb_stripes: u32, /* addrInfo->GetnumWbStripes() */
    stripes_per_segment: u32, /* addrInfo->GetstripesPerSegment() */
}

impl Default for AllocatorCtx {
    fn default() -> Self {
        AllocatorCtx {
            activeStripeTail: Default::default(),
            allocWbLsidBitmap: Default::default(),
            currentSsdLsid: 0,
            allocCtxLock: Arc::new(Mutex::new(0)),
            activeStripeTailLock: Default::default(),
            ctxStoredVersion: Default::default(),
            ctxDirtyVersion: Default::default(),
            ctxHeader: Default::default(),
            initialized: false,
            num_wb_stripes: 0,
            stripes_per_segment: 0,
        }
    }
}

impl AllocatorCtx {
    pub fn new(addr_info: &AllocatorAddressInfo) -> AllocatorCtx {
        let mut allocator_ctx = AllocatorCtx::default();
        allocator_ctx.num_wb_stripes = addr_info.num_wb_stripes;
        allocator_ctx.stripes_per_segment = addr_info.stripes_per_segment;
        allocator_ctx
    }

    pub fn Init(&mut self) {
        if self.initialized {
            return;
        }
        self.allocWbLsidBitmap = BitMapMutex::new(self.num_wb_stripes as u64);
        for vol_id in 0..ACTIVE_STRIPE_TAIL_ARRAYLEN {
            self.activeStripeTail.insert(vol_id as u32, UNMAP_VSA);
        }
        self.currentSsdLsid = STRIPES_PER_SEGMENT - 1;
        self.ctxHeader.header.ctxVersion = 0;
        self.ctxStoredVersion = 0.into();
        self.ctxDirtyVersion = 0.into();
        self.initialized = true;
    }

    pub fn GetActiveStripeTail(&self, as_tail_array_idx: u32) -> Option<&VirtualBlkAddr> {
        self.activeStripeTail.get(&as_tail_array_idx)
    }

    pub fn SetActiveStripeTail(&mut self, as_tail_array_idx: u32, vsa: VirtualBlkAddr) {
        self.activeStripeTail.insert(as_tail_array_idx, vsa);
    }

    pub fn AllocFreeWbStripe(&self) -> Option<StripeId> {
        let stripe: StripeId = self.allocWbLsidBitmap.SetNextZeroBit() as StripeId;
        if !self.allocWbLsidBitmap.IsValidBit(stripe) {
            return None
        } else {
            Some(stripe)
        }
    }

    pub fn GetCurrentSsdLsid(&self) -> StripeId {
        self.currentSsdLsid
    }

    pub fn SetCurrentSsdLsid(&mut self, stripe_id: Option<StripeId>) {
        self.currentSsdLsid = stripe_id.unwrap();
    }

    pub fn ReleaseWbStripe(&self, stripe_id: StripeId) {
        self.allocWbLsidBitmap.ClearBit(stripe_id as u64);
    }

    pub fn GetCtxLock(&self) -> Arc<Mutex<u32 /* don't care */>> {
        self.allocCtxLock.clone()
    }

    pub fn GetActiveStripeTailLock(&mut self, volume_id: u32) -> Arc<Mutex<u32>> {
        let tail_lock = self.activeStripeTailLock.get(&volume_id);
        if let Some(l) = tail_lock {
            return l.clone();
        } else {
            // the first time access. initialize the lock and return
            let tail_lock = Arc::new(Mutex::new(0));
            self.activeStripeTailLock.insert(volume_id, tail_lock.clone());
            return tail_lock.clone();
        }
    }

    // In fact, this is how to initialize "allocWbLsidBitmap" in pos-cpp.
    pub fn AfterLoad(&mut self, new_numBits: u64) {
        self.ctxStoredVersion = AtomicU64::from(self.ctxHeader.header.ctxVersion);
        self.ctxDirtyVersion = AtomicU64::from(self.ctxHeader.header.ctxVersion + 1);
        self.allocWbLsidBitmap.SetNumBitsSet(new_numBits);
    }
}