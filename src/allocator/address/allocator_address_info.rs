pub struct AllocatorAddressInfo {
    pub blks_per_stripe: u32,
    pub chunks_per_stripe: u32,
    pub num_wb_stripes: u32,
    pub num_userarea_stripes: u32,
    pub blks_per_segment: u32,
    pub stripes_per_segment: u32,
    pub num_userarea_segments: u32,
}

impl Default for AllocatorAddressInfo {
    fn default() -> Self {
        AllocatorAddressInfo {
            blks_per_stripe: 0,
            chunks_per_stripe: 0,
            num_wb_stripes: 0,
            num_userarea_stripes: 0,
            blks_per_segment: 0,
            stripes_per_segment: 0,
            num_userarea_segments: 0
        }
    }
}