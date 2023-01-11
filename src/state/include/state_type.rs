use strum_macros::Display;

#[derive(PartialEq, Eq, Display)]
pub enum StateEnum {
    OFFLINE,
    STOP,
    NORMAL,
    BUSY,
    PAUSE,
}
