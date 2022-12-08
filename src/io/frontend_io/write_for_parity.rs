use crate::bio::volume_io::VolumeIo;
use crate::event_scheduler::callback::Callback;
use crate::event_scheduler::event::Event;
use crate::include::backend_event::BackendEvent;

pub struct WriteForParity;

impl WriteForParity {
    pub fn new(volume_io: &VolumeIo, is_frontend: bool) -> WriteForParity {
        WriteForParity {

        }
    }
}

impl Event for WriteForParity {
    fn GetEventType(&self) -> BackendEvent {
        todo!()
    }

    fn Execute(&mut self) -> bool {
        todo!()
    }
}

impl Callback for WriteForParity {
    fn _DoSpecificJob(&mut self) -> bool {
        todo!()
    }

    fn _TakeCallee(&mut self) -> Option<Box<dyn Callback>> {
        todo!()
    }

    fn _MarkExecutedDone(&mut self) {
        todo!()
    }
}