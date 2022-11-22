use crate::bio::ubio::Ubio;
use crate::event_scheduler::event::Event;
use crate::event_scheduler::io_completer::IoCompleter;
use crate::include::backend_event::BackendEvent;

pub struct IoRecoveryEvent;

impl Event for IoRecoveryEvent {
    fn GetEventType(&self) -> BackendEvent {
        todo!()
    }

    fn Execute(&mut self) -> bool {
        todo!()
    }
}

impl IoRecoveryEvent {
    pub fn new(ubio: Ubio, ioCompleter: Option<IoCompleter>) -> IoRecoveryEvent {
        // TODO
        IoRecoveryEvent
    }
}