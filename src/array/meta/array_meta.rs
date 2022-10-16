use crate::array::meta::device_meta::DeviceMeta;
use crate::array_models::dto::device_set::DeviceSet;

#[derive(Clone)]
pub struct ArrayMeta {
    pub devs: DeviceSet<DeviceMeta>,
    pub id: u32,
    pub arrayName: String,
    pub metaRaidType: String,
    pub dataRaidType: String,
    pub createDatetime: String,
    pub updateDatetime: String,
    pub unique_id: u32,
}

impl ArrayMeta {
    pub fn new(arrayName: String, devs: DeviceSet<DeviceMeta>,
               metaRaidType: String, dataRaidType: String, uniqueId: u32) -> ArrayMeta {
        ArrayMeta {
            devs,
            id: 0,
            arrayName,
            metaRaidType,
            dataRaidType,
            createDatetime: "TODO".to_string(),
            updateDatetime: "TODO".to_string(),
            unique_id: uniqueId
        }
    }

    pub fn arrayName(&self) -> String {
        self.arrayName.clone()
    }

    pub fn metaRaidType(&self) -> String {
        self.metaRaidType.clone()
    }

    pub fn dataRaidType(&self) -> String {
        self.dataRaidType.clone()
    }
}