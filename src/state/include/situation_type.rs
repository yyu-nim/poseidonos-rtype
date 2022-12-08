pub enum SituationEnum {
    DEFAULT,
    NORMAL,
    TRY_MOUNT,
    DEGRADED,
    TRY_UNMOUNT,
    JOURNAL_RECOVERY,
    REBUILDING,
    FAULT,
}

pub struct SituationType {
    pub val: SituationEnum,
}

impl SituationType {
    pub fn new(val: SituationEnum) -> SituationType {
        SituationType {
            val
        }
    }
}