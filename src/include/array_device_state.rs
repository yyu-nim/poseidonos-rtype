#[derive(Eq, PartialEq)]
pub enum ArrayDeviceState {
    NORMAL = 0,
    FAULT,
    REBUILD,
}