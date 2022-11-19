use std::os::raw::c_int;
use log::{error, warn};
use crate::event_scheduler::io_completer::IoCompleter;
use crate::generated::bindings::{IO_TYPE, IO_TYPE_ADMIN, IO_TYPE_FLUSH, IO_TYPE_READ, IO_TYPE_WRITE, pos_io, POS_IO_STATUS_FAIL, POS_IO_STATUS_SUCCESS};
use crate::include::io_error_type::IOErrorType::VOLUME_UMOUNTED;
use crate::include::pos_event_id::PosEventId::{BLKHDLR_WRONG_IO_DIRECTION, SCHEDAPI_NULL_COMMAND, SCHEDAPI_SUBMISSION_FAIL, SCHEDAPI_WRONG_BUFFER};
use crate::io::frontend_io::aio::AIO;
use crate::io::frontend_io::aio_submission_adapter::AioSubmissionAdapter;
use crate::qos::qos_manager::QosManagerSingleton;
use crate::volume::volume_base::VolumeIoType;
use crate::volume::volume_service::VolumeServiceSingleton;

pub extern "C" fn UNVMfCompleteHandler() {
    // TODO
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
        let volumeIo = aio.CreateVolumeIo(ioObj);

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