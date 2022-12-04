const SECTOR_SIZE_SHIFT: usize = 9;
const SECTOR_SIZE: usize = 1 << SECTOR_SIZE_SHIFT;

pub fn ChangeByteToSector(b: u64) -> u64 {
    b >> SECTOR_SIZE_SHIFT
}