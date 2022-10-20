use std::borrow::Borrow;
use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};
use log::error;
use crate::bio::ubio::Ubio;
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

        let uBlock_cloned = Box::clone(&uBlock);
        IODispatcherSubmissionSingleton.lock().unwrap().SubmitIO(
            uBlock_cloned, ubio, None);

        SUCCESS
    }
}

pub fn RegisterRecoveryEventFactory(_recoveryEventFactory: IoRecoveryEventFactory) {
    // Not meaningful. Instead, use "recoveryEventFactory" as above.
}

#[cfg(test)]
mod tests {
    use crate::bio::ubio::{Ubio, UbioDir};
    use crate::device::base::ublock_device::UBlockDevice;
    use crate::device::ufile::ufile_ssd::UfileSsd;
    use crate::io_scheduler::io_dispatcher::IODispatcherSingleton;

    #[test]
    fn test_submit_io_to_ufile_ssd() {
        let mut ublock_device = UfileSsd::new("/tmp/dev1".into(), 1024*1024);
        ublock_device.Open();

        let PATTERN = b"CerTAinUnIquePATteRn";
        let mut ubio = Ubio::new(UbioDir::Write, 0, PATTERN.to_vec());
        ubio.uBlock = Some(ublock_device.boxed());

        IODispatcherSingleton.lock().unwrap().Submit(ubio, false, false);

        //ublock_device.Close();
    }
}