use strum_macros::Display;

#[derive(Display, Eq, PartialEq, Clone)]
pub enum ArrayDeviceState {
    NORMAL = 0,
    FAULT,
    REBUILD,
}

impl From<u32> for ArrayDeviceState {
    fn from(val: u32) -> Self {
        match val {
            0 => ArrayDeviceState::NORMAL,
            1 => ArrayDeviceState::FAULT,
            2 => ArrayDeviceState::REBUILD,
            _ => {
                panic!("Invalid array device state provided");
            }
        }
    }
}
