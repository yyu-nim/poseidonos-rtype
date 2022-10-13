use log::info;
use crate::array::array::Array;
use crate::array::device::array_device_manager::ArrayDeviceManager;
use crate::array::interface::i_abr_control::IAbrControl;
use crate::array::meta::array_meta::ArrayMeta;
use crate::array::partition::partition_manager::PartitionManager;
use crate::array::state::array_state::ArrayState;
use crate::array_models::dto::device_set::DeviceSet;

pub struct ArrayComponents {
    array: Array,
}

impl ArrayComponents {

    pub fn new() -> ArrayComponents {
        struct MockAbrControl;
        impl IAbrControl for MockAbrControl {
            fn CreateAbr(&self, meta: ArrayMeta) -> i32 {
                // TODO
                0
            }

            fn DeleteAbr(&self, arrayName: String) -> i32 {
                // TODO
                0
            }

            fn LoadAbr(&self, meta: ArrayMeta) -> i32 {
                // TODO
                0
            }

            fn SaveAbr(&self, meta: ArrayMeta) -> i32 {
                // TODO
                0
            }

            fn ResetMbr(&self) -> i32 {
                // TODO
                0
            }

            fn FindArrayWithDeviceSN(&self, devSN: String) -> String {
                // TODO
                "TODO".to_string()
            }

            fn GetLastUpdatedDateTime(&self, arrayName: String) -> String {
                // TODO
                "TODO".to_string()
            }

            fn GetCreatedDateTime(&self, arrayNameL: String) -> String {
                // TODO
                "TODO".to_string()
            }
        }

        let boxed = Box::new(MockAbrControl);
        ArrayComponents {
            array: Array::new("test".to_string(), ArrayDeviceManager, boxed, PartitionManager, ArrayState),
        }
    }

    pub fn Create(&self, name: String, devs: DeviceSet<String>,
               metaFt: String, dataFt: String) {
        // TODO
        info!("[CREATE_ARRAY_DEBUG_MSG] Creating array component for {}", name);
        self.array.Create(devs, metaFt, dataFt);

        self._InstantiateMetaComponentsAndMountSequenceInOrder(false/* array has not been loaded yet*/);
        self._SetMountSequence();
    }

    fn _InstantiateMetaComponentsAndMountSequenceInOrder(&self, is_array_loaded: bool) {
        // TODO
    }

    fn _SetMountSequence(&self) {
        // TODO
    }

}