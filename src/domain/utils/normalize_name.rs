use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Deserializer};

lazy_static! {
    pub static ref NAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9\s\-_,.]+$").unwrap();
    pub static ref COUPON_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9]{4,20}$").unwrap();
}

pub fn normalize_name<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let s = s.trim().to_lowercase();
    let s = s.replace(|c: char| c.is_whitespace(), " ");
    Ok(s)
}
