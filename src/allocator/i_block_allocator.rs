use crate::allocator::stripe::stripe::Stripe;
use crate::include::address_type::{StripeId, VirtualBlks};

pub trait IBlockAllocator: Send + Sync {

    fn AllocateWriteBufferBlks(&self, volume_id: u32, num_blks: u32) -> (VirtualBlks, StripeId);
    fn AllocateGcDestStripe(&self, volume_id: u32) -> Stripe;
    fn ProhibitUserBlkAlloc(&self);
    fn PermitUserBlkAlloc(&self);
    fn BlockAllocating(&self, volume_id: u32) -> bool;
    fn UnblockAllocating(&self, volume_id: u32);
    fn TryRdLock(&self, volume_id: u32) -> bool;
    fn Unlock(&self, volume_id: u32) -> bool;

    fn boxed_clone(&self) -> Box<dyn IBlockAllocator>;
}

pub mod tests {
    use crate::allocator::i_block_allocator::IBlockAllocator;
    use crate::allocator::stripe::stripe::Stripe;
    use crate::include::address_type::{StripeId, VirtualBlkAddr, VirtualBlks};

    #[derive(Clone, Copy)]
    pub struct MockIBlockAllocator;
    impl IBlockAllocator for MockIBlockAllocator {
        fn AllocateWriteBufferBlks(&self, volume_id: u32, num_blks: u32) -> (VirtualBlks, StripeId) {
            // TODO
            let virtual_blks = VirtualBlks {
                start_vsa: VirtualBlkAddr { stripe_id: 0, offset: 0 },
                num_blks
            };
            let stripe_id = 0;
            (virtual_blks, stripe_id)
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
            // TODO
            true
        }

        fn Unlock(&self, volume_id: u32) -> bool {
            // TODO
            true
        }

        fn boxed_clone(&self) -> Box<dyn IBlockAllocator> {
            let cloned = self.clone();
            Box::new(cloned)
        }
    }
}