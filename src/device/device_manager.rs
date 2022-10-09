use lazy_static::lazy_static;
use crate::device::i_io_dispatcher::IIODispatcher;
lazy_static!{
    pub static ref DeviceManagerSingleton: DeviceManager = {
        DeviceManager::new()
    };
}

pub struct DeviceManager {

}

impl DeviceManager {
    pub fn new() -> DeviceManager {
        DeviceManager {

        }
    }

    pub fn Initialize(&self) {
        // self.ioDispatcher = Some(ioDispatcherInterface);
        // io_dispatcher.rs 의 fn을 호출하는 형식이 나을 듯
        // (vs. IODispatcherSingleton의 reference를 직접 가져오는 형태)
        // 즉, 이 구현은 일부러 비워둠.
    }
}