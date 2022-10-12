use crate::array::meta::device_meta::DeviceMeta;
use crate::array_models::dto::device_set::DeviceSet;

pub struct ArrayMeta {
    devs: DeviceSet<DeviceMeta>,
    id: u32,
    arrayName: String,
    metaRaidType: String,
    dataRaidType: String,
    createDatetime: String,
    updateDatetime: String,
    unique_id: u32,
}