use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref NAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9\s\-_,.]+$").unwrap();
    pub static ref COUPON_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9]{4,20}$").unwrap();
}
