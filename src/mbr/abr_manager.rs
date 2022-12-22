use crate::mbr::mbr_manager::MbrManager;
use crate::array::meta::array_meta::ArrayMeta;
use crate::array::meta::device_meta::DeviceMeta;
use crate::array_models::dto::device_set::DeviceSet;
use crate::array::device::array_device_type::ArrayDeviceType;

use log::{error, warn};

pub struct AbrManager {
    mbrManager: MbrManager,
}

use crate::include::array_device_state::ArrayDeviceState;
use crate::include::pos_event_id::PosEventId;
use crate::mbr::mbr_info::{ArrayBootRecord, MBR_ABR_OFFSET};

impl AbrManager {
    pub fn new() -> Self {
        AbrManager {
            mbrManager: MbrManager::new(None),
        }
    }

    pub fn LoadAbr(&self, name: &String) -> Result<ArrayMeta, PosEventId> {
        let (abr, index) = match self.mbrManager.GetAbr(name) {
            Some((a, i)) => (a, i),
            None => {
                let eventId = PosEventId::MBR_ABR_NOT_FOUND;
                warn!("[{}] No array found with arrayName {}", eventId.to_string(), name);
                return Err(eventId);
            }
        };

        let nvmNum = abr.nvmDevNum;
        let dataNum = abr.dataDevNum;
        let spareNum = abr.spareDevNum;

        let mut devIndex: usize = 0;

        let mut nvm = Vec::new();
        for i in 0..nvmNum {
            devIndex = i as usize;
            let uid = std::str::from_utf8(&abr.devInfo[devIndex].deviceUid).unwrap().to_string();
            nvm.push(DeviceMeta::new(uid, ArrayDeviceState::NORMAL));
        }

        let mut data = Vec::new();
        for i in 0..dataNum {
            devIndex = (nvmNum + i) as usize;
            let uid = std::str::from_utf8(&abr.devInfo[devIndex].deviceUid).unwrap().to_string();
            let state = ArrayDeviceState::from(abr.devInfo[devIndex].deviceState);
            data.push(DeviceMeta::new(uid, state));
        }

        let mut spares = Vec::new();
        for i in 0..spareNum {
            devIndex = (nvmNum + dataNum + i) as usize;
            let uid = std::str::from_utf8(&abr.devInfo[devIndex].deviceUid).unwrap().to_string();
            spares.push(DeviceMeta::new(uid, ArrayDeviceState::NORMAL));
        }

        let arrayName = std::str::from_utf8(&abr.arrayName).unwrap().to_string();
        let metaRaidType = std::str::from_utf8(&abr.metaRaidType).unwrap().to_string();
        let dataRaidType = std::str::from_utf8(&abr.dataRaidType).unwrap().to_string();
        let createDatetime = std::str::from_utf8(&abr.createDatetime).unwrap().to_string();
        let updateDatetime = std::str::from_utf8(&abr.updateDatetime).unwrap().to_string();

        let meta = ArrayMeta {
            devs: DeviceSet::new(nvm, data, spares),
            id: index,
            arrayName,
            metaRaidType,
            dataRaidType,
            createDatetime,
            updateDatetime,
            unique_id: abr.uniqueId
        };

        Ok(meta)
    }

