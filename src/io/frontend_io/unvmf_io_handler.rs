use std::os::raw::c_int;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use lazy_static::lazy_static;
use log::{error, info, warn};
use crate::bio::volume_io::VolumeIo;
use crate::event_scheduler::io_completer::IoCompleter;
use crate::generated::bindings::{IO_TYPE, IO_TYPE_ADMIN, IO_TYPE_FLUSH, IO_TYPE_READ, IO_TYPE_WRITE, pos_io, POS_IO_STATUS_FAIL, POS_IO_STATUS_SUCCESS};
use crate::include::io_error_type::IOErrorType::VOLUME_UMOUNTED;
use crate::include::pos_event_id::PosEventId;
use crate::include::pos_event_id::PosEventId::{BLKHDLR_WRONG_IO_DIRECTION, SCHEDAPI_NULL_COMMAND, SCHEDAPI_SUBMISSION_FAIL, SCHEDAPI_WRONG_BUFFER};
use crate::io::frontend_io::aio::AIO;
use crate::io::frontend_io::aio_submission_adapter::AioSubmissionAdapter;
use crate::qos::qos_manager::QosManagerSingleton;
use crate::spdk_wrapper::event_framework_api::EventFrameworkApiSingleton;
use crate::volume::volume_base::VolumeIoType;
use crate::volume::volume_service::VolumeServiceSingleton;

lazy_static!{
    static ref STOP_REACTOR: AtomicBool = {
        AtomicBool::new(false)
    };
}

pub extern "C" fn UNVMfCompleteHandler() {
    let aio = AIO;
    aio.CompleteIOs();
    let ret1 = EventFrameworkApiSingleton.CompleteEvents();
    let ret2 = EventFrameworkApiSingleton.CompleteSingleQueueEvents();
    if ret1 && ret2 {
        thread::sleep(Duration::from_micros(1));
    }
}

pub extern "C" fn UNVMfSubmitHandler(io: *mut pos_io) -> u32 {
    if io.is_null() {
        error!("[{}] Command from bdev is empty", SCHEDAPI_NULL_COMMAND.to_string());
        handle_exception(io);
    } else {
        let ioObj = unsafe {*io};
        let io_type = ioObj.ioType as IO_TYPE;
        if io_type > IO_TYPE_ADMIN {
            let aio = AIO;
            aio.SubmitAsyncAdmin(ioObj);
            return POS_IO_STATUS_SUCCESS;
        }
        match io_type {
            IO_TYPE_READ | IO_TYPE_WRITE => {
                if ioObj.iovcnt != 1 {
                    let event_id = SCHEDAPI_WRONG_BUFFER;
                    error!("[{}] Single IO command should have a continuous buffer", event_id.to_string());
                    handle_exception(io);
                    return POS_IO_STATUS_SUCCESS; // pos-cpp returns success even after catching an exception, so we do the same.
                }
            },
            IO_TYPE_FLUSH => {
                let aio = AIO;
                aio.SubmitFlush(ioObj);
                return POS_IO_STATUS_SUCCESS;
            },
            _ => {
                let event_id = BLKHDLR_WRONG_IO_DIRECTION;
                error!("[{}] Wrong IO direction (only read/write types are supported)", event_id.to_string());
                handle_exception(io);
                return POS_IO_STATUS_SUCCESS;
            },
        }

        let array_id = ioObj.array_id as i32;
        let volumeManager = VolumeServiceSingleton.GetVolumeManagerById(array_id);
        let vol_io_type : VolumeIoType = {
            match io_type {
                IO_TYPE_READ => VolumeIoType::UserRead,
                IO_TYPE_WRITE => VolumeIoType::UserWrite,
                IO_TYPE_FLUSH => VolumeIoType::InternalIo, // TODO: need to check
                _ => {
                    error!("Not supported io_type: {}", io_type);
                    VolumeIoType::MaxVolumeIoTypeCnt
                }
            }
        };
        let aio = AIO;
        let volumeIo = match aio.CreateVolumeIo(ioObj) {
            Ok(volume_io) => volume_io,
            Err(e) => {
                error!("[{}] failed to create volume I/O", e.to_string());
                return POS_IO_STATUS_SUCCESS; // SPDK에 단순히 요청을 처리했다 정도로 이해함 (성공이든/실패이든)
            }
        };

        if let Err(e) = volumeManager.IncreasePendingIOCountIfNotZero(ioObj.volume_id as i32, vol_io_type, 1 /*by default*/) {
            let ioCompleter = IoCompleter;
            ioCompleter.CompleteUbioWithoutRecovery(VOLUME_UMOUNTED, true);
            warn!("[{}] failed to increase pending io count if not zero...", e.to_string());
            return POS_IO_STATUS_SUCCESS;
        }

        if QosManagerSingleton.IsFeQosEnabled() {
            // Disabled at the moment. IsFeQosEnabled() always returns false.
            let aioSubmission = AioSubmissionAdapter;
            QosManagerSingleton.HandlePosIoSubmission(volumeIo);
        } else {
            aio.SubmitAsyncIO(volumeIo);
        }
    }

    // "io"가 null pointer로 온 경우, error인 경우, 성공인 경우 모두 항상 SUCCESS return 하는 것이 pos-cpp의 구현.
    return POS_IO_STATUS_SUCCESS;
}

fn handle_exception(io: *mut pos_io) {
    error!("[{}] Fail to submit pos IO", SCHEDAPI_SUBMISSION_FAIL.to_string());

    unsafe {
        if !io.is_null() && (*io).complete_cb.is_some() {
            let complete_cb = (*io).complete_cb.unwrap();
            complete_cb(io, POS_IO_STATUS_FAIL);
        }
    }
}

