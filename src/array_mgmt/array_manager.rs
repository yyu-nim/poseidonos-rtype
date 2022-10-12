use lazy_static::lazy_static;
use log::info;
use crate::array_components::array_components::ArrayComponents;
use crate::array_models::dto::device_set::DeviceSet;
lazy_static!{
    pub static ref ArrayManagerSingleton: ArrayManager = {
        let array_manager = ArrayManager::new();
        info!("ArrayManager has been created");
        array_manager
    };
}

pub struct ArrayManager {
    array_components: ArrayComponents,
}

impl ArrayManager {
    fn new() -> ArrayManager {
        ArrayManager {
            array_components: ArrayComponents::new(),
        }
    }

    pub fn Create(&mut self, name: String, devs: DeviceSet<String>, metaFt: String, dataFt: String) {
        // TODO
        info!("Creating an array {} with devices {:?} with meta {} and data {}", name, devs, metaFt, dataFt);


    }
}