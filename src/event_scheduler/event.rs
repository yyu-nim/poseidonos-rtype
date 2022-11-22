use crate::include::backend_event::BackendEvent;

pub trait Event : Send {
    fn GetEventType(&self) -> BackendEvent;
    fn Execute(&mut self) -> bool;
}

pub struct EventImpl;
impl Event for EventImpl {
    fn GetEventType(&self) -> BackendEvent {
        todo!("need to implement")
    }

    fn Execute(&mut self) -> bool {
        todo!()
    }
}