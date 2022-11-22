use crate::bio::volume_io::VolumeIo;
use crate::event_scheduler::event::Event;
use crate::include::backend_event::BackendEvent;
use crate::include::backend_event::BackendEvent::BackendEvent_FrontendIO;

pub struct ReadSubmission {
    volume_io: VolumeIo,
}

impl ReadSubmission {
    pub fn new(volume_io: VolumeIo) -> ReadSubmission {
        ReadSubmission {
            volume_io
        }
    }
}

impl Event for ReadSubmission {
    fn GetEventType(&self) -> BackendEvent {
        BackendEvent_FrontendIO
    }

    fn Execute(&mut self) -> bool {
        // TODO: do something with volume_io
        true
    }
}