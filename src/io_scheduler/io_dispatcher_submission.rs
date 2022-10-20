use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use crate::bio::ubio::Ubio;
use crate::device::base::ublock_device::UBlockDevice;
use crate::io_scheduler::dispatcher_policy::DispatcherPolicy;

lazy_static!{
    pub static ref IODispatcherSubmissionSingleton: Arc<Mutex<IODispatcherSubmission>> = {
        let dispatcher_submission = IODispatcherSubmission::new();
        Arc::new(Mutex::new(dispatcher_submission))
    };
}

pub struct IODispatcherSubmission;
impl IODispatcherSubmission {

    pub fn new() -> IODispatcherSubmission {
        IODispatcherSubmission
    }

    pub fn SubmitIO(&self, ublock: Box<dyn UBlockDevice>,
                    mut ubio: Ubio,
                    _dispatcherPolicy: Option<DispatcherPolicy>) {
        ublock.SubmitAsyncIO(&mut ubio);
    }

}
