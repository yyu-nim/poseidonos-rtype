use std::os::raw::c_int;
use std::ptr::{null, null_mut};
use std::sync::{Arc, Mutex};
use log::{error, info};
use crate::bio::ubio::UbioDir;
use crate::bio::volume_io::VolumeIo;
use crate::event_scheduler::callback::Callback;
use crate::event_scheduler::event::Event;
use crate::event_scheduler::spdk_event_scheduler;
use crate::generated::bindings::{IO_TYPE_FLUSH, pos_io, POS_IO_STATUS_SUCCESS};
use crate::include::backend_event::BackendEvent;
use crate::include::backend_event::BackendEvent::BackendEvent_FrontendIO;
use crate::include::memory::ChangeByteToSector;
use crate::include::pos_event_id::PosEventId;
use crate::include::pos_event_id::PosEventId::{AIO_FLUSH_END, BLKHDLR_WRONG_IO_DIRECTION};
use crate::io::frontend_io::read_submission::ReadSubmission;
use crate::io::frontend_io::write_submission::WriteSubmission;
use crate::io_scheduler::io_dispatcher;
use crate::io_scheduler::io_dispatcher::IODispatcher;
use crate::volume::volume_base::VolumeIoType;

pub struct AIO;

impl AIO {
    pub fn SubmitAsyncIO(&self, volumeIo: VolumeIo) {
        match volumeIo.dir {
            UbioDir::Write => {
                let w_event = Box::new(WriteSubmission::new(volumeIo));
                //pos-cpp => spdk_event_scheduler::ExecuteOrScheduleEvent( 0, w_event );
                //pos-cpp 버전에서는 sync 하게 실행한 다음에 실패하면 async 실행을 위해 enqueue를 하는 구조.
                //일단은 SubmitAsyncIO라는 메서드명의 의미에 충실하고자 무조건 async 실행을 위해 enqueue를 하는 식으로 구현
                spdk_event_scheduler::SendSpdkEvent(Some(w_event));
            }
            UbioDir::Read => {
                let r_event = Box::new(ReadSubmission::new(volumeIo) );
                //pos-cpp => spdk_event_scheduler::ExecuteOrScheduleEvent( 0, r_event );
                spdk_event_scheduler::SendSpdkEvent(Some(r_event));
            }
        }
    }

    pub fn SubmitAsyncAdmin(&self, io: pos_io/*, arrayInfo: */) {
        // TODO
    }

    pub fn SubmitFlush(&self, posIo: pos_io) {
        // TODO
    }

    pub fn CreateVolumeIo(&self, posIo: pos_io) -> Result<VolumeIo, PosEventId> {
        let mut volumeIo = self._CreateVolumeIo(posIo.clone());
        // Note that AioCompletion is injected at _CreateVolumeIo() (unlike pos-cpp)

        volumeIo
    }

    pub fn CompleteIOs(&self) {
        io_dispatcher::CompleteForThreadLocalDeviceList();
    }

    fn _CreateVolumeIo(&self, posIo: pos_io) -> Result<VolumeIo, PosEventId> {
        let sector_size = ChangeByteToSector(posIo.length);

        // TODO: the final result should be copied back to buffer
        let buffer;
        if !posIo.iov.is_null() {
            unsafe { buffer = (*posIo.iov).iov_base; }
        } else {
            buffer = null_mut();
        }

        let array_id = posIo.array_id;
        let volume_id = posIo.volume_id;
        let volume_io_dir = match posIo.ioType {
            0 /*IO_TYPE_READ*/ => UbioDir::Read,
            1 /*IO_TYPE_WRITE*/ => UbioDir::Write,
            _ => {
                let event_id = BLKHDLR_WRONG_IO_DIRECTION;
                error!("Wrong IO direction (only read/write types are supported)");
                return Err(event_id);
            }
        };
        let sector_rba = ChangeByteToSector(posIo.offset);
        let event_type = BackendEvent_FrontendIO;

        let mut aioCompletion = AioCompletion::new(posIo.clone()).to_callback();
        let v = VolumeIo::new(array_id,volume_id,
                              sector_rba, sector_size,
                              volume_io_dir, vec![],
                              aioCompletion);
        Ok(v)
    }
}

pub struct AioCompletion {
    posIo: pos_io,
    errorCount: u32,
}

impl Event for AioCompletion {
    fn GetEventType(&self) -> BackendEvent {
        todo!()
    }

    fn Execute(&mut self) -> bool {
        todo!()
    }
}

impl Callback for AioCompletion {
    fn _DoSpecificJob(&mut self) -> bool {
        // Please note that pos-rtype doesn't have per-core or reactor awareness yet
        // Also, we support write-through mode only.
        self._SendUserCompletion();
        return true;
    }

    fn _TakeCallee(&mut self) -> Option<Box<dyn Callback>> {
        todo!()
    }

    fn _MarkExecutedDone(&mut self) {
        todo!()
    }
}

// TODO: move to this bindings.rs, or somewhere else.
// pos_io는 SPDK header에서 생성된 struct 인데, raw pointer를 멤버로 몇개 가지고
// 있고 raw pointer는 !Send 이기 때문에, 개발자의 책임으로 unsafe를 주고
// Send를 marking 해두어야 함. 그래야, Callback을 EventScheduler로
// 넘겨주고, 다른 Thread가 이를 pick up 하여 사용할 수 있는 구조를 만들 수 있음.
// 이를 다른 방식으로 해결하려면, raw pointer를 thread간 넘기지 않는 구조로 만들어야
// 하는데, 이는 추가 고민이 필요할 듯.
unsafe impl Send for pos_io {}

impl AioCompletion {
    pub fn new(/*volumeIo, */ posIo: pos_io) -> AioCompletion {
        AioCompletion {
            posIo: posIo,
            errorCount: 0,
        }
    }

    pub fn to_callback(self) -> Box<dyn Callback> {
        Box::new(self)
    }

    fn _SendUserCompletion(&mut self) {
        if let Some(complete_cb) = self.posIo.complete_cb {
            // TODO: _GetErrorCount() needs to be in Callback trait
            let ptr_posIo = &mut self.posIo as *mut pos_io;
            unsafe {
                complete_cb(ptr_posIo, POS_IO_STATUS_SUCCESS as c_int);
            }
        }

        if self.posIo.ioType == IO_TYPE_FLUSH as c_int {
            let volume_id = self.posIo.volume_id;
            info!("[{}] Flush End in Aio, volume id : {}", AIO_FLUSH_END.to_string(), volume_id);
        } else {
            // TODO: update pending io count by using IVolumeIoManager
        }
    }
}