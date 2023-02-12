use std::borrow::Borrow;
use std::sync::{Arc, Mutex};
use log::{info, warn};
use crate::include::pos_event_id::PosEventId;
use crate::allocator::address::allocator_address_info::AllocatorAddressInfo;
use crate::allocator::block_manager::block_manager::BlockManager;
use crate::allocator::context_manager::context_manager::ContextManager;
use crate::allocator::i_wbstripe_allocator::IWBStripeAllocator;
use crate::allocator::wbstripe_manager::wbstripe_manager::WBStripeManager;
use crate::allocator_service::allocator_service::{AllocatorService, AllocatorServiceSingleton};
use crate::array_models::interface::i_array_info::ArrayInfo;
use crate::array_models::interface::i_mount_sequence::IMountSequence;

pub struct Allocator {
    isInitialized: bool,
    addrInfo: AllocatorAddressInfo,
    info_: ArrayInfo,
    contextManager: ContextManager,
    blockManager: BlockManager,
    wbStripeManager: Arc<Mutex<Box<WBStripeManager>>>,
}

impl Allocator {
    pub fn new(arrayInfo: ArrayInfo) -> Allocator {
        info!("Creating Allocator for {}", arrayInfo.name);
        Allocator {
            isInitialized: false,
            addrInfo: Default::default(),
            info_: Default::default(),
            contextManager: Default::default(),
            blockManager: Default::default(),
            wbStripeManager: Arc::new(Mutex::new(Box::new(Default::default()))),
        }
    }

    fn _RegisterToAllocatorService(&self) {
        //let allocator_service = AllocatorServiceSingleton
        AllocatorServiceSingleton
    }
}

impl IMountSequence for Allocator {
    fn Init(&mut self) -> i32 {
        info!("TODO: Init() for Allocator...");
        if !self.isInitialized {
            self.addrInfo.Init(self.info_.borrow());
            self.contextManager.Init();
            let i_wb_stripe_allocator = self.wbStripeManager.clone() as Arc<Mutex<Box<dyn IWBStripeAllocator>>>;
            self.blockManager.Init(i_wb_stripe_allocator);
            self.wbStripeManager.lock().unwrap().Init();

            self._RegisterToAllocatorService();
            self.isInitialized = true;

            info!("[{}] Allocator of array {} is initialized", PosEventId::ALLOCATOR_INITIALIZE.to_string(),
                self.info_.name);
        } else {
            warn!("[{}] Allocator of array {} is already initialized, so skip Init().
            Init() is designed to be idempotent, but needs developer's further attention when called multiple times",
            PosEventId::ALLOCATOR_INITIALIZE.to_string(), self.info_.name);
        }

        0
    }

    fn Dispose(&self) {
        info!("TODO: Dispose() for Allocator...");
    }

    fn Shutdown(&self) {
        info!("TODO: Shutdown() for Allocator...");
    }

    fn Flush(&self) {
        info!("TODO: Flush() for Allocator...");
    }
}