use log::warn;
use crate::include::pos_event_id::PosEventId::*;

const MIN_LEN: usize = 2;
const MAX_LEN: usize = 63;

pub fn CheckArrayName(name: &String) -> i32 {
    if name.len() < MIN_LEN {
        let ret = CREATE_ARRAY_NAME_TOO_SHORT as i32;
        warn!("[CREATE_ARRAY_NAME_TOO_SHORT] name len: {}", name.len());
        return ret;
    }
    else if name.len() > MAX_LEN {
        let ret = CREATE_ARRAY_NAME_TOO_LONG as i32;
        warn!("[CREATE_ARRAY_NAME_TOO_LONG] name len: {}", name.len());
        return ret;
    }

    if name.chars().nth(0).unwrap().is_whitespace() || name.chars().last().unwrap().is_whitespace() {
        let ret = CREATE_ARRAY_NAME_START_OR_END_WITH_SPACE as i32;
        warn!("[CREATE_ARRAY_NAME_START_OR_END_WITH_SPACE] name: {}", name);
        return ret;
    }

    let char_validities: Vec<bool> = name.chars().map(|c| c.is_numeric() || c.is_alphabetic() ).collect();
    if !char_validities.iter().all(|v| *v == true) {
        let ret = CREATE_ARRAY_NAME_INCLUDES_SPECIAL_CHAR as i32;
        warn!("[CREATE_ARRAY_NAME_INCLUDES_SPECIAL_CHAR] name allowed only: numerics, ascii alphabetic, name: {}", name);
        return ret;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_name() {
        let name = "a".to_string();
        assert_ne!(CheckArrayName(&name), 0);
    }

    #[test]
    fn test_long_name() {
        let name = "a".repeat(100);
        assert_ne!(CheckArrayName(&name), 0);
    }

    #[test]
    fn test_name_starts_with_whitespace() {
        let name = " name".to_string();
        assert_ne!(CheckArrayName(&name), 0);
    }

    #[test]
    fn test_name_ends_with_whitespace() {
        let name = "name  ".to_string();
        assert_ne!(CheckArrayName(&name), 0);
    }

    #[test]
    fn test_name_with_non_valid_chars() {
        let name = "&^%*".to_string();
        assert_ne!(CheckArrayName(&name), 0);
    }

    #[test]
    fn test_valid_array_name() {
        let name = "POSArray".to_string();
        assert_eq!(CheckArrayName(&name), 0);
    }
}