use std::borrow::BorrowMut;
use lazy_static::lazy_static;
use log::{error, info, warn};
use crate::array::device::array_device_manager::ArrayDeviceManager;
use crate::array::interface::i_abr_control::IAbrControl;
use crate::array::meta::array_meta::ArrayMeta;
use crate::array::partition::partition_manager::PartitionManager;
use crate::array::state::array_state::ArrayState;
use crate::array_models::dto::device_set::DeviceSet;
use crate::master_context::unique_id_generator;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::array::array_name_policy;
use crate::array_models::interface::i_array_info::ArrayInfo;
use crate::include::pos_event_id::PosEventId::*;
use crate::include::raid_type::RaidTypeEnum;

lazy_static!{
    static ref array_idx_allocator : AtomicU32 = AtomicU32::new(0);
}

pub struct Array {
    info: Option<ArrayInfo>,
    devMgr_ : ArrayDeviceManager,
    abrControl: Box<dyn IAbrControl>,
    ptnMgr: PartitionManager,
    state: ArrayState,
    index: u32,
}

impl std::fmt::Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.info {
            Some(info) =>f.write_str(&info.to_string())?,
            None => {},
        };
        write!(f, ", index: {}", self.index)
    }
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
            index: 0,
        }
    }

    pub fn Create(&mut self, name: String, nameSet: DeviceSet<String>, metaFt: String, dataFt: String) -> i32 {
        let dataRaidType = RaidTypeEnum::from(dataFt.as_str());
        let metaRaidType = RaidTypeEnum::from(metaFt.as_str());
        info!("[CREATE_ARRAY_DEBUG_MSG] Trying to create array({}), dataFt: {}, metaFt:{}",
            &name, dataRaidType.to_string(), metaRaidType.to_string());

        if dataRaidType == RaidTypeEnum::NOT_SUPPORTED ||
            metaRaidType == RaidTypeEnum::NOT_SUPPORTED {
            let eventId = CREATE_ARRAY_NOT_SUPPORTED_RAIDTYPE as i32;
            warn!("[CREATE_ARRAY_NOT_SUPPORTED_RAIDTYPE],\
                metaFt: {}, dataFt: {}", metaFt, dataFt);

            return eventId;
        }

        let canAddSpare = dataRaidType != RaidTypeEnum::NONE && dataRaidType != RaidTypeEnum::RAID0;
        if canAddSpare == false && nameSet.spares.len() > 0 {
            let eventId = CREATE_ARRAY_RAID_DOES_NOT_SUPPORT_SPARE_DEV as i32;
            warn!("[CREATE_ARRAY_RAID_DOES_NOT_SUPPORT_SPARE_DEV], RaidType: {}", dataRaidType.to_string());

            return eventId;
        }

        //TODO pthread_rwlock_wrlock(&stateLock);

        let ret = self.devMgr_.ImportByName(nameSet);
        if ret != 0 {
            error!("Import device manager failed, ret: {} array: {}", ret, name);
            self._CleanupAfterError();
            return ret;
        }

        let ret = array_name_policy::CheckArrayName(&name);
        if ret != 0 {
            error!("Unable to create array due to invalid name, ret: {}, array: {}", ret, &name);
            self._CleanupAfterError();
            return ret;
        }

        let index = array_idx_allocator.fetch_add(1, Ordering::Relaxed);
        let uniqueId = unique_id_generator::GenerateUniqueId();

        let info = ArrayInfo {
            name: name.clone(),
            index,
            metaRaidType: metaFt.clone(),
            dataRaidType: dataFt.clone(),
            uniqueId,
            isWriteThroughEnabled: false, // TODO
        };

        let devs = self.devMgr_.ExportToMeta();
        let meta = ArrayMeta::new(info.name.clone(),
                                  devs, info.metaRaidType.clone(),
                                  info.dataRaidType.clone(),
                                  info.uniqueId);

        self.info = Some(info);

        let ret = self.abrControl.CreateAbr(meta.clone());
        if ret != 0 {
            error!("[{}] Unable to create array({})", ret, &name);
            self._CleanupAfterError();
            return ret;
        } else {
            self.index = meta.id;
        }

        let ret = self._Flush(meta.clone());
        if ret != 0 {
            error!("failed to flush array metadata!");
            self.abrControl.DeleteAbr(name.clone());
            self._CleanupAfterError();
            return ret;
        }

        let ret = self._CreatePartitions();
        if ret != 0 {
            error!("failed to create new partitions!");
            self.abrControl.DeleteAbr(name.clone());
            self._CleanupAfterError();
            return ret;
        }
        self.ptnMgr.FormatPartition();
        self.state.SetCreate();

        info!("[POS_TRACE_ARRAY_CREATED] Array has been created");

        // TODO array metrics publisher..
        self.state.SetCreate();
        // TODO pthread_rwlock_unlock(&stateLock);
        info!("[POS_TRACE_ARRAY_CREATED] {}", self.to_string());

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

    fn _CleanupAfterError(&self) {
        info!("Cleaning up intermediate states of ArrayDeviceManager");
        self.devMgr_.Clear();
        // TODO pthread_rwlock_unlock(&stateLock);
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