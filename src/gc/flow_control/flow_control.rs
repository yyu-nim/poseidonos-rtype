use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use crate::include::pos_event_id::PosEventId;

pub struct FlowControl;

lazy_static!{
    pub static ref FlowControlSingleton: Arc<Mutex<FlowControl>> = {
        Arc::new(Mutex::new(FlowControl::new()))
    };
}

pub enum FlowControlType {
    USER,
    GC,
}

impl FlowControl {

    pub fn new() -> FlowControl {
        FlowControl
    }

    pub fn GetToken(&self, fc_type: FlowControlType, token: i32) -> Result<i32, PosEventId> {
        // TODO
        Ok(1)
    }

    pub fn ReturnToken(&self, fc_type: FlowControlType, token: i32) -> Result<(), PosEventId>{
        // TODO
        Ok(())
    }

}

pub fn get() -> Arc<Mutex<FlowControl>> {
    return FlowControlSingleton.clone();
}