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
    mapContent: MapContent,
    totalBlks: u64,
    // TODO flushCmdManager and related variables
    arrayId: u32,
    // TODO callback
}

impl VSAMapContent {
    pub fn new(mapId: u32, addrInfo: &MapperAddressInfo) -> Self {
        let fileName = format!("VSAMap.{}.bin", mapId);
        Self {
            mapContent: MapContent::new(mapId, &addrInfo, fileName, MetaFileType::Map),
            totalBlks: 0,
            arrayId: addrInfo.arrayId,
        }
    }

    pub fn InMemoryInit(&mut self, volId: u64, blkCnt: u64, mpageSize: u64) {
        self.totalBlks = blkCnt;
        self.mapContent.Init(
            self.totalBlks,
            std::mem::size_of::<VirtualBlkAddr>() as u64,
            mpageSize,
        );
    }

    pub fn GetEntry(&mut self, rba: BlkAddr) -> VirtualBlkAddr {
        let entries_per_page = self.mapContent.entriesPerPage;
        let map = self.mapContent.GetMutMap();

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
        let entries_per_page = self.mapContent.entriesPerPage;
        let page_num = rba / entries_per_page;

        let old_vsa = {
            // Update map
            let map = self.mapContent.GetMutMap();
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
            let map_header = self.mapContent.GetMutMapHeader();

            // It should be set allocated only when it's newly allocated, but moved to here due to lifetime issues..
            map_header.SetMapAllocated(page_num);

            map_header.UpdateNumUsedBlks(old_vsa);
            map_header.SetTouchedMpageBit(page_num);
        }

        Ok(())
    }

    pub fn GetDirtyPages(&self, start: u64, numEntries: u64) -> MpageList {
        let entries_per_page = self.mapContent.entriesPerPage;
        let start_page_num = start / entries_per_page;
        let end_page_num = (start + numEntries) / entries_per_page;
        let end_offset = (start + numEntries) % entries_per_page;

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
        let map_header = self.mapContent.GetMapHeader();
        map_header.GetNumUsedBlks()
    }

    pub fn InvalidateAllBlocks(&mut self) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_set_entry() {
        let addr_info = MapperAddressInfo {
            maxVsid: 1000,
            blksPerStripe: 128,
            numWbStripes: 10,
            mpageSize: 4032,
            arrayId: 0,
        };
        let mut vsamap_content = VSAMapContent::new(0, &addr_info);
        vsamap_content.InMemoryInit(0, 128 * 1000, 4032);

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
            maxVsid: 1000,
            blksPerStripe: 128,
            numWbStripes: 10,
            mpageSize: 4032,
            arrayId: 0,
        };
        let mut vsamap_content = VSAMapContent::new(0, &addr_info);
        vsamap_content.InMemoryInit(0, 128 * 1000, 4032);

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
