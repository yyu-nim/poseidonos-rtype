use lazy_static::lazy_static;
lazy_static!{
    pub static ref FlushCmdManagerSingleton: FlushCmdManager = {
        FlushCmdManager::new()
    };
}

pub struct FlushCmdManager;

impl FlushCmdManager {
    fn new() -> FlushCmdManager {
        FlushCmdManager
    }
}