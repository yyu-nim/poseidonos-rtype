use crate::state::state_context::StateContext;

pub trait IStateControl : Sync + Send {

    fn GetState(&self) -> StateContext;

}