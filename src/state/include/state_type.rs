#[derive(PartialEq)]
pub enum StateEnum {
    OFFLINE,
    STOP,
    NORMAL,
    BUSY,
    PAUSE,
}

#[derive(PartialEq)]
pub struct StateType {
    pub val: StateEnum,
}

impl StateType {
    pub fn new(val: StateEnum) -> StateType {
        StateType {
            val
        }
    }
}