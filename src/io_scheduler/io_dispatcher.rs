use std::borrow::Borrow;
use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};
use log::{error, info};
use crate::bio::ubio::Ubio;
use crate::device::base::ublock_device::UBlockDevice;
use crate::device::i_io_dispatcher::IIODispatcher;
use crate::device::ufile::ufile_ssd::UfileSsd;
use crate::include::pos_event_id::PosEventId;
use crate::include::pos_event_id::PosEventId::{SUCCESS, UBIO_WITHOUT_UBLOCKDEV};
use crate::io::general_io::io_recovery_event_factory::IoRecoveryEventFactory;
use crate::io_scheduler::io_dispatcher_submission::{IODispatcherSubmission, IODispatcherSubmissionSingleton};

lazy_static!{
    pub static ref IODispatcherSingleton: Mutex<Box<dyn IIODispatcher>> = {
        let io_dispatcher = IODispatcher::new();
        Mutex::new(io_dispatcher)
    };

    static ref recoveryEventFactory: IoRecoveryEventFactory = {
        IoRecoveryEventFactory::new()
    };
}

#[derive(Clone)]
pub struct IODispatcher;

impl IODispatcher {
    pub fn new() -> Box<dyn IIODispatcher> {
        Box::new(IODispatcher)
    }
}

impl IIODispatcher for IODispatcher {
    fn Submit(&self, mut ubio: Arc<Mutex<Ubio>>, _sync: bool, _ioRecoveryNeeded: bool) -> PosEventId {
        let uBlock_cloned = {
            let ubio_locked = ubio.lock().unwrap();
            let uBlock;
            if let Some(u) = ubio_locked.uBlock.as_ref() {
                uBlock = u;
            } else {
                error!("The ubio does not have its UBlockDevice: {:?}. Cannot submit I/O", ubio_locked);
                return UBIO_WITHOUT_UBLOCKDEV;
            }
            uBlock.clone_box()
        };

        IODispatcherSubmissionSingleton.lock().unwrap().SubmitIO(
            uBlock_cloned, ubio.clone(), None);

        SUCCESS
    }

    fn AddIOWorker(&self) {
        // TODO
    }
}

pub fn RegisterRecoveryEventFactory(_recoveryEventFactory: IoRecoveryEventFactory) {
    // Not meaningful. Instead, use "recoveryEventFactory" as above.
}

pub fn CompleteForThreadLocalDeviceList() {
    // TODO
    // info!("CompleteForThreadLocalDeviceList() isn't supported yet...");
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod tests {
    use std::sync::{Arc, Mutex};
    use std::sync::mpsc::channel;
    use crossbeam::sync::{Parker, Unparker};
    use log::{debug, info};
    use crate::bio::ubio::{Ubio, UbioDir};
    use crate::device::base::ublock_device::UBlockDevice;
    use crate::device::ufile::ufile_ssd::UfileSsd;
    use crate::event_scheduler::callback::{Callback, tests};
    use crate::event_scheduler::event::Event;
    use crate::include::backend_event::BackendEvent;
    use crate::io_scheduler::io_dispatcher::IODispatcherSingleton;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    pub struct WaitReadDone {
        pub unparker: Unparker,
        pub done: bool,
    }
    impl Event for WaitReadDone {
        fn GetEventType(&self) -> BackendEvent { todo!() }

        fn Execute(&mut self) -> bool { todo!() }
    }
    impl Callback for WaitReadDone {
        fn _DoSpecificJob(&mut self) -> bool {
            self.unparker.unpark();
            true
        }

        fn _TakeCallee(&mut self) -> Option<Box<dyn Callback>> {
            None
        }

        fn _MarkExecutedDone(&mut self) {
            self.done = true;
        }
    }

    #[test]
    fn test_submit_io_to_ufile_ssd() {
        init();

        // Given: a device written with a specific pattern
        let mut ublock_device = UfileSsd::new("/tmp/dev1".into(), 1024*1024);
        ublock_device.Open();
        let ublock_device_cloned = ublock_device.clone_box(); // arc<mutex>로 구현

        let PATTERN = b"CerTAinUnIquePATteRn";
        let write_buffer = Arc::new(Mutex::new(PATTERN.to_vec()));
        let mut write_ubio = Ubio::new(Some(write_buffer), None /* no callback */, 0);
        write_ubio.lba = 0;
        write_ubio.dir = UbioDir::Write;
        write_ubio.uBlock = Some(ublock_device.boxed());
        let write_ubio = Arc::new(Mutex::new(write_ubio));
        IODispatcherSingleton.lock().unwrap().Submit(write_ubio, false, false);

        // When: we read a block where the pattern was written to
        let parker = Parker::new();
        let unparker = parker.unparker().clone();
        let mut read_buffer = Arc::new(Mutex::new(vec![0; PATTERN.len()]));
        let read_callback: Box<dyn Callback> = Box::new(
            WaitReadDone {
                unparker: unparker,
                done: false
            }
        );
        let mut read_ubio = Ubio::new(Some(read_buffer.clone()), Some(read_callback), 0);
        read_ubio.lba = 0;
        read_ubio.dir = UbioDir::Read;
        read_ubio.uBlock = Some(ublock_device_cloned);
        let read_ubio = Arc::new(Mutex::new(read_ubio));
        IODispatcherSingleton.lock().unwrap().Submit(read_ubio, false, false);
        parker.park(); // read i/o가 끝날때까지 synchronous하게 기다려 주는 역할.

        // Then: we should see the same pattern
        let actual = read_buffer.lock().unwrap();
        assert_eq!(PATTERN.to_vec(), actual.to_vec());
    }
}