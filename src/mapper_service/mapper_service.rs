use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use crate::mapper::i_reversemap::IReverseMap;
use crate::mapper::i_stripemap::IStripeMap;

lazy_static!{
    pub static ref MapperServiceSingleton: Mutex<MapperService> = {
        let svc = MapperService {

        };
        Mutex::new(svc)
    };
}

pub struct MapperService {
    arrayNameToId: HashMap::<String, u32>,
    //iVSAMaps: HashMap::<u32, Arc<Mutex<Box<dyn IVSAMap>>>>, // TODO
    iStripeMaps: HashMap::<u32, Arc<Mutex<Box<dyn IStripeMap>>>>,
    iReverseMaps: HashMap::<u32, Arc<Mutex<Box<dyn IReverseMap>>>>,
    //iMapFlushes: HashMap::<u32, Arc<Mutex<Box<dyn IMapFlush>>>>, // TODO
}

impl MapperService {
    pub fn GetIReverseMap(&self, array_id: u32) -> Option<Arc<Mutex<Box<dyn IReverseMap>>>> {
        let m = self.iReverseMaps.get(&array_id);
        match m {
            None => None,
            Some(i_reverse_map) => {
                Some(i_reverse_map.clone())
            }
        }
    }

    pub fn GetIStripeMap(&self, array_id: u32) -> Option<Arc<Mutex<Box<dyn IStripeMap>>>> {
        let m = self.iStripeMaps.get(&array_id);
        match m {
            None => None,
            Some(i_stripe_map) => {
                Some(i_stripe_map.clone())
            }
        }
    }
}