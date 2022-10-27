use log::warn;
use crate::include::pos_event_id::PosEventId;

const MIN_LEN: usize = 2;
const MAX_LEN: usize = 63;

pub fn CheckArrayName(name: &String) -> Result<(), PosEventId> {
    if name.len() < MIN_LEN {
        let eventId = PosEventId::CREATE_ARRAY_NAME_TOO_SHORT;
        warn!("[{}] name len: {}", eventId.to_string(), name.len());
        return Err(eventId);
    }
    else if name.len() > MAX_LEN {
        let eventId = PosEventId::CREATE_ARRAY_NAME_TOO_LONG;
        warn!("[{}] name len: {}", eventId.to_string(), name.len());
        return Err(eventId);
    }

    if name.chars().nth(0).unwrap().is_whitespace() || name.chars().last().unwrap().is_whitespace() {
        let eventId = PosEventId::CREATE_ARRAY_NAME_START_OR_END_WITH_SPACE;
        warn!("[{}] name: {}", eventId.to_string(), name);
        return Err(eventId);
    }

    let char_validities: Vec<bool> = name.chars().map(|c| c.is_numeric() || c.is_alphabetic() ).collect();
    if !char_validities.iter().all(|v| *v == true) {
        let eventId = PosEventId::CREATE_ARRAY_NAME_INCLUDES_SPECIAL_CHAR;
        warn!("[{}] name allowed only: numerics, ascii alphabetic, name: {}", eventId.to_string(), name);
        return Err(eventId);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_name() {
        let name = "a".to_string();
        assert_eq!(CheckArrayName(&name).is_err(), true);
        assert_eq!(CheckArrayName(&name), Err(PosEventId::CREATE_ARRAY_NAME_TOO_SHORT));
    }

    #[test]
    fn test_long_name() {
        let name = "a".repeat(100);
        assert_eq!(CheckArrayName(&name).is_err(), true);
        assert_eq!(CheckArrayName(&name), Err(PosEventId::CREATE_ARRAY_NAME_TOO_LONG));
    }

    #[test]
    fn test_name_starts_with_whitespace() {
        let name = " name".to_string();
        assert_eq!(CheckArrayName(&name).is_err(), true);
        assert_eq!(CheckArrayName(&name), Err(PosEventId::CREATE_ARRAY_NAME_START_OR_END_WITH_SPACE));
    }

    #[test]
    fn test_name_ends_with_whitespace() {
        let name = "name  ".to_string();
        assert_eq!(CheckArrayName(&name).is_err(), true);
        assert_eq!(CheckArrayName(&name), Err(PosEventId::CREATE_ARRAY_NAME_START_OR_END_WITH_SPACE));
    }

    #[test]
    fn test_name_with_non_valid_chars() {
        let name = "&^%*".to_string();
        assert_eq!(CheckArrayName(&name).is_err(), true);
        assert_eq!(CheckArrayName(&name), Err(PosEventId::CREATE_ARRAY_NAME_INCLUDES_SPECIAL_CHAR));
    }

    #[test]
    fn test_valid_array_name() {
        let name = "POSArray".to_string();
        assert_eq!(CheckArrayName(&name).is_ok(), true);
    }
}