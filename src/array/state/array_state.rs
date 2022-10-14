use log::info;
use crate::include::array_state_type::ArrayStateEnum;
use crate::state::interface::i_state_control::IStateControl;

pub struct ArrayState;

impl ArrayState {
    pub fn new() -> ArrayState {
        info!("ArrayState has been created");
        ArrayState
    }

    pub fn boxed_interface() -> Box<dyn IStateControl> {
        let state = ArrayState::new();
        Box::new(state)
    }

    pub fn SetCreate(&self) {

    }

    fn _SetState(&self, newState: ArrayStateEnum) {
        // TODO
    }
}

impl IStateControl for ArrayState {

}