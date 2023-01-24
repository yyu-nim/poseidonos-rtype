use crate::include::memory::DivideUp;
use crate::include::pos_event_id::PosEventId;
use crate::mapper::address::mapper_address_info::MapperAddressInfo;
use crate::metafs::include::mf_property::MetaFileType;

use super::map::Map;
use super::map_header::MapHeader;

pub struct MapContent {
    mapHeader: MapHeader,
    map: Map,
    // TODO add mapIoHandler
    fileName: String,
    fileType: MetaFileType,
    mapId: u32,
    pub entriesPerPage: u64,
    addrInfo: MapperAddressInfo,
    isInitialized: bool,
}

impl MapContent {
    pub fn new(
        mapId: u32,
        addrInfo: &MapperAddressInfo,
        fileName: String,
        fileType: MetaFileType,
    ) -> Self {
        Self {
            mapHeader: MapHeader::new(),
            map: Map::new(),
            fileName,
            fileType,
            mapId,
            entriesPerPage: 0,
            addrInfo: addrInfo.clone(),
            isInitialized: false,
        }
    }

    pub fn Init(&mut self, numEntires: u64, entrySize: u64, mpageSize: u64) {
        if self.isInitialized == false {
            self.isInitialized = true;

            self.entriesPerPage = mpageSize / entrySize;
            let numMpages = DivideUp(numEntires, self.entriesPerPage);
            self.map.Init(numMpages, mpageSize);
            self.mapHeader.Init(numMpages, mpageSize);
        }
    }

    pub fn Dispose(&mut self) {
        self.isInitialized = false;
    }

    pub fn OpenMapFile(&mut self) -> Result<(), PosEventId> {
        todo!();
    }

    pub fn GetMap(&self) -> &Map {
        &self.map
    }
    pub fn GetMapHeader(&self) -> &MapHeader {
        &self.mapHeader
    }
    pub fn GetMutMap(&mut self) -> &mut Map {
        &mut self.map
    }
    pub fn GetMutMapHeader(&mut self) -> &mut MapHeader {
        &mut self.mapHeader
    }
}
