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

    fn AddIOWorker(&self) {
        // TODO
    }
}

pub fn RegisterRecoveryEventFactory(_recoveryEventFactory: IoRecoveryEventFactory) {
    // Not meaningful. Instead, use "recoveryEventFactory" as above.
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use std::sync::{Arc, Mutex};
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
        let mut write_ubio = Ubio::new(UbioDir::Write, 0, PATTERN.to_vec(), Box::new(|_| {}));
        write_ubio.uBlock = Some(ublock_device.boxed()); // TODO: move to constructor param

        IODispatcherSingleton.lock().unwrap().Submit(write_ubio, false, false);

        // When: we read a block where the pattern was written to
        let result_to_copy_to = Arc::new(Mutex::new(Vec::new()));
        let read_callback = {
            let result_to_copy_to = result_to_copy_to.clone();
            Box::new(move |read_buffer: &Vec<u8>| {
                // "FnMut" closure to capture "a result buffer" and modify it by pushing bytes
                // TODO: 사실 Arc, Mutex, move 를 쓰게되면서 FnMut을 쓴 의미가 퇴색되었고,
                // 대신 FnOnce closure로 더 제한하는게 맞아보이는데,
                // ubio 혹은 그 안에 담긴 member들을 move out하는 것이, callstack 여러 부분의 signature를 바꿔야 하는 것 같고,
                // 이 경우, original pos (cpp) 코드와 차이가 벌어지게 만들 수 있어서, 일단은 현재의 fn signature를
                // 유지하고자 Arc, Mutex, move를 써서 구현함. Callback.cpp 포팅/리팩토링이 끝난 이후 이 부분 다시 검토해 볼 것.
                let mut result_to_copy_to = result_to_copy_to.lock().unwrap();
                for &each_byte in read_buffer {
                    result_to_copy_to.push(each_byte);
                }
            })
        };
        let mut read_buffer = vec![0; PATTERN.len()];
        let mut read_ubio = Ubio::new(UbioDir::Read, 0, read_buffer, read_callback);
        read_ubio.uBlock = Some(ublock_device_cloned);
        IODispatcherSingleton.lock().unwrap().Submit(read_ubio, false, false);

        // Then: we should see the same pattern
        let actual = result_to_copy_to.lock().unwrap();
        assert_eq!(PATTERN.to_vec(), actual.to_vec());
    }
}