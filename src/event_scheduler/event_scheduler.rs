use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use crate::event_scheduler::callback::Callback;
use crate::event_scheduler::event::Event;
lazy_static!{
    pub static ref EventSchedulerSingleton : Arc<Mutex<EventScheduler>> = {
        Arc::new(Mutex::new(EventScheduler))
    };
}

pub struct EventScheduler;

impl EventScheduler {

    /***
     @schedulerCpuInput: e.g., [1, 3, 5]
     @eventCpuSetInput: e.g., [6, 7]
     */
    pub fn Initialize(&self, workerCountInput: u32, schedulerCpuInput: Vec<u8>, eventCpuSetInput: Vec<u8>)
    {
        // TODO
    }

    pub fn EnqueueEvent(&self, input: Box<dyn Event>) {
        // TODO
    }

    pub fn EnqueueCallback(&self, input: Box<dyn Callback>) {
        // TODO
    }
}