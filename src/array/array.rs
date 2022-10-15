use std::borrow::BorrowMut;
use lazy_static::lazy_static;
use log::{error, info};
use crate::array::device::array_device_manager::ArrayDeviceManager;
use crate::array::interface::i_abr_control::IAbrControl;
use crate::array::meta::array_meta::ArrayMeta;
use crate::array::partition::partition_manager::PartitionManager;
use crate::array::state::array_state::ArrayState;
use crate::array_models::dto::device_set::DeviceSet;
use crate::master_context::unique_id_generator;
use std::sync::atomic::{AtomicU32, Ordering};

lazy_static!{
    static ref array_idx_allocator : AtomicU32 = AtomicU32::new(0);
}

pub struct Array {
    name_ : String,
    index_ : u32,
    devMgr_ : ArrayDeviceManager,
    abrControl: Box<dyn IAbrControl>,
    ptnMgr: PartitionManager,
    state: ArrayState,
}

impl Array {

    pub fn new(name_: String, devMgr_: ArrayDeviceManager,
               abrControl: Box<dyn IAbrControl>, ptnMgr: PartitionManager, state: ArrayState) -> Array {
        let index_ = array_idx_allocator.fetch_add(1, Ordering::Relaxed);
        Array {
            name_,
            index_,
            devMgr_,
            abrControl,
            ptnMgr,
            state,
        }
    }

    pub fn Create(&self, nameSet: DeviceSet<String>, metaFs: String, dataFt: String) -> i32 {
        self.devMgr_.ImportByName(nameSet);
        let devs = self.devMgr_.ExportToMeta();
        let uniqueId = unique_id_generator::GenerateUniqueId();
        let meta = ArrayMeta::new(self.name_.clone(),
                                  devs, metaFs, dataFt, uniqueId);

        self.abrControl.CreateAbr(meta.clone());
        let ret = self._Flush(meta.clone());
        if ret != 0 {
            error!("failed to flush array metadata!");
            self._CleanupAfterError(self.name_.clone());
            return ret;
        }

        let ret = self._CreatePartitions();
        if ret != 0 {
            error!("failed to create new partitions!");
            self._CleanupAfterError(self.name_.clone());
            return ret;
        }
        self.ptnMgr.FormatPartition();
        self.state.SetCreate();
        info!("[POS_TRACE_ARRAY_CREATED] Array has been created");

        0
    }

    fn _Flush(&self, meta: ArrayMeta) -> i32 {
        info!("[UPDATE_ABR_DEBUG_MSG] Trying to save Array to MBR, name:{}, metaRaid:{}, dataRaid:{}",
            meta.arrayName(), meta.metaRaidType(), meta.dataRaidType());
        self.abrControl.SaveAbr(meta)
    }

    fn _CreatePartitions(&self) -> i32 {
        // TODO
        info!("TODO: _CreatePartitions() ...");
        0
    }

    fn _CleanupAfterError(&self, arrayName: String) {
        info!("Cleaning up intermediate states of ArrayDeviceManager and AbrControl");
        self.devMgr_.Clear();
        self.abrControl.DeleteAbr(arrayName);
    }

    pub fn GetName(&self) -> String {
        self.name_.clone()
    }

    pub fn GetIndex(&self) -> u32 {
        self.index_.clone()
    }

    pub fn Delete(&self) {
        // TODO
    }
}