fn spin_up_completion_reactor() {
    info!("Spinning up a thread to poll on event completion...");
    thread::spawn(|| {
        loop {
            UNVMfCompleteHandler();
            thread::sleep(Duration::from_millis(1));
            if STOP_REACTOR.load(Ordering::Relaxed) {
                info!("Stopping a completion reactor...");
                break;
            }
        }
    });
}

fn stop_completion_reactor() {
    info!("Signalling stop to a completion reactor...");
    STOP_REACTOR.store(true, Ordering::Relaxed);
}

mod tests {
    use std::env::set_var;
    use std::os::raw::c_int;
    use std::ptr::{null, null_mut};
    use std::thread;
    use std::time::Duration;
    use crate::generated::bindings::{IO_TYPE_READ, IO_TYPE_WRITE, pos_io};
    use crate::io::frontend_io::unvmf_io_handler::{spin_up_completion_reactor, stop_completion_reactor, UNVMfSubmitHandler};
    use crate::spdk_wrapper::event_framework_api::EventFrameworkApiSingleton;

    #[test]
    fn test_single_writesubmission_to_completion() {
        // Given: a completion reactor thread running, and a write I/O
        setup_loglevel();
        spin_up_completion_reactor();
        let mut array_name = "posarray".to_string();
        let mut posIo = pos_io {
            ioType: IO_TYPE_WRITE as c_int,
            volume_id: 0,
            array_id: 0,
            iov: null_mut() /* not used at the moment */,
            iovcnt: 1,
            length: 0,
            offset: 0,
            context: null_mut(),
            arrayName: array_name.as_mut_ptr() as *mut ::std::os::raw::c_char,
            complete_cb: None
        };

        // When: we (as initiator) submit the I/O through UNVMfSubmitHandler
        let ret = UNVMfSubmitHandler(&mut posIo);
        let actual_queue_len = EventFrameworkApiSingleton.GetEventSingleQueueSize();
        assert_eq!(1, actual_queue_len);

        // Then: we should be able to see a single completion event
        thread::sleep(Duration::from_secs(1)); // enough time to handle the completion
        let actual_queue_len = EventFrameworkApiSingleton.GetEventSingleQueueSize();
        assert_eq!(0, actual_queue_len);

        // Cleanup
        stop_completion_reactor();
    }

    #[test]
    fn test_single_readsubmission_to_completion() {
        // Given: a completion reactor thread running, and a read I/O
        setup_loglevel();
        spin_up_completion_reactor();
        let mut array_name = "posarray".to_string();
        let mut posIo = pos_io {
            ioType: IO_TYPE_READ as c_int,
            volume_id: 0,
            array_id: 0,
            iov: null_mut() /* not used at the moment */,
            iovcnt: 1,
            length: 0,
            offset: 0,
            context: null_mut(),
            arrayName: array_name.as_mut_ptr() as *mut ::std::os::raw::c_char,
            complete_cb: None
        };

        // When: we (as initiator) submit the I/O through UNVMfSubmitHandler
        let ret = UNVMfSubmitHandler(&mut posIo);
        let actual_queue_len = EventFrameworkApiSingleton.GetEventSingleQueueSize();
        assert_eq!(1, actual_queue_len);

        // Then: we should be able to see a single completion event
        thread::sleep(Duration::from_secs(1)); // enough time to handle the completion
        let actual_queue_len = EventFrameworkApiSingleton.GetEventSingleQueueSize();
        assert_eq!(0, actual_queue_len);

        // Cleanup
        stop_completion_reactor();
    }

    #[test]
    fn test_five_writesubmissions_to_completion() {
        // Given: a completion reactor thread running, and five write I/Os
        setup_loglevel();
        let mut array_name = "posarray".to_string();
        let mut posIo1 = pos_io {
            ioType: IO_TYPE_WRITE as c_int,
            volume_id: 0,
            array_id: 0,
            iov: null_mut() /* not used at the moment */,
            iovcnt: 1,
            length: 0,
            offset: 0,
            context: null_mut(),
            arrayName: array_name.as_mut_ptr() as *mut ::std::os::raw::c_char,
            complete_cb: None
        };
        let mut posIo2 = posIo1.clone();
        posIo2.offset = 4096;
        let mut posIo3 = posIo1.clone();
        posIo3.offset = 65536;
        let mut posIo4 = posIo1.clone();
        posIo4.offset = 12000;
        let mut posIo5 = posIo1.clone();
        posIo5.offset = 1024;

        // When: we (as initiator) submit those five writes through UNVMfSubmitHandler
        let ret = UNVMfSubmitHandler(&mut posIo1);
        let ret = UNVMfSubmitHandler(&mut posIo2);
        let ret = UNVMfSubmitHandler(&mut posIo3);
        let ret = UNVMfSubmitHandler(&mut posIo4);
        let ret = UNVMfSubmitHandler(&mut posIo5);
        let actual_queue_len = EventFrameworkApiSingleton.GetEventSingleQueueSize();
        assert_eq!(5, actual_queue_len);
        // 일부러 submit 5개가 모두 끝난 시점에 spin_up_completion_reactor() 를 호출함.
        // 그래야 actual_queue_len이 기대하는 대로 5로 나올 수 있으므로.
        spin_up_completion_reactor();

        // Then: we should be able to see five completion events
        thread::sleep(Duration::from_secs(1)); // enough time to handle the completion
        let actual_queue_len = EventFrameworkApiSingleton.GetEventSingleQueueSize();
        assert_eq!(0, actual_queue_len);

        // Cleanup
        stop_completion_reactor();
    }

    fn setup_loglevel() {
        // set up the logger for the test context
        set_var("RUST_LOG", "DEBUG");
        env_logger::builder().is_test(true).try_init().unwrap();
    }
}