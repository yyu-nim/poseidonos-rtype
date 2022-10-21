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
use crate::array_models::interface::i_array_info::ArrayInfo;

lazy_static!{
    static ref array_idx_allocator : AtomicU32 = AtomicU32::new(0);
}

pub struct Array {
    info: Option<ArrayInfo>,
    devMgr_ : ArrayDeviceManager,
    abrControl: Box<dyn IAbrControl>,
    ptnMgr: PartitionManager,
    state: ArrayState,
}

impl Array {

    pub fn new(devMgr_: ArrayDeviceManager,
               abrControl: Box<dyn IAbrControl>, ptnMgr: PartitionManager, state: ArrayState) -> Array {
        Array {
            info: None,
            devMgr_,
            abrControl,
            ptnMgr,
            state,
        }
    }

    pub fn Create(&mut self, name: String, nameSet: DeviceSet<String>, metaFt: String, dataFt: String) -> i32 {
        let index = array_idx_allocator.fetch_add(1, Ordering::Relaxed);
        let uniqueId = unique_id_generator::GenerateUniqueId();

        let info = ArrayInfo {
            name,
            index,
            metaRaidType: metaFt.clone(),
            dataRaidType: dataFt.clone(),
            uniqueId,
            isWriteThroughEnabled: false, // TODO
        };

        self.devMgr_.ImportByName(nameSet);
        let devs = self.devMgr_.ExportToMeta();
        let meta = ArrayMeta::new(info.name.clone(),
                                  devs, info.metaRaidType.clone(),
                                  info.dataRaidType.clone(),
                                  info.uniqueId);

        self.abrControl.CreateAbr(meta.clone());
        let ret = self._Flush(meta.clone());
        if ret != 0 {
            error!("failed to flush array metadata!");
            self._CleanupAfterError(info.name.clone());
            return ret;
        }

        let ret = self._CreatePartitions();
        if ret != 0 {
            error!("failed to create new partitions!");
            self._CleanupAfterError(info.name);
            return ret;
        }
        self.ptnMgr.FormatPartition();
        self.state.SetCreate();

        self.info = Some(info);
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
        self.info.as_ref().unwrap().name.clone()
    }

    pub fn GetIndex(&self) -> u32 {
        self.info.as_ref().unwrap().index.clone()
    }

    pub fn Delete(&self) {
        // TODO
    }

    pub fn GetArrayInfo(&self) -> ArrayInfo {
        self.info.as_ref().expect("Cannot get array info").clone()
    }
}