pub const SECTOR_SIZE_SHIFT: usize = 9;
pub const SECTOR_SIZE: usize = 1 << SECTOR_SIZE_SHIFT;
pub const BLOCK_SIZE_SHIFT: usize = 12;
pub const BLOCK_SIZE: usize = 1 << BLOCK_SIZE_SHIFT;
pub const SECTORS_PER_BLOCK_SHIFT: usize = BLOCK_SIZE_SHIFT - SECTOR_SIZE_SHIFT;

pub fn ChangeByteToSector(b: u64) -> u64 {
    b >> SECTOR_SIZE_SHIFT
}

pub fn ChangeByteToBlock(b: u64) -> u64 {
    b >> BLOCK_SIZE_SHIFT
}

pub fn GetByteOffsetInBlock(address: u64) -> u64 {
    address & (BLOCK_SIZE - 1) as u64
}

pub fn ChangeBlockToSector(b: u64) -> u64 {
    b << SECTORS_PER_BLOCK_SHIFT
}

pub fn ChangeSectorToByte(s: u64) -> u64 {
    s << SECTOR_SIZE_SHIFT
}

pub fn DivideUp(v: u64, a: u64) -> u64 {
    (v + a - 1) / a
}

pub fn Align(v: u64, u: u64) -> u64 {
    u * DivideUp(v, u)
}
