use std::borrow::Borrow;
use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};
use log::error;
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
    fn Submit(&self, mut ubio: Ubio, _sync: bool, _ioRecoveryNeeded: bool) -> PosEventId {

        let uBlock;
        if let Some(u) = ubio.uBlock.as_ref() {
            uBlock = u;
        } else {
            error!("The ubio does not have its UBlockDevice: {:?}. Cannot submit I/O", ubio);
            return UBIO_WITHOUT_UBLOCKDEV;
        }

        let uBlock_cloned = uBlock.clone_box();
        IODispatcherSubmissionSingleton.lock().unwrap().SubmitIO(
            uBlock_cloned, ubio, None);

        SUCCESS
    }
}

pub fn RegisterRecoveryEventFactory(_recoveryEventFactory: IoRecoveryEventFactory) {
    // Not meaningful. Instead, use "recoveryEventFactory" as above.
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use std::sync::mpsc::channel;
    use log::{debug, info};
    use crate::bio::ubio::{Ubio, UbioDir};
    use crate::device::base::ublock_device::UBlockDevice;
    use crate::device::ufile::ufile_ssd::UfileSsd;
    use crate::io_scheduler::io_dispatcher::IODispatcherSingleton;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_submit_io_to_ufile_ssd() {
        init();

        // Given: a device written with a specific pattern
        let mut ublock_device = UfileSsd::new("/tmp/dev1".into(), 1024*1024);
        ublock_device.Open();
        let mut ublock_device_cloned = ublock_device.clone_box(); // TODO: clone vs. arc<mutex> 고민해볼 것.

        let PATTERN = b"CerTAinUnIquePATteRn";
        let mut write_ubio = Ubio::new(UbioDir::Write, 0, PATTERN.to_vec());
        write_ubio.uBlock = Some(ublock_device.boxed()); // TODO: move to constructor param

        IODispatcherSingleton.lock().unwrap().Submit(write_ubio, false, false);

        // When: we read a block where the pattern was written to
        let mut dataBuffer = vec![0; PATTERN.len()];
        let mut read_ubio = Ubio::new(UbioDir::Read, 0, dataBuffer);
        let (tx, rx) = channel::<Vec<u8>>();
        read_ubio.uBlock = Some(ublock_device_cloned);
        read_ubio.callback_tx = Some(tx);
        read_ubio.callback = Some(move |tx, data| {
            // TODO: callback 이라기 보다는 completion handler에 가까운데, 일단 POC로 동작 검증. 사용성에 대해서는 추가 고민 필요할듯.
            debug!("read callback is invoked");
            tx.send(data.to_vec()); // TODO: 복사가 없이 sender에게 보낼 수 있으면 좋을 듯.
        });
        IODispatcherSingleton.lock().unwrap().Submit(read_ubio, false, false);

        // Then: we should see the same pattern
        let actual = rx.recv();
        assert!(actual.is_ok());
        let actual = actual.unwrap();

        assert_eq!(PATTERN.to_vec(), actual);
    }
}