use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};
use crate::device::i_io_dispatcher::IIODispatcher;
use crate::io::general_io::io_recovery_event_factory::IoRecoveryEventFactory;
lazy_static!{
    pub static ref IODispatcherSingleton: Mutex<Box<dyn IIODispatcher>> = {
        let io_dispatcher = IODispatcher::new();
        Mutex::new(io_dispatcher)
    };

    static ref recoveryEventFactory: IoRecoveryEventFactory = {
        IoRecoveryEventFactory::new()
    };
}

#[derive(Clone)]
pub struct IODispatcher;

impl IODispatcher {
    pub fn new() -> Box<dyn IIODispatcher> {
        Box::new(IODispatcher)
    }
}

impl IIODispatcher for IODispatcher {

}

pub fn RegisterRecoveryEventFactory(_recoveryEventFactory: IoRecoveryEventFactory) {
    // Not meaningful. Instead, use "recoveryEventFactory" as above.
}