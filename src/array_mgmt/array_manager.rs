use std::borrow::Borrow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use log::{info, warn, error};
use crate::array_components::array_components::ArrayComponents;
use crate::array_models::dto::device_set::DeviceSet;
use anyhow::{Context, Result, bail};
use crate::include::array_mgmt_policy;
use crate::mbr::abr_manager::AbrManager;
use crate::mbr::mbr_info::ArrayBootRecord;
use crate::array::meta::array_meta::ArrayMeta;

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

    pub fn Create(&mut self, name: String, devs: DeviceSet<String>, metaFt: String, dataFt: String) -> Result<()>{
        info!("Creating an array {} with devices {:?} with meta {} and data {}", name, devs, metaFt, dataFt);

        if self._FindArray(&name).is_some() == true {
            warn!("[CREATE_ARRAY_SAME_ARRAY_NAME_EXISTS] name duplicated: {name}");
            bail!("Create array same array name exists".to_string());
        }

        if self.arrayList.len() >= array_mgmt_policy::MAX_ARRAY_CNT {
            let len = self.arrayList.len();
            warn!("[CREATE_ARRAY_EXCEED_MAX_NUM_OF_ARRAYS] Current num of arrays: {len}");

            bail!("Create array exceed max num of arrays".to_string());
        }

        let mut components = ArrayComponents::new();
        // TODO: hands over abr manager
        components.Create(name.clone(), devs, metaFt, dataFt)?;
        self.arrayList.insert(name.clone(), components);

        Ok(())
    }

    pub fn Delete(&mut self, name: String) -> Result<()> {
        let array = match self._FindArray(&name) {
            Some(a) => a,
            None => {
                if self.AbrExist(&name) {
                    self._DeleteFaultArray(&name);
                }

                warn!("[DELETE_ARRAY_ARRAY_NAME_DOES_NOT_EXIST] array_name: {name}");
                bail!("Delete array {name} failed. Do not exist");
            }
        };

        array.Delete()?;
        self.arrayList.remove(&name);

        Ok(())
    }

    fn _FindArray(&mut self, name: &String) -> Option<&mut ArrayComponents> {
        self.arrayList.get_mut(name)
    }

    pub fn AbrExist(&self, name: &String) -> bool {
        let abrList = match self.GetAbrList() {
            Some(list) => list,
            None => {
                error!("Failed to get abr list");
                return false;
            }
        };

        abrList.iter().find(|&abr| &abr.arrayName == name).is_some()
    }

    pub fn GetAbrList(&self) -> Option<&Vec::<ArrayBootRecord>>{
        self.abrManager.GetAbrList()
    }

    fn _DeleteFaultArray(&self, name: &String) -> Result<()> {
        self.abrManager.LoadAbr(name)?;
        self.abrManager.DeleteAbr(name)?;

        Ok(())
    }
}
