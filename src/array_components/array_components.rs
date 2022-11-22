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
use crate::network::nvmf::Nvmf;
use crate::state::interface::i_state_control::IStateControl;
use crate::state::state_manager::{StateManager, StateManagerSingleton};
use crate::volume::volume_manager::VolumeManager;
use crate::include::pos_event_id::PosEventId;
use crate::metadata::metadata::Metadata;

pub struct ArrayComponents {
    array: Array,
    metafs: Option<MetaFs>,
    stateMgr: Arc<Mutex<StateManager>>,
    state: Box<dyn IStateControl>,
    volMgr: Option<VolumeManager>,
    nvmf: Option<Nvmf>,
    meta: Option<Metadata>,
}

impl ArrayComponents {

    pub fn new() -> ArrayComponents {
        struct MockAbrControl;
        impl IAbrControl for MockAbrControl {
            fn CreateAbr(&self, meta: ArrayMeta) -> Result<u32, PosEventId> {
                // TODO
                Ok(0)
            }

            fn DeleteAbr(&self, arrayName: String) -> Result<(), PosEventId> {
                // TODO
                Ok(())
            }

            fn LoadAbr(&self, meta: ArrayMeta) -> Result<(), PosEventId> {
                // TODO
                Ok(())
            }

            fn SaveAbr(&self, meta: ArrayMeta) -> Result<(), PosEventId> {
                // TODO
                Ok(())
            }

            fn ResetMbr(&self) -> Result<(), PosEventId> {
                // TODO
                Ok(())
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
            array: Array::new(ArrayDeviceManager, boxed, PartitionManager, ArrayState),
            metafs: None,
            stateMgr: state_manager,
            state: state,
            volMgr: None,
            nvmf: None,
            meta: None,
        }
    }

    pub fn Create(&mut self, name: String, devs: DeviceSet<String>,
                  metaFt: String, dataFt: String) -> Result<(), PosEventId> {
        // TODO
        info!("[CREATE_ARRAY_DEBUG_MSG] Creating array component for {}", name);
        self.array.Create(name, devs, metaFt, dataFt)?;

        self._InstantiateMetaComponentsAndMountSequenceInOrder(false/* array has not been loaded yet*/);
        self._SetMountSequence();

        Ok(())
    }

    pub fn Delete(&mut self) -> Result<(), PosEventId> {
        self.array.Delete();
        Ok(())
    }

    fn _InstantiateMetaComponentsAndMountSequenceInOrder(&mut self, is_array_loaded: bool) {
        // TODO
        self.metafs = Some(MetaFs::new(&self.array, is_array_loaded));
        self.volMgr = Some(VolumeManager::new(&self.array, &self.state));
        self.nvmf = Some(Nvmf::new(self.array.GetArrayInfo().clone()));
        self.meta = Some(Metadata::new(self.array.GetArrayInfo().clone(), &self.state));
    }

    fn _SetMountSequence(&self) {
        // TODO
    }

}