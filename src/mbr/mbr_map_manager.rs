use crate::array::meta::array_meta::ArrayMeta;
use crate::array::meta::device_meta::DeviceMeta;
use crate::include::pos_event_id;
use crate::include::pos_event_id::PosEventId;
use log::{debug, warn};
use std::collections::HashMap;

#[derive(Default)]
pub struct MbrMapManager {
    arrayDeviceIndexMap: HashMap<String, u32>,
}

impl MbrMapManager {
    pub fn InsertDevices(&mut self, meta: &ArrayMeta, arrayIndex: u32) -> Result<(), PosEventId> {
        let mut insertNum = 0;

        let new_dev_uids = meta
            .devs
            .nvm
            .iter()
            .map(|dev| dev.uid.to_string())
            .collect();
        insertNum += self.add_to_array_device_index_map(new_dev_uids, arrayIndex);

        let new_dev_uids = meta
            .devs
            .data
            .iter()
            .map(|dev| dev.uid.to_string())
            .collect();
        insertNum += self.add_to_array_device_index_map(new_dev_uids, arrayIndex);

        let new_dev_uids = meta
            .devs
            .spares
            .iter()
            .map(|dev| dev.uid.to_string())
            .collect();
        insertNum += self.add_to_array_device_index_map(new_dev_uids, arrayIndex);

        debug!(
            "[{}] Inserted {} devices to arrayDeviceMap",
            PosEventId::MBR_DEBUG_MSG.to_string(),
            insertNum
        );

        Ok(())
    }

    fn add_to_array_device_index_map(&mut self, uids: Vec<String>, arrayIndex: u32) -> u32 {
        let mut insertNum = 0;
        uids.iter().for_each(|uid| {
            self.arrayDeviceIndexMap.insert(uid.to_owned(), arrayIndex);

            debug!(
                "[{}] Inserted {} to array {}",
                PosEventId::MBR_DEBUG_MSG.to_string(),
                uid,
                arrayIndex
            );
            insertNum += 1;
        });

        insertNum
    }

    pub fn InsertDevice(&mut self, deviceUid: &String, arrayIndex: u32) -> Result<(), PosEventId> {
        self.arrayDeviceIndexMap
            .insert(deviceUid.to_string(), arrayIndex);
        Ok(())
    }

    pub fn DeleteDevices(&mut self, arrayIndex: u32) -> Result<(), PosEventId> {
        let prev_hashmap_size = self.arrayDeviceIndexMap.len();
        self.arrayDeviceIndexMap.retain(|_, v| *v != arrayIndex);
        let after_hashmap_size = self.arrayDeviceIndexMap.len();

        debug!(
            "[{}] Deleted {} devices from arrayDeviceMap",
            PosEventId::MBR_DEBUG_MSG.to_string(),
            after_hashmap_size - prev_hashmap_size
        );

        Ok(())
    }

    pub fn CheckAllDevices(&self, meta: &ArrayMeta) -> Result<(), PosEventId> {
        self._CheckDevices(&meta.devs.nvm)?;
        self._CheckDevices(&meta.devs.data)?;
        self._CheckDevices(&meta.devs.spares)?;

        Ok(())
    }

    fn _CheckDevices(&self, devs: &Vec<DeviceMeta>) -> Result<(), PosEventId> {
        for dev in devs {
            if let Some(arrayIdx) = self.arrayDeviceIndexMap.get(&dev.uid.to_string()) {
                let eventId = PosEventId::MBR_DEVICE_ALREADY_IN_ARRAY;
                warn!(
                    "[{}] device_uid: {}, array: {}",
                    eventId.to_string(),
                    dev.uid,
                    arrayIdx
                );
                return Err(eventId);
            }
        }

        Ok(())
    }

    pub fn ResetMap(&mut self) {
        self.arrayDeviceIndexMap.clear();
    }

    pub fn FindArrayIndex(&self, devSN: &String) -> Result<u32, PosEventId> {
        if let Some(arrayIndex) = self.arrayDeviceIndexMap.get(devSN) {
            return Ok(*arrayIndex);
        }
        Err(PosEventId::ERROR)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::array_models::dto::device_set::DeviceSet;
    use crate::include::array_device_state::ArrayDeviceState;

    #[test]
    fn test_insert_and_reset_map() {
        let devs = DeviceSet {
            nvm: vec![DeviceMeta {
                uid: "nvm_uid".to_string(),
                state: ArrayDeviceState::NORMAL,
            }],
            data: vec![
                DeviceMeta {
                    uid: "data_0".to_string(),
                    state: ArrayDeviceState::NORMAL,
                },
                DeviceMeta {
                    uid: "data_1".to_string(),
                    state: ArrayDeviceState::NORMAL,
                },
            ],
            spares: vec![DeviceMeta {
                uid: "spare_0".to_string(),
                state: ArrayDeviceState::NORMAL,
            }],
        };
        let meta = ArrayMeta {
            devs,
            id: 0,
            arrayName: "test_array".to_string(),
            metaRaidType: "RAID0".to_string(),
            dataRaidType: "RAID5".to_string(),
            createDatetime: "".to_string(),
            updateDatetime: "".to_string(),
            unique_id: 0,
        };

        let arrayIndex = 1;
        let mut mbrMapManager = MbrMapManager::default();
        assert!(mbrMapManager.InsertDevices(&meta, arrayIndex).is_ok());

        assert_eq!(
            mbrMapManager.FindArrayIndex(&meta.devs.nvm[0].uid),
            Ok(arrayIndex)
        );
        assert_eq!(
            mbrMapManager.FindArrayIndex(&meta.devs.data[0].uid),
            Ok(arrayIndex)
        );
        assert_eq!(
            mbrMapManager.FindArrayIndex(&meta.devs.data[1].uid),
            Ok(arrayIndex)
        );
        assert_eq!(
            mbrMapManager.FindArrayIndex(&meta.devs.spares[0].uid),
            Ok(arrayIndex)
        );

        assert_eq!(mbrMapManager.CheckAllDevices(&meta).is_err(), true);

        mbrMapManager.ResetMap();
        assert_eq!(mbrMapManager.CheckAllDevices(&meta).is_ok(), true);
    }
}
