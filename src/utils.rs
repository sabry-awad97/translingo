use regex::Regex;

use crate::error::TranslateError;

pub fn extract_too_many_requests_info(html: &str) -> (String, String, String) {
    let ip_regex = Regex::new(r"IP address: (.+?)<br>").unwrap();
    let ip = ip_regex
        .captures(html)
        .map_or(String::new(), |cap| cap[1].to_string());

    let time_regex = Regex::new(r"Time: (.+?)<br>").unwrap();
    let time = time_regex
        .captures(html)
        .map_or(String::new(), |cap| cap[1].to_string());

    let url_regex = Regex::new(r"URL: (.+?)<br>").unwrap();
    let url = url_regex
        .captures(html)
        .map_or(String::new(), |cap| cap[1].to_string())
        .replace("&amp;", "&");

    (ip, time, url)
}

pub fn validate_input_text(text: &str) -> Result<(), TranslateError> {
    if text.trim().is_empty() {
        return Err(TranslateError::InvalidInputError(String::from(
            "Input text cannot be empty",
        )));
    }

    const MAX_TEXT_LENGTH: usize = 5000;
    const VALID_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 !@#$%^&*()-=_+[]{}|;':\",./<>?\\";
    const INVALID_CHARS_ERROR: &str = "Input text contains invalid characters";
    const LENGTH_ERROR: &str = "Input text is too long";

    if text.chars().all(|c| VALID_CHARS.contains(c)) {
        if text.len() > MAX_TEXT_LENGTH {
            return Err(TranslateError::InvalidInputError(LENGTH_ERROR.to_string()));
        } else {
            return Ok(());
        }
    } else {
        return Err(TranslateError::InvalidInputError(
            INVALID_CHARS_ERROR.to_string(),
        ));
    }
}
