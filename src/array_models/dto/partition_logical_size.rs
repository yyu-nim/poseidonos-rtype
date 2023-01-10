#[derive(Clone, Debug)]
pub struct PartitionLogicalSize {
    pub minWriteBlkCnt: u32,
    pub blksPerChunk: u32,
    pub blksPerStripe: u32,
    pub chunksPerStripe: u32,
    pub stripesPerSegment: u32,
    pub totalStripes: u32,
    pub totalSegments: u32,
}