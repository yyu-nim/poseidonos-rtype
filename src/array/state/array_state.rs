use log::info;
use crate::include::array_state_type::ArrayStateEnum;

pub struct ArrayState;

impl ArrayState {
    pub fn new() -> ArrayState {
        info!("ArrayState has been created");
        ArrayState
    }

    pub fn SetCreate(&self) {

    }

    fn _SetState(&self, newState: ArrayStateEnum) {
        // TODO
    }
}