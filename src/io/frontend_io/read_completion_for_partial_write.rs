use crate::bio::volume_io::VolumeIo;
use crate::event_scheduler::callback::Callback;
use crate::event_scheduler::event::Event;
use crate::include::backend_event::BackendEvent;

pub struct ReadCompletionForPartialWrite;

impl ReadCompletionForPartialWrite {

    pub fn new(volume_io: &VolumeIo, alignment_size: u32, alignment_offset: u32) -> ReadCompletionForPartialWrite {
        ReadCompletionForPartialWrite {

        }
    }

    pub fn to_callback(self) -> Box<dyn Callback> {
        Box::new(self)
    }
}

impl Event for ReadCompletionForPartialWrite {
    fn GetEventType(&self) -> BackendEvent {
        todo!()
    }

    fn Execute(&mut self) -> bool {
        todo!()
    }
}

impl Callback for ReadCompletionForPartialWrite {
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