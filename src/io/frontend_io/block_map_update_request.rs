use crate::bio::volume_io::VolumeIo;
use crate::event_scheduler::callback::Callback;
use crate::event_scheduler::event::Event;
use crate::include::backend_event::BackendEvent;

pub struct BlockMapUpdateRequest {
    //volume_io: &'a VolumeIo,
    //origin_callback: Option<Box<dyn Callback>>,
}

impl Event for BlockMapUpdateRequest {
    fn GetEventType(&self) -> BackendEvent {
        todo!()
    }

    fn Execute(&mut self) -> bool {
        todo!()
    }
}

impl Callback for BlockMapUpdateRequest {
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

impl BlockMapUpdateRequest {
    pub fn new(volume_io: &VolumeIo) -> BlockMapUpdateRequest {
        BlockMapUpdateRequest {
            /*volume_io,
            origin_callback,*/
        }
    }

    pub fn to_callback(self) -> Box<dyn Callback> {
        Box::new(self)
    }
}