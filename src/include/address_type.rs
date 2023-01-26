use crate::include::i_array_device::IArrayDevice;

pub type StripeId = u32;
pub type BlkOffset = u64;
pub type BlkAddr = u64;

pub const STRIPE_ID_BIT_LEN: u32 = 30;
pub const BLOCK_OFFSET_BIT_LEN: u32 = 33;
pub const UNMAP_STRIPE: StripeId = (1 << STRIPE_ID_BIT_LEN) - 1;
pub const UNMAP_OFFSET: BlkOffset = (1 << BLOCK_OFFSET_BIT_LEN) - 1;
pub const UNMAP_VSA: VirtualBlkAddr = VirtualBlkAddr {
    stripe_id: UNMAP_STRIPE,
    offset: UNMAP_OFFSET,
};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct VirtualBlkAddr {
    pub stripe_id: StripeId,
    pub offset: BlkOffset,
}

#[derive(Clone)]
pub struct PhysicalBlkAddr
{
    pub lba: u64,
    pub array_dev: Option<Box<dyn IArrayDevice>>,
}

#[derive(Copy, Clone)]
pub struct LogicalBlkAddr
{
    pub stripeId: StripeId,
    pub offset: BlkOffset,
}

#[derive(Copy, Clone)]
pub struct VirtualBlks {
    pub start_vsa: VirtualBlkAddr,
    pub num_blks: u32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct StripeAddr {
    pub stripe_loc: StripeLoc,
    pub stripe_id: StripeId,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum StripeLoc {
    IN_USER_AREA,
    IN_WRITE_BUFFER_AREA,
}

#[derive(Clone)]
pub struct PhysicalEntry
{
    pub addr: PhysicalBlkAddr,
    pub blk_cnt: u32,
}


impl PhysicalEntry {
    pub fn IncrementsLbaBy(&mut self, sectors: u64) {
        self.addr.lba += sectors;
    }
}

pub fn IsUnMapVsa(vsa: &VirtualBlkAddr) -> bool {
    (vsa.stripe_id == UNMAP_STRIPE) && (vsa.offset == UNMAP_OFFSET)
}