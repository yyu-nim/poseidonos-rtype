use lazy_static::lazy_static;
lazy_static!{
    pub static ref IoTimeoutCheckerSingleton: IoTimeoutChecker = {
        IoTimeoutChecker::new()
    };
}


pub struct IoTimeoutChecker;

impl IoTimeoutChecker {
    fn new() -> IoTimeoutChecker {
        IoTimeoutChecker
    }

    pub fn Initialize(&self) {
        // TODO
    }
}