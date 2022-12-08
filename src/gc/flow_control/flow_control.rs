use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

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

    pub fn GetToken(&self, fc_type: FlowControlType, token: i32) -> i32 {
        // TODO
        1
    }

    pub fn ReturnToken(&self, fc_type: FlowControlType, token: i32) {
        // TODO
    }

}

pub fn get() -> Arc<Mutex<FlowControl>> {
    return FlowControlSingleton.clone();
}