use crate::include::array_device_state::ArrayDeviceState;

#[derive(Eq, PartialEq)]
pub struct DeviceMeta {
    uid: String,
    state: ArrayDeviceState,
}

