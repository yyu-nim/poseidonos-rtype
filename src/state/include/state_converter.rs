use crate::state::include::situation_type::SituationEnum;
use crate::state::include::state_type::StateEnum;

pub fn Convert(situ: &SituationEnum) -> StateEnum {
    match situ {
        SituationEnum::DEFAULT => StateEnum::OFFLINE,
        SituationEnum::NORMAL => StateEnum::NORMAL,
        SituationEnum::TRY_MOUNT => StateEnum::PAUSE,
        SituationEnum::DEGRADED => StateEnum::BUSY,
        SituationEnum::TRY_UNMOUNT => StateEnum::PAUSE,
        SituationEnum::JOURNAL_RECOVERY => StateEnum::PAUSE,
        SituationEnum::REBUILDING => StateEnum::BUSY,
        SituationEnum::FAULT => StateEnum::STOP,
    }
}