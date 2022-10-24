use crate::include::backend_event::BackendEvent;

pub trait Event {
    fn GetEventType(&self) -> BackendEvent;
}

pub struct EventImpl;
impl Event for EventImpl {
    fn GetEventType(&self) -> BackendEvent {
        todo!("need to implement")
    }
}