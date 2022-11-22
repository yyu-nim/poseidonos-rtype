use crate::event_scheduler::event::Event;
use crate::event_scheduler::event_scheduler::EventSchedulerSingleton;
use crate::include::backend_event::BackendEvent::BackendEvent_UserdataRebuild;

pub trait Callback : Event {
    fn Execute(&mut self) -> bool {
        let done = self._DoSpecificJob();
        if done {
            self._InvokeCallee();
            self._MarkExecutedDone();
        }

        return done;
    }

    fn _InvokeCallee(&mut self) {
        if let Some(mut callee) = self._TakeCallee() {
            let isOkToCall = callee._RecordCallerCompletionAndCheckOkToCall();
            if isOkToCall {
                self._PreCallExecuteCallee();
                let mut done = false;
                if self.GetEventType() != BackendEvent_UserdataRebuild {
                    done = Callback::Execute(&mut *callee);
                    if done {
                        return;
                    }
                }
            }

            //error[E0658]: trait upcasting coercion is experimental => EnqueueEvent 대신 EnqueueCall을 구현해야 할듯.
            //EventSchedulerSingleton.EnqueueEvent(callee);
            EventSchedulerSingleton.EnqueueCallback(callee);
        }
    }

    fn _RecordCallerCompletionAndCheckOkToCall(&self) -> bool {
        // TODO
        true
    }
    fn _PreCallExecuteCallee(&self) {
        // TODO
    }

    // pure function to implement
    fn _DoSpecificJob(&mut self) -> bool;

    // 아래 두개는, trait이 member variable을 못가지기 때문에, member getter/setter를
    // pure function으로 두어 개발자에게 맡기도록 함; abstract struct 같은게 있었으면 좋을듯;
    fn _TakeCallee(&mut self) -> Option<Box<dyn Callback>>;
    fn _MarkExecutedDone(&mut self);
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::sync::{Arc, Mutex};
    use crate::event_scheduler::callback::Callback;
    use crate::event_scheduler::event::Event;
    use crate::include::backend_event::BackendEvent;
    use crate::include::backend_event::BackendEvent::BackendEvent_Unknown;

    #[test]
    fn test_define_my_callback() {
        // Given: a chain of callbacks (callback1 -> callback2)
        struct MyCallback {
            callee: Option<Box<dyn Callback>>,
            executed: Arc<Mutex<bool>>,
            name: &'static str,
        };
        impl Event for MyCallback {
            fn GetEventType(&self) -> BackendEvent {
                BackendEvent_Unknown
            }

            fn Execute(&mut self) -> bool {
                todo!()
            }
        }
        impl Callback for MyCallback {
            fn _DoSpecificJob(&mut self) -> bool {
                println!("I'm doing something great: {}", self.name);
                true
            }

            fn _TakeCallee(&mut self) -> Option<Box<dyn Callback>> {
                self.callee.take()
            }

            fn _MarkExecutedDone(&mut self) {
                let mut executed = self.executed.lock().unwrap();
                *executed = true;
            }
        }

        let callback2_executed = Arc::new(Mutex::new(false));
        let callback2_executed_clone = callback2_executed.clone();
        let callback2 = MyCallback {
            callee: None,
            executed: callback2_executed_clone,
            name: "callback2"
        };
        let mut callback1 = MyCallback {
            callee: Some(Box::new(callback2)),
            executed: Arc::new(Mutex::new(false)),
            name: "callback1"
        };

        // When: callback1 is executed,
        Callback::Execute(&mut callback1);

        // Then: callback1 and callback2 should be executed
        assert_eq!(true, *callback1.executed.lock().unwrap());
        assert_eq!(true, *callback2_executed.lock().unwrap());
    }
}