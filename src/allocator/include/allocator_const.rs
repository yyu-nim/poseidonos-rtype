use crate::volume::volume_base::MAX_VOLUME_COUNT;

pub const ACTIVE_STRIPE_TAIL_ARRAYLEN: usize = MAX_VOLUME_COUNT;

#[repr(C)]
pub struct CtxHeader {
    pub sig: u32,
    pub ctxVersion: u64,
}

impl Default for CtxHeader {
    fn default() -> Self {
        CtxHeader {
            sig: 0,
            ctxVersion: 0
        }
    }
}

#[repr(C)]
pub struct AllocatorCtxHeader {
    pub header: CtxHeader,
    pub numValidWbLsid: u32,
}

impl Default for AllocatorCtxHeader {
    fn default() -> Self {
        AllocatorCtxHeader {
            header: Default::default(),
            numValidWbLsid: 0
        }
    }
}