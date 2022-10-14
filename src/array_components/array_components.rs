use std::sync::{Arc, Mutex};
use log::info;
use crate::array::array::Array;
use crate::array::device::array_device_manager::ArrayDeviceManager;
use crate::array::interface::i_abr_control::IAbrControl;
use crate::array::meta::array_meta::ArrayMeta;
use crate::array::partition::partition_manager::PartitionManager;
use crate::array::state::array_state::ArrayState;
use crate::array_models::dto::device_set::DeviceSet;
use crate::metafs::metafs::MetaFs;
use crate::state::interface::i_state_control::IStateControl;
use crate::state::state_manager::{StateManager, StateManagerSingleton};

pub struct ArrayComponents {
    array: Array,
    metafs: Option<MetaFs>,
    stateMgr: Arc<Mutex<StateManager>>,
    state: Box<dyn IStateControl>,
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

        let array_name = "POSArray"; // TODO
        let boxed = Box::new(MockAbrControl);
        let state_manager = StateManagerSingleton.clone();
        let state = state_manager.lock().unwrap().CreateStateControl(array_name.to_string());
        ArrayComponents {
            array: Array::new(array_name.to_string(), ArrayDeviceManager, boxed, PartitionManager, ArrayState),
            metafs: None,
            stateMgr: state_manager,
            state: state,
        }
    }

    pub fn Create(&mut self, name: String, devs: DeviceSet<String>,
                  metaFt: String, dataFt: String) {
        // TODO
        info!("[CREATE_ARRAY_DEBUG_MSG] Creating array component for {}", name);
        self.array.Create(devs, metaFt, dataFt);

        self._InstantiateMetaComponentsAndMountSequenceInOrder(false/* array has not been loaded yet*/);
        self._SetMountSequence();
    }

    fn _InstantiateMetaComponentsAndMountSequenceInOrder(&mut self, is_array_loaded: bool) {
        // TODO
        self.metafs = Some(MetaFs::new(&self.array, is_array_loaded));


    }

    fn _SetMountSequence(&self) {
        // TODO
    }

}