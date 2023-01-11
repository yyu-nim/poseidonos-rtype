use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use crate::array::state::array_state::ArrayState;
use crate::state::interface::i_state_control::IStateControl;

pub struct StateManager;

lazy_static!{
    pub static ref StateManagerSingleton: Arc<Mutex<StateManager>> = {
        Arc::new(Mutex::new(StateManager::new()))
    };
}

impl StateManager {

    pub fn new() -> StateManager {
        StateManager
    }

    pub fn CreateStateControl(&self, array: String) -> Box<dyn IStateControl> {
        // TODO
        ArrayState::boxed_interface()
    }

    pub fn GetStateControl(&self, array_id: u32/*array_name: String*/) -> Box<dyn IStateControl> {
        // TODO: pos-cpp는 array_name을 받도록 되어 있으나, 통합 편의상 현재는 array_id를 쓰도록 하고, 추후 array_name으로 변경
        ArrayState::boxed_interface()
    }

}