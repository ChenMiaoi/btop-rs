use std::time::{SystemTime, UNIX_EPOCH};

pub mod logger;

pub enum InvalidIntReason {
    ValueTooHigh,
    ValueTooLow,
    ParseError,
}

pub fn time_s() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

pub fn is_in<T: PartialEq>(first: &T, others: &[T]) -> bool {
    others.iter().any(|item| item == first) || !others.is_empty() && is_in(first, &others[1..])
}

pub fn is_bool(value: &str) -> bool {
    is_in(&value, &["true", "false", "True", "False"])
}

pub fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_lowercase().as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

pub fn is_int(value: &str) -> bool {
    match value.parse::<i32>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn is_valid_int(key: &str, value: &str) -> Result<i32, InvalidIntReason> {
    let parsed_value = match key {
        "update_ms" => match value.parse::<i32>() {
            Ok(parsed) if parsed < 100 => Err(InvalidIntReason::ValueTooLow),
            Ok(parsed) if parsed > 86400000 => Err(InvalidIntReason::ValueTooHigh),
            Ok(parsed) => Ok(parsed),
            _ => Err(InvalidIntReason::ParseError),
        },
        _ => match value.parse::<i32>() {
            Ok(parsed) => Ok(parsed),
            _ => Err(InvalidIntReason::ParseError),
        },
    };

    // match parsed_value {
    //     Ok(parsed) => Ok(parsed),
    //     Err(err) => Err(err),
    // }
    parsed_value
}
