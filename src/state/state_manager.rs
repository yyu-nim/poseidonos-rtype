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

}