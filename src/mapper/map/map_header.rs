use bit_vec::BitVec;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::include::{
    address_type::{IsUnMapVsa, VirtualBlkAddr},
    memory,
};

struct MpageInfo {
    pub num_valid_mpages: u64,
    pub num_total_mpages: u64,
    pub num_used_blocks: u64,
    pub age: u64,
}

pub struct MapHeader {
    age: u64,
    size: u64,
    num_used_blks: AtomicU64,
    mpage_map: BitVec,
    touched_mpages: BitVec,
    map_id: i32,
}

impl MapHeader {
    pub fn new() -> Self {
        Self {
            age: 0,
            size: 0,
            num_used_blks: AtomicU64::new(0),
            mpage_map: BitVec::new(),
            touched_mpages: BitVec::new(),
            map_id: 0,
        }
    }

    pub fn Init(&mut self, num_mpages: u64, mpage_size: u64) {
        self.mpage_map = BitVec::from_elem(num_mpages as usize, false);
        self.touched_mpages = BitVec::from_elem(num_mpages as usize, false);

        let mpage_map_size = self.mpage_map.storage().len() * std::mem::size_of::<u32>();
        let size = std::mem::size_of::<MpageInfo>() + mpage_map_size;
        self.size = memory::Align(size as u64, mpage_size);
    }

    pub fn CopyToBuffer(&self, buffer: Vec<u8>) {
        todo!()
    }
    pub fn GetBitmapFromTempBuffer(&mut self, buffer: Vec<u8>) {
        todo!()
    }
    pub fn GetSize(&self) -> u64 {
        self.size
    }
    pub fn ApplyHeader(&mut self, buffer: Vec<u8>) {
        todo!()
    }
    pub fn GetNumValidMpages(&self) -> u64 {
        self.mpage_map.iter().filter(|x| *x).count() as u64
    }
    pub fn GetMpageMap(&mut self) -> &BitVec {
        &self.mpage_map
    }
    pub fn SetMapAllocated(&mut self, page_num: u64) {
        self.mpage_map.set(page_num as usize, true);
    }
    pub fn GetTouchedMpages(&mut self) -> &BitVec {
        &self.touched_mpages
    }
    pub fn UpdateNumUsedBlks(&mut self, vsa: VirtualBlkAddr) {
        if IsUnMapVsa(&vsa) {
            self.num_used_blks.fetch_add(1, Ordering::SeqCst);
        }
    }
    pub fn GetNumUsedBlks(&self) -> u64 {
        self.num_used_blks.load(Ordering::SeqCst)
    }
    pub fn GetMapId(&self) -> i32 {
        self.map_id
    }
    pub fn GetNumTouchedMpagesSet(&self) -> u64 {
        self.touched_mpages.iter().filter(|x| *x).count() as u64
    }
    pub fn GetNumTotalTouchedMpages(&self) -> u64 {
        self.touched_mpages.len() as u64
    }
    pub fn SetTouchedMpageBit(&mut self, page_num: u64) {
        self.touched_mpages.set(page_num as usize, true);
    }
}

#[cfg(test)]
mod tests {
    use crate::include::{address_type::UNMAP_VSA, memory::Align};

    use super::*;

    #[test]
    fn test_bitvec() {
        // 128 bits / 32 bits = 4 storage required
        let bitmap = BitVec::from_elem(128, false);
        assert_eq!(bitmap.storage().len(), 4);

        // 130 bits / 32 bits = 4 + 1 storage required
        let bitmap = BitVec::from_elem(130, false);
        assert_eq!(bitmap.storage().len(), 5);
        assert_eq!(bitmap.storage().len() * std::mem::size_of::<u32>(), 20);
    }

    #[test]
    fn test_map_header_initialization() {
        let mut header = MapHeader::new();
        header.Init(10, 4032);

        let actual_size = header.GetSize();
        assert_eq!(Align(actual_size, 4032), actual_size);
    }

    #[test]
    fn test_set_mpage_allocated() {
        let mut header = MapHeader::new();
        header.Init(100, 4032);

        header.SetMapAllocated(3);
        header.SetMapAllocated(5);

        assert_eq!(header.GetNumValidMpages(), 2);

        let mpage_map = header.GetMpageMap();
        assert_eq!(mpage_map.get(3), Some(true));
        assert_eq!(mpage_map.get(5), Some(true));
    }

    #[test]
    fn test_num_used_blocks() {
        let mut header = MapHeader::new();
        header.Init(100, 4032);

        header.UpdateNumUsedBlks(UNMAP_VSA);
        header.UpdateNumUsedBlks(UNMAP_VSA);

        assert_eq!(header.GetNumUsedBlks(), 2);
    }

    #[test]
    fn test_touched_mpages() {
        let mut header = MapHeader::new();
        header.Init(100, 4032);

        header.SetTouchedMpageBit(50);
        header.SetTouchedMpageBit(60);

        assert_eq!(header.GetNumTouchedMpagesSet(), 2);

        let touched_mpages = header.GetTouchedMpages();
        assert_eq!(touched_mpages.get(50), Some(true));
        assert_eq!(touched_mpages.get(60), Some(true));
    }
}
