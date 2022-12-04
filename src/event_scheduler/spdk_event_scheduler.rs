use log::error;
use crate::event_scheduler::event::Event;
use crate::include::pos_event_id::PosEventId::EVENTFRAMEWORK_INVALID_EVENT;
use crate::spdk_wrapper::event_framework_api::{EventFrameworkApi, EventFrameworkApiSingleton};

pub fn ExecuteOrScheduleEvent(_core: Option<u32>, mut event: Box<dyn Event>) {
    let done = event.Execute();
    if !done {
        SendSpdkEvent(Some(event));
    }
}

pub fn SendSpdkEvent(event: Option<Box<dyn Event>>) -> bool {
    if let Some(event) = event {
        let event_closure = Box::new(||{
            ExecuteOrScheduleEvent(None, event);
        });
        EventFrameworkApiSingleton.SendSpdkEvent(event_closure);
        return true;
    } else {
        let event_id = EVENTFRAMEWORK_INVALID_EVENT;
        error!("[{}] Invalid Event to send", event_id.to_string());
        return false;
    }

}