    pub fn SaveAbr(&mut self, meta: &mut ArrayMeta) -> Result<(), PosEventId> {
        let (mut abr, index) = match self.mbrManager.GetAbr(&meta.arrayName) {
            Some((a, i)) => (a, i),
            None => {
                let eventId = PosEventId::MBR_ABR_NOT_FOUND;
                error!("[{}] Cannot save abr, abr not found {}", eventId.to_string(), meta.arrayName);
                return Err(eventId);
            }
        };

        let nvmNum = meta.devs.nvm.len();
        let dataNum = meta.devs.data.len();
        let spareNum = meta.devs.spares.len();

        abr.nvmDevNum = nvmNum as u32;
        abr.dataDevNum = dataNum as u32;
        abr.spareDevNum = spareNum as u32;
        abr.totalDevNum = (nvmNum + dataNum + spareNum) as u32;

        // TODO
        //CopyData(abr->arrayName, meta.arrayName, ARRAY_NAME_SIZE);
        //CopyData(abr->metaRaidType, meta.metaRaidType, META_RAID_TYPE_SIZE);
        //CopyData(abr->dataRaidType, meta.dataRaidType, DATA_RAID_TYPE_SIZE);
        //CopyData(abr->updateDatetime, Time::GetCurrentTimeStr("%Y-%m-%d %X %z", DATE_SIZE), DATE_SIZE);

        for i in 0..nvmNum {
            let deviceIndex = i as usize;
            abr.devInfo[deviceIndex].deviceType = ArrayDeviceType::NVM as u32;
            // CopyData(abr->devInfo[deviceIndex].deviceUid,
            //             meta.devs.nvm.at(i).uid, DEVICE_UID_SIZE);
        }

        for i in 0..dataNum {
            let deviceIndex = (nvmNum + i) as usize;
            abr.devInfo[deviceIndex].deviceType = ArrayDeviceType::DATA as u32;
            // CopyData(abr->devInfo[deviceIndex].deviceUid,
            //         meta.devs.data.at(i).uid, DEVICE_UID_SIZE);
            abr.devInfo[deviceIndex].deviceState = meta.devs.data[i].state.clone() as u32;
        }

        for i in 0..spareNum {
            let deviceIndex = (nvmNum + dataNum + i) as usize;
            abr.devInfo[deviceIndex].deviceType = ArrayDeviceType::SPARE as u32;
            // CopyData(abr->devInfo[deviceIndex].deviceUid,
            //         meta.devs.spares.at(i).uid, DEVICE_UID_SIZE);
        }

        meta.createDatetime = std::str::from_utf8(&abr.createDatetime).unwrap().to_string();
        meta.updateDatetime = std::str::from_utf8(&abr.updateDatetime).unwrap().to_string();

        self.mbrManager.UpdateDeviceIndexMap(&meta.arrayName)?;

        self.mbrManager.SaveMbr()
    }

    pub fn CreateAbr(&mut self, meta: ArrayMeta) -> Result<u32 /* array idx */, PosEventId> {
        self.mbrManager.CreateAbr(meta)
    }

    pub fn DeleteAbr(&self, name: &String) -> Result<(), PosEventId> {
        self.mbrManager.DeleteAbr(name)
    }

    pub fn ResetMbr(&mut self) -> Result<(), PosEventId> {
        self.mbrManager.ResetMbr()
    }

    pub fn GetAbrList(&mut self) -> Result<Vec::<ArrayBootRecord>, PosEventId> {
        self.mbrManager.LoadMbr()?;
        self.mbrManager.GetAbrList()
    }

    pub fn GetLastUpdatedDateTime(&self, arrayName: &String) -> Result<String, PosEventId> {
        let (abr, _) = match self.mbrManager.GetAbr(arrayName) {
            Some((a, i)) => (a, i),
            None => {
                let eventId = PosEventId::MBR_ABR_NOT_FOUND;
                warn!("[{}] Cannot get Abr for {}", eventId.to_string(), arrayName);
                return Err(eventId);
            }
        };

        let updateDatetime = std::str::from_utf8(&abr.updateDatetime).unwrap().to_string();
        Ok(updateDatetime)
    }

    pub fn GetCreatedDateTime(&self, arrayName: &String) -> Result<String, PosEventId> {
        let (abr, _) = match self.mbrManager.GetAbr(arrayName) {
            Some((a, i)) => (a, i),
            None => {
                let eventId = PosEventId::MBR_ABR_NOT_FOUND;
                warn!("[{}] Cannot get Abr for {}", eventId.to_string(), arrayName);
                return Err(eventId);
            }
        };

        let createDatetime = std::str::from_utf8(&abr.createDatetime).unwrap().to_string();
        Ok(createDatetime)
    }

    pub fn FindArrayWithDeviceSN(&self, devSN: String) -> String {
        self.mbrManager.FindArrayWithDeviceSN(devSN)
    }

    pub fn InitDisk(&self, /*dev: UblockSharedPtr*/) {
        todo!();
    }
}
