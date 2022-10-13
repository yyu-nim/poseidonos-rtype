use log::info;
use crate::array::meta::device_meta::DeviceMeta;
use crate::array_models::dto::device_set::DeviceSet;

pub struct ArrayDeviceManager;

impl ArrayDeviceManager {
    pub fn ImportByName(&self, nameSet: DeviceSet<String>) -> i32 {
        // TODO
        info!("Importing {:?}...", nameSet);
        0
    }

    pub fn ExportToMeta(&self) -> DeviceSet<DeviceMeta> {
        // TODO
        info!("Exporting devices info with DeviceMeta");
        let nvm = Vec::<DeviceMeta>::new();
        let data = Vec::<DeviceMeta>::new();
        let spares = Vec::<DeviceMeta>::new();
        DeviceSet::new(nvm, data, spares)
    }

    pub fn Clear(&self) {
        // TODO
    }
}