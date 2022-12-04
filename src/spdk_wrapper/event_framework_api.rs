use std::sync::Mutex;
use crossbeam::queue::ArrayQueue;
use lazy_static::lazy_static;
lazy_static!{
    pub static ref EventFrameworkApiSingleton: EventFrameworkApi = {
        EventFrameworkApi::new()
    };
}

pub type EventClosure = Box<dyn FnOnce()->() + Send>;
const EVENT_QUEUE_SIZE: usize = 65536; // picked random val for now.
const MAX_PROCESSABLE_EVENTS: usize = 16;

pub struct EventFrameworkApi {
    eventSingleQueue: ArrayQueue<EventClosure>,
    eventQueues: ArrayQueue<EventClosure>,
}

impl EventFrameworkApi {
    pub fn new() -> EventFrameworkApi {
        EventFrameworkApi {
            eventSingleQueue: ArrayQueue::new(EVENT_QUEUE_SIZE),
            eventQueues: ArrayQueue::new(EVENT_QUEUE_SIZE),
            // pos-cpp에서는 reactor별로 eventQueue가 하나씩 있기 때문에 "s"를 붙임.
            // pos-rtype에서는 아직 reactor awareness가 없으므로 eventQueues 라는 이름의 1개 큐를 일단 사용.
        }
    }

    pub fn SendSpdkEvent(&self, closure: EventClosure) -> bool {
        self._SendEventToSingleQueue(closure);
        true
    }

    pub fn CompleteEvents(&self) -> bool {
        let mut events_processed = 0;
        loop {
            if let Some(event_closure) = self.eventQueues.pop() {
                event_closure();
            } else {
                break;
            }

            events_processed += 1;
            if events_processed >= MAX_PROCESSABLE_EVENTS {
                break;
            }
        }

        return self.eventQueues.is_empty();
    }

    pub fn CompleteSingleQueueEvents(&self) -> bool {
        let mut events_processed = 0;
        loop {
            if let Some(event_closure) = self.eventSingleQueue.pop() {
                event_closure();
            } else {
                break;
            }

            events_processed += 1;
            if events_processed >= MAX_PROCESSABLE_EVENTS {
                break;
            }
        }

        return self.eventSingleQueue.is_empty();
    }

    pub fn GetEventSingleQueueSize(&self) -> usize {
        self.eventSingleQueue.len()
    }

    pub fn GetEventQueuesSize(&self) -> usize {
        self.eventQueues.len()
    }

    fn _SendEventToSingleQueue(&self, closure: EventClosure) {
        // Note: NUMA isn't supported in pos-rtype yet
        self.eventSingleQueue.push(closure);
    }

    fn _SendEventToSpareQueue(&self, _core: u32, closure: EventClosure) {
        // Note: Per-core event queue isn't supported in pos-rtype yet
        self.eventQueues.push(closure);
    }

}