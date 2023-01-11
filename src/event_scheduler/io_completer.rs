use lazy_static::lazy_static;
use crate::include::io_error_type::IOErrorType;
use crate::io::general_io::io_recovery_event_factory::IoRecoveryEventFactory;

pub struct IoCompleter;

lazy_static!{
    static ref recoveryEventFactory: IoRecoveryEventFactory = {
        IoRecoveryEventFactory::new()
    };
}

impl IoCompleter {

    pub fn CompleteUbioWithoutRecovery(&self, errorType: IOErrorType, executeCallback: bool) {
        // TODO
    }

}

pub fn RegisterRecoveryEventFactory(_recoveryEventFactory: IoRecoveryEventFactory) {
    // Not meaningful. Instead, use "recoveryEventFactory" as above.
}