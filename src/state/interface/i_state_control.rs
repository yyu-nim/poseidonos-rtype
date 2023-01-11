use crate::state::include::state_type::StateEnum;
use crate::state::state_context::StateContext;

pub trait IStateControl : Sync + Send {

    fn GetState(&self) -> StateContext;
    fn GetStateEnum(&self) -> StateEnum; // available at pos-rtype only

}