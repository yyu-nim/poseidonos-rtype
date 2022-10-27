use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use log::{info, warn, error};
use crate::array_components::array_components::ArrayComponents;
use crate::array_models::dto::device_set::DeviceSet;
use crate::include::array_mgmt_policy;
use crate::mbr::abr_manager::AbrManager;
use crate::mbr::mbr_info::ArrayBootRecord;
use crate::array::meta::array_meta::ArrayMeta;
use crate::include::pos_event_id::PosEventId;

lazy_static!{
    pub static ref ArrayManagerSingleton: Arc<Mutex<ArrayManager>> = {
        let array_manager = ArrayManager::new();
        info!("ArrayManager has been created");
        Arc::new(Mutex::new(array_manager))
    };
}

pub struct ArrayManager {
    arrayList: HashMap::<String, ArrayComponents>,
    abrManager: AbrManager,
}

impl ArrayManager {
    fn new() -> ArrayManager {
        ArrayManager {
            arrayList: HashMap::new(),
            abrManager: AbrManager::new(),
        }
    }

    pub fn Create(&mut self, name: String, devs: DeviceSet<String>, metaFt: String, dataFt: String) -> Result<(), PosEventId>{
        info!("Creating an array {} with devices {:?} with meta {} and data {}", name, devs, metaFt, dataFt);

        if self._FindArray(&name).is_some() == true {
            let eventId = PosEventId::CREATE_ARRAY_SAME_ARRAY_NAME_EXISTS;
            warn!("[{}] name duplicated: {name}", eventId.to_string());
            return Err(eventId);
        }

        if self.arrayList.len() >= array_mgmt_policy::MAX_ARRAY_CNT {
            let len = self.arrayList.len();

            let eventId = PosEventId::CREATE_ARRAY_EXCEED_MAX_NUM_OF_ARRAYS;
            warn!("[{}] Current num of arrays: {len}", eventId.to_string());

            return Err(eventId);
        }

        let mut components = ArrayComponents::new();
        // TODO: hands over abr manager
        components.Create(name.clone(), devs, metaFt, dataFt)?;
        self.arrayList.insert(name.clone(), components);

        Ok(())
    }

    pub fn Delete(&mut self, name: String) -> Result<(), PosEventId> {
        let array = match self._FindArray(&name) {
            Some(a) => a,
            None => {
                if self.AbrExists(&name) {
                    self._DeleteFaultArray(&name);
                }

                let eventId = PosEventId::DELETE_ARRAY_ARRAY_NAME_DOES_NOT_EXIST;
                warn!("[{}] array_name: {name}", eventId.to_string());
                return Err(eventId);
            }
        };

        array.Delete()?;
        self.arrayList.remove(&name);

        Ok(())
    }

    fn _FindArray(&mut self, name: &String) -> Option<&mut ArrayComponents> {
        self.arrayList.get_mut(name)
    }

    pub fn AbrExists(&self, name: &String) -> bool {
        let abrList = match self.GetAbrList() {
            Ok(list) => list,
            Err(e) => {
                error!("Failed to get abr list, {e}");
                return false;
            }
        };

        abrList.iter().find(|&abr| &std::str::from_utf8(&abr.arrayName).unwrap() == name).is_some()
    }

    pub fn GetAbrList(&self) -> Result<Vec::<ArrayBootRecord>, PosEventId>{
        self.abrManager.GetAbrList()
    }

    fn _DeleteFaultArray(&self, name: &String) -> Result<(), PosEventId> {
        self.abrManager.LoadAbr(name)?;
        self.abrManager.DeleteAbr(name)?;

        Ok(())
    }
}
