use lazy_static::lazy_static;
lazy_static!{
    pub static ref QosManagerSingleton : QosManager = {
        QosManager
    };
}

pub struct QosManager;
impl QosManager {

    pub fn Initialize(&self) {
        // TODO
    }

    pub fn InitializeSpdkManager(&self) {
        // TODO
    }

}