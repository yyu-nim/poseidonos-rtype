use crate::{mapper::{map::map_content::MapContent, address::mapper_address_info::MapperAddressInfo, include::mpage_info::MpageList}, metafs::include::mf_property::MetaFileType, include::{address_type::{StripeId, StripeAddr, UNMAP_STRIPE, StripeLoc}, pos_event_id::PosEventId}};

pub struct StripeMapContent {
    map_content: MapContent
}

impl StripeMapContent {
    pub fn new(map_id: i32, addr_info: &MapperAddressInfo) -> Self {
        Self {
            map_content: MapContent::new(map_id, addr_info, "StripeMap.bin".to_string(), MetaFileType::Map),
        }
    }

    pub fn InMemoryInit(&mut self, num_entries: u64, mpage_size: u64) {
        self.map_content.Init(
            num_entries,
            std::mem::size_of::<StripeAddr>() as u64,
            mpage_size);
    }

    pub fn GetEntry(&mut self, vsid: StripeId) -> StripeAddr {
        let entries_per_mpage = self.map_content.GetEntriesPerPage();
        let page_num = vsid as u64 / entries_per_mpage;
        let mpage = self.map_content.GetMapMut().GetMpage(page_num);

        if mpage.is_none() || mpage.as_deref().unwrap().is_empty() {
            return StripeAddr {
                stripe_id: UNMAP_STRIPE,
                stripe_loc: StripeLoc::IN_WRITE_BUFFER_AREA,
            };
        } else {
            let entry_offset = (vsid as u64 % entries_per_mpage) as usize * std::mem::size_of::<StripeAddr>();
            let stripe_addr = unsafe {
                std::ptr::read(mpage.unwrap().as_ptr().add(entry_offset) as *const _ )
            };
            return stripe_addr;
        }
    }

    pub fn SetEntry(&mut self, vsid: StripeId, entry: StripeAddr) -> Result<(), PosEventId> {
        let entries_per_mpage = self.map_content.GetEntriesPerPage();
        let page_num = vsid as u64 / entries_per_mpage;
        
        {
            // Update map
            let map = self.map_content.GetMapMut();
            let mut mpage = map.GetMpage(page_num);

            if mpage.is_none() || mpage.as_deref().unwrap().is_empty() {
                mpage = map.AllocateMpage(page_num);
                if mpage.is_none() {
                    return Err(PosEventId::STRIPEMAP_SET_FAILURE);
                }
            }

            let entry_offset = (vsid as u64 % entries_per_mpage) as usize * std::mem::size_of::<StripeAddr>();
            unsafe {
                std::ptr::write(
                    mpage.as_ref().unwrap().as_ptr().add(entry_offset) as *mut StripeAddr,
                    entry
                )
            };
        }   

        {
            // Update map header
            let map_header = self.map_content.GetMapHeaderMut();
        
            map_header.SetMapAllocated(page_num);
            map_header.SetTouchedMpageBit(page_num);
        }

        Ok(())
    }

    pub fn GetDirtyPages(&self, start: u64, num_entries: u64) -> MpageList {
        assert!(num_entries == 1);

        let entries_per_mpage = self.map_content.GetEntriesPerPage();
        let mut list = MpageList::new();
        list.insert(start / entries_per_mpage);

        return list;
    }
}

#[cfg(test)]
mod tests {
    use crate::mapper::stripemap;

    use super::*;

    #[test]
    fn test_get_set_entry() {
        let addr_info = MapperAddressInfo {
            max_vsid: 1000,
            blks_per_stripe: 128,
            num_wb_stripes: 20,
            mpage_size: 4032,
            array_id: 0,
        };
        let mut stripemap_content = StripeMapContent::new(0, &addr_info);
        stripemap_content.InMemoryInit(1000, 4032);

        assert_eq!(stripemap_content.GetEntry(20).stripe_id, UNMAP_STRIPE);
        let expected = StripeAddr {
            stripe_id: 600,
            stripe_loc: StripeLoc::IN_USER_AREA,
        };
        assert_eq!(stripemap_content.SetEntry(20, expected).is_ok(), true);
        assert_eq!(stripemap_content.GetEntry(20), expected);
    }

    #[test]
    fn test_dirty_page_list() {
        let addr_info = MapperAddressInfo {
            max_vsid: 1000,
            blks_per_stripe: 128,
            num_wb_stripes: 20,
            mpage_size: 4032,
            array_id: 0,
        };
        let mut stripemap_content = StripeMapContent::new(0, &addr_info);
        stripemap_content.InMemoryInit(1000, 4032);

        // sizeof(StripeAddr) = 8
        // entries_per_mpage = 4032 / sizeof(StripeAddr) = 4032/8 = 504 expected
        let dirty_list = stripemap_content.GetDirtyPages(0, 1);
        assert_eq!(dirty_list.len(), 1);
        assert_eq!(dirty_list.get(&0).is_some(), true);

        let dirty_list = stripemap_content.GetDirtyPages(550, 1);
        assert_eq!(dirty_list.len(), 1);
        assert_eq!(dirty_list.get(&1).is_some(), true);
    }
}