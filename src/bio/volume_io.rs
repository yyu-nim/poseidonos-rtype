use std::sync::{Arc, Mutex};
use log::{info, warn};
use crate::bio::ubio::{CallbackClosure, Ubio, UbioDir};
use crate::event_scheduler::callback::Callback;
use crate::include::address_type::{StripeAddr, VirtualBlkAddr};
use crate::include::memory::{ChangeByteToSector, ChangeSectorToByte};

//#[derive(Copy, Clone)]
pub struct VolumeIo {
    pub vol_id: Option<u32>,
    pub origin_core: Option<u32>,
    pub lsid_entry: Option<StripeAddr>,
    pub old_lsid_entry: Option<StripeAddr>,
    pub vsa: Option<VirtualBlkAddr>,
    pub sector_rba: Option<u64>,
    pub stripe_id: Option<u64>,
    pub ubio: Option<Arc<Mutex<Ubio>>>, // pos-cpp implements this as inheritance, but pos-rtype will do so with composition
}

impl VolumeIo {

    pub fn new(data_buffer: Option<Arc<Mutex<Vec<u8>>>>,
               unit_count: u32,
               array_id: u32) -> VolumeIo {
        let ubio = Ubio::new(data_buffer, None, array_id as i32);
        VolumeIo {
            vol_id: None,
            origin_core: None,
            lsid_entry: None,
            old_lsid_entry: None,
            vsa: None,
            sector_rba: None,
            stripe_id: None,
            ubio: Some(Arc::new(Mutex::new(ubio))),
        }
    }

    pub fn Split(&mut self, sectors: u32, removal_from_tail: bool) -> VolumeIo {
        let mut new_volume_io = VolumeIo {
            vol_id: self.vol_id.clone(),
            origin_core: self.origin_core.clone(),
            lsid_entry: self.lsid_entry.clone(),
            old_lsid_entry: self.old_lsid_entry.clone(),
            vsa: None, // pos-cpp: vsa(INVALID_VSA)
            sector_rba: self.sector_rba.clone(),
            stripe_id: None, // pos-cpp: stripeId(UNMAP_STRIPE)
            ubio: Some(self.ubio.as_ref().unwrap().clone()),
        };

        self._ReflectSplit(&mut new_volume_io, sectors, removal_from_tail);
        if removal_from_tail {
            let data_buffer_size = new_volume_io.ubio.as_ref().unwrap().lock().unwrap()
                .dataBuffer.as_ref().unwrap().size();
            let new_sector_rba = new_volume_io.sector_rba.unwrap()
                + ChangeByteToSector(data_buffer_size as u64);
            new_volume_io.sector_rba = Some(new_sector_rba);
        } else {
            let new_sector_rba = self.sector_rba.unwrap() + sectors as u64;
            self.sector_rba = Some(new_sector_rba);
        }

        new_volume_io
    }

    fn _ReflectSplit(&self, new_volume_io: &mut VolumeIo, sectors: u32, removal_from_tail: bool) {
        let mut ubio_inner = new_volume_io.ubio.as_ref().unwrap().lock().unwrap();
        let removal_size = ChangeSectorToByte(sectors as u64);
        let remaining_size = ubio_inner.dataBuffer.as_ref().unwrap().size() - removal_size as usize;
        ubio_inner.dataBuffer.as_mut().unwrap().remove(remaining_size as u64, !removal_from_tail);
    }
}