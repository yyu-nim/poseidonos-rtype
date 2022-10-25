#[derive(PartialEq, Debug)]
pub enum RaidTypeEnum {
    NOT_SUPPORTED,
    NONE,
    RAID0,
    RAID5,
    RAID10,
    RAID6,
}

impl From<&str> for RaidTypeEnum {
    fn from(str: &str) -> Self {
        match str {
            "NOT_SUPPORTED" => RaidTypeEnum::NOT_SUPPORTED,
            "NONE" => RaidTypeEnum::NONE,
            "RAID0" => RaidTypeEnum::RAID0,
            "RAID5" => RaidTypeEnum::RAID5,
            "RAID10" => RaidTypeEnum::RAID10,
            "RAID6" => RaidTypeEnum::RAID6,
            _ => RaidTypeEnum::NOT_SUPPORTED,
        }
    }
}


impl std::fmt::Display for RaidTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RaidTypeEnum::NOT_SUPPORTED => write!(f, "NOT_SUPPORTED"),
            RaidTypeEnum::NONE => write!(f, "NONE"),
            RaidTypeEnum::RAID0 => write!(f, "RAID0"),
            RaidTypeEnum::RAID5 => write!(f, "RAID5"),
            RaidTypeEnum::RAID10 => write!(f, "RAID10"),
            RaidTypeEnum::RAID6 => write!(f, "RAID6"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_raid_type() {
        assert_eq!(RaidTypeEnum::from("NOT_SUPPORTED"), RaidTypeEnum::NOT_SUPPORTED);
        assert_eq!(RaidTypeEnum::from("NONE"), RaidTypeEnum::NONE);
        assert_eq!(RaidTypeEnum::from("RAID0"), RaidTypeEnum::RAID0);
        assert_eq!(RaidTypeEnum::from("RAID5"), RaidTypeEnum::RAID5);
        assert_eq!(RaidTypeEnum::from("RAID10"), RaidTypeEnum::RAID10);
        assert_eq!(RaidTypeEnum::from("RAID6"), RaidTypeEnum::RAID6);

        assert_eq!(RaidTypeEnum::from("R"), RaidTypeEnum::NOT_SUPPORTED);
    }

    #[test]
    fn test_raid_type_to_str() {
        assert_eq!(RaidTypeEnum::NOT_SUPPORTED.to_string(), "NOT_SUPPORTED");
        assert_eq!(RaidTypeEnum::RAID0.to_string(), "RAID0");
        assert_eq!(RaidTypeEnum::RAID5.to_string(), "RAID5");
        assert_eq!(RaidTypeEnum::RAID10.to_string(), "RAID10");
        assert_eq!(RaidTypeEnum::RAID6.to_string(), "RAID6");
    }
}