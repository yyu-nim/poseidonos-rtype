#[derive(Clone, Debug)]
pub struct PartitionLogicalSize {
    pub min_write_blk_cnt: u32,
    pub blks_per_chunk: u32,
    pub blks_per_stripe: u32,
    pub chunks_per_stripe: u32,
    pub stripes_per_segment: u32,
    pub total_stripes: u32,
    pub total_segments: u32,
}