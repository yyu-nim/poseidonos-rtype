use crate::bio::ubio::Ubio;
use crate::event_scheduler::event::Event;
use crate::event_scheduler::io_completer::IoCompleter;

pub struct IoRecoveryEvent;

impl Event for IoRecoveryEvent {

}

impl IoRecoveryEvent {
    pub fn new(ubio: Ubio, ioCompleter: Option<IoCompleter>) -> IoRecoveryEvent {
        // TODO
        IoRecoveryEvent
    }
}