use core::str;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod logger;
pub mod macro_def;

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

pub fn ssplit<'a>(s: &'a str, delim: char) -> Vec<&'a str> {
    s.split(delim)
        .map(|substring| substring.trim())
        .filter(|substring| !substring.is_empty())
        .collect()
}
