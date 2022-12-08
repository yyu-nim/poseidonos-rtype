use crate::state::include::situation_type::{SituationEnum, SituationType};
use crate::state::include::state_converter::Convert;
use crate::state::include::state_type::StateType;

pub struct StateContext {
    situation: SituationType,
}

impl StateContext {

    pub fn new(situation: SituationEnum) -> StateContext {
        StateContext {
            situation: SituationType::new(situation),
        }
    }

    pub fn ToStateType(&self) -> StateType {
        let state_enum = Convert(&self.situation.val);
        StateType::new(state_enum)
    }
}