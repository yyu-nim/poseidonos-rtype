use crate::include::memory::DivideUp;
use crate::include::pos_event_id::PosEventId;
use crate::mapper::address::mapper_address_info::MapperAddressInfo;
use crate::metafs::include::mf_property::MetaFileType;

use super::map::Map;
use super::map_header::MapHeader;

pub struct MapContent {
    map_header: MapHeader,
    map: Map,
    // TODO add mapIoHandler
    file_name: String,
    file_type: MetaFileType,
    map_id: i32,
    entries_per_page: u64,
    addr_info: MapperAddressInfo,
    is_initialized: bool,
}

impl MapContent {
    pub fn new(
        mapId: i32,
        addrInfo: &MapperAddressInfo,
        fileName: String,
        fileType: MetaFileType,
    ) -> Self {
        Self {
            map_header: MapHeader::new(),
            map: Map::new(),
            file_name: fileName,
            file_type: fileType,
            map_id: mapId,
            entries_per_page: 0,
            addr_info: addrInfo.clone(),
            is_initialized: false,
        }
    }

    pub fn Init(&mut self, num_entries: u64, entry_size: u64, mpage_size: u64) {
        if self.is_initialized == false {
            self.is_initialized = true;

            self.entries_per_page = mpage_size / entry_size;
            let numMpages = DivideUp(num_entries, self.entries_per_page);
            self.map.Init(numMpages, mpage_size);
            self.map_header.Init(numMpages, mpage_size);
        }
    }

    pub fn Dispose(&mut self) {
        self.is_initialized = false;
    }

    pub fn OpenMapFile(&mut self) -> Result<(), PosEventId> {
        todo!();
    }

    pub fn GetMap(&self) -> &Map {
        &self.map
    }
    pub fn GetMapHeader(&self) -> &MapHeader {
        &self.map_header
    }
    pub fn GetMapMut(&mut self) -> &mut Map {
        &mut self.map
    }
    pub fn GetMapHeaderMut(&mut self) -> &mut MapHeader {
        &mut self.map_header
    }

    pub fn GetEntriesPerPage(&self) -> u64 {
        self.entries_per_page
    }
}
