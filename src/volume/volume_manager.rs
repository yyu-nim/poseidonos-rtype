use std::sync::{Arc, Mutex};
use log::info;
use crate::array::array::Array;
use crate::state::interface::i_state_control::IStateControl;

pub struct VolumeManager {
    arrayName: String,
    arrayIndex: u32,
}

impl VolumeManager {

    pub fn new(array: &Array, _stateControl: &Box<dyn IStateControl>) -> VolumeManager {
        // TODO: _stateControl
        let arrayName = array.GetName();
        let arrayIndex = array.GetIndex();
        info!("Creating VolumeManager for {} with idx {}", arrayName, arrayIndex);
        VolumeManager {
            arrayName,
            arrayIndex,
        }
    }


}