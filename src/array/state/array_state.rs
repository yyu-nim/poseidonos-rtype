use log::info;
use crate::include::array_state_type::ArrayStateEnum;
use crate::state::include::state_type::StateEnum;
use crate::state::interface::i_state_control::IStateControl;
use crate::state::state_context::StateContext;

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
    fn GetState(&self) -> StateContext {
        todo!()
    }

    fn GetStateEnum(&self) -> StateEnum {
        todo!()
    }
}