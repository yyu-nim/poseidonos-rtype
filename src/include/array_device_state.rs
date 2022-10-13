#[derive(Eq, PartialEq, Clone)]
pub enum ArrayDeviceState {
    NORMAL = 0,
    FAULT,
    REBUILD,
}