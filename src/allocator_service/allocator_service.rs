use std::collections::HashMap;
use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::allocator::allocator::Allocator;
use crate::allocator::i_block_allocator::IBlockAllocator;
use crate::allocator::stripe::stripe::Stripe;
use crate::include::address_type::{StripeId, VirtualBlks};

pub struct AllocatorService {
    pub arrayId_to_allocator: DashMap<u32, Box<dyn IBlockAllocator>>,
}

lazy_static!{
    pub static ref AllocatorServiceSingleton: AllocatorService = {
        AllocatorService::new()
    };
}

impl Clone for AllocatorService {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl IBlockAllocator for AllocatorService {
    fn AllocateWriteBufferBlks(&self, volume_id: u32, num_blks: u32) -> (VirtualBlks, StripeId) {
        todo!()
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

impl AllocatorService {
    pub fn new() -> AllocatorService {
        AllocatorService {
            arrayId_to_allocator: Default::default(),
        }
    }

    pub fn GetIBlockAllocator(&self, array_id: u32) -> Option<Box<dyn IBlockAllocator>> {
        if let Some(allocator) = self.arrayId_to_allocator.get(&array_id) {
            Some(allocator.boxed_clone())
        } else {
            None
        }
    }
}