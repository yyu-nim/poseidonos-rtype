use crate::bio::ubio::Ubio;
use crate::event_scheduler::event::Event;
use crate::io::general_io::io_recovery_event::IoRecoveryEvent;

pub struct IoRecoveryEventFactory;

impl IoRecoveryEventFactory {

    pub fn new() -> IoRecoveryEventFactory {
        IoRecoveryEventFactory
    }

    pub fn Create(ubio: Ubio) -> Box<dyn Event> {
        let io_recovery_event = IoRecoveryEvent::new(ubio, None);
        let event = Box::new(io_recovery_event);
        event
    }

}