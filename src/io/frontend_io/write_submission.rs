use crate::bio::volume_io::VolumeIo;
use crate::event_scheduler::event::Event;
use crate::include::backend_event::BackendEvent;
use crate::include::backend_event::BackendEvent::BackendEvent_FrontendIO;

pub struct WriteSubmission {
    volume_io: VolumeIo,
}

impl WriteSubmission {
    pub fn new(volume_io: VolumeIo) -> WriteSubmission {
        WriteSubmission {
            volume_io
        }
    }
}

impl Event for WriteSubmission {
    fn GetEventType(&self) -> BackendEvent {
        BackendEvent_FrontendIO // TODO: need to check if it's right (vs. Unknown type)
    }

    fn Execute(&mut self) -> bool {
        // TODO: do something with volume_io
        true
    }
}