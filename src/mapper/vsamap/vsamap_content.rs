use crate::include::address_type::UNMAP_VSA;
use crate::{
    include::{
        address_type::{BlkAddr, VirtualBlkAddr},
        pos_event_id::PosEventId,
    },
    mapper::{
        address::mapper_address_info::MapperAddressInfo, include::mpage_info::MpageList,
        map::map_content::MapContent,
    },
    metafs::include::mf_property::MetaFileType,
};
pub struct VSAMapContent {
    map_content: MapContent,
    total_blks: u64,
    // TODO flushCmdManager and related variables
    array_id: u32,
    // TODO callback
}

impl VSAMapContent {
    pub fn new(map_id: i32, addr_info: &MapperAddressInfo) -> Self {
        let fileName = format!("VSAMap.{}.bin", map_id);
        Self {
            map_content: MapContent::new(map_id, &addr_info, fileName, MetaFileType::Map),
            total_blks: 0,
            array_id: addr_info.array_id,
        }
    }

    pub fn InMemoryInit(&mut self, vol_id: u64, blk_cnt: u64, mpage_size: u64) -> Result<(), PosEventId> {
        self.total_blks = blk_cnt;
        self.map_content.Init(
            self.total_blks,
            std::mem::size_of::<VirtualBlkAddr>() as u64,
            mpage_size,
        );

        Ok(())
    }

    pub fn GetEntry(&mut self, rba: BlkAddr) -> VirtualBlkAddr {
        let entries_per_page = self.map_content.GetEntriesPerPage();
        let map = self.map_content.GetMapMut();

        let page_num = rba / entries_per_page;
        let mpage = map.GetMpage(page_num);
        if mpage.is_none() || mpage.as_deref().unwrap().is_empty() {
            return UNMAP_VSA;
        } else {
            let entry_offset =
                (rba % entries_per_page) as usize * std::mem::size_of::<VirtualBlkAddr>();
            let vsa =
                unsafe { std::ptr::read(mpage.unwrap().as_ptr().add(entry_offset) as *const _) };
            return vsa;
        }
    }

    pub fn SetEntry(&mut self, rba: BlkAddr, vsa: VirtualBlkAddr) -> Result<(), PosEventId> {
        let entries_per_page = self.map_content.GetEntriesPerPage();
        let page_num = rba / entries_per_page;

        let old_vsa = {
            // Update map
            let map = self.map_content.GetMapMut();
            let mut mpage = map.GetMpage(page_num);

            if mpage.is_none() || mpage.as_deref().unwrap().is_empty() {
                mpage = map.AllocateMpage(page_num);
                if mpage.is_none() {
                    return Err(PosEventId::VSAMAP_SET_FAILURE);
                }
            }

            let entry_offset =
                (rba % entries_per_page) as usize * std::mem::size_of::<VirtualBlkAddr>();
            let old_vsa = unsafe {
                std::ptr::read(mpage.as_ref().unwrap().as_ptr().add(entry_offset) as *const _)
            };

            unsafe {
                std::ptr::write(
                    mpage.as_ref().unwrap().as_ptr().add(entry_offset) as *mut VirtualBlkAddr,
                    vsa,
                )
            };

            old_vsa
        };

        {
            // Update map header
            let map_header = self.map_content.GetMapHeaderMut();

            // It should be set allocated only when it's newly allocated, but moved to here due to lifetime issues..
            map_header.SetMapAllocated(page_num);

            map_header.UpdateNumUsedBlks(old_vsa);
            map_header.SetTouchedMpageBit(page_num);
        }

        Ok(())
    }

    pub fn GetDirtyPages(&self, start: u64, num_entries: u64) -> MpageList {
        let entries_per_page = self.map_content.GetEntriesPerPage();
        let start_page_num = start / entries_per_page;
        let end_page_num = (start + num_entries) / entries_per_page;
        let end_offset = (start + num_entries) % entries_per_page;

        let mut dirtyList = MpageList::new();

        for page_num in start_page_num..end_page_num {
            dirtyList.insert(page_num);
        }

        if end_offset != 0 {
            dirtyList.insert(end_page_num);
        }

        dirtyList
    }

    pub fn GetNumUsedBlks(&self) -> u64 {
        let map_header = self.map_content.GetMapHeader();
        map_header.GetNumUsedBlks()
    }

    pub fn InvalidateAllBlocks(&mut self) {
        todo!();
    }

    pub fn OpenMapFile(&self) -> Result<(), PosEventId> {
        // TODO
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set_entry() {
        let addr_info = MapperAddressInfo {
            max_vsid: 1000,
            blks_per_stripe: 128,
            num_wb_stripes: 10,
            mpage_size: 4032,
            array_id: 0,
        };
        let mut vsamap_content = VSAMapContent::new(0, &addr_info);
        assert_eq!(vsamap_content.InMemoryInit(0, 128 * 1000, 4032).is_ok(), true);

        assert_eq!(vsamap_content.GetEntry(10), UNMAP_VSA);
        let expected = VirtualBlkAddr {
            stripe_id: 30,
            offset: 5,
        };
        assert_eq!(vsamap_content.SetEntry(10, expected).is_ok(), true);
        assert_eq!(vsamap_content.GetEntry(10), expected);
    }

    #[test]
    fn test_dirty_page_list() {
        let addr_info = MapperAddressInfo {
            max_vsid: 1000,
            blks_per_stripe: 128,
            num_wb_stripes: 10,
            mpage_size: 4032,
            array_id: 0,
        };
        let mut vsamap_content = VSAMapContent::new(0, &addr_info);
        assert_eq!(vsamap_content.InMemoryInit(0, 128 * 1000, 4032).is_ok(), true);

        // sizeof(VirtualBlkAddr) = 16
        // entries_per_mpage = 4032 / sizeof(VirtualBlkAddr) = 4032/16 = 252 expected

        let dirty_list = vsamap_content.GetDirtyPages(0, 250);
        assert_eq!(dirty_list.len(), 1);
        assert_eq!(dirty_list.get(&0).is_some(), true);

        let dirty_list = vsamap_content.GetDirtyPages(0, 400);
        assert_eq!(dirty_list.len(), 2);
        assert_eq!(dirty_list.get(&0).is_some(), true);
        assert_eq!(dirty_list.get(&1).is_some(), true);
    }
}
