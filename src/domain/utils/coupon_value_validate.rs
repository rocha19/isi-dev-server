use lazy_static::lazy_static;
use regex::Regex;
use validator::ValidationError;

lazy_static! {
    pub static ref NAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9\s\-_,.]+$").unwrap();
    pub static ref COUPON_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9]{4,20}$").unwrap();
}

pub fn validate_coupon_value(value: u64) -> Result<(), ValidationError> {
    match value <= 1 {
        true => return Err(ValidationError::new("Value must be positive")),
        false => (),
    }
    Ok(())
}
