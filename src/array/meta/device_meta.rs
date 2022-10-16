use crate::include::array_device_state::ArrayDeviceState;

#[derive(Eq, PartialEq, Clone)]
pub struct DeviceMeta {
    pub uid: String,
    pub state: ArrayDeviceState,
}

impl DeviceMeta {
    pub fn new(uid: String, state: ArrayDeviceState) -> Self {
        Self {
            uid,
            state,
        }
    }
}
