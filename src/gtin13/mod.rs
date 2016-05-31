use std::ascii::AsciiExt;
use utils;

#[derive(Debug)]
pub enum FixError {
    NonAsciiString,
    TooLong,
    CheckDigitIncorrect
}

/// Check that a GTIN-13 code is valid by checking the length (should be
/// exactly 13 digits) and that the check-digit is correct.
///
/// # Examples
/// ```
/// use gtin_validate::gtin13;
///
/// assert_eq!(gtin13::check("1498279802125"), true);  // Valid GTIN-13
/// assert_eq!(gtin13::check("468712378699"), false);  // Too short
/// assert_eq!(gtin13::check("1498279802124"), false); // Bad check digit
/// ```
pub fn check(upc: &str) -> bool {
    // Chech that input is ASCII with length 13
    if upc.is_ascii() == false {
        return false;
    }
    if upc.len() != 13 {
        return false;
    }
    println!("pass 1");

    let bytes = upc.as_bytes();
    if !utils::is_number(bytes, 13) {
        return false;
    }

    println!("pass 2");

    let check = utils::compute_check_digit(bytes, 13);

    // Compare check digit
    if check != bytes[12] - 48 {
        return false;
    }

    return true;
}

pub fn fix(upc: &str) -> Result<String, FixError> {
    if !upc.is_ascii() {
        return Err(FixError::NonAsciiString);
    }
    if upc.len() > 13 {
        return Err(FixError::TooLong);
    }
    panic!("Not implemented!");
}

#[cfg(test)]
mod tests {
    use super::check;

    #[test]
    fn check_valid() {
        assert_eq!(check("0000000000000"), true);
        assert_eq!(check("8845791354268"), true);
        assert_eq!(check("0334873614126"), true);
    }

    #[test]
    fn check_invalid_length() {
        assert_eq!(check("000"), false);
        assert_eq!(check("00000000000000"), false);
    }

    #[test]
    fn check_non_ascii() {
        assert_eq!(check("❤"), false);
    }

    #[test]
    fn check_non_numeric() {
        assert_eq!(check("a"), false);
        assert_eq!(check("abcdabcdabcda"), false); // length 13
        assert_eq!(check("000000000000a"), false); // invalid check digit
    }

    #[test]
    fn check_invalid_check_digit() {
        assert_eq!(check("0000000000001"), false);
        assert_eq!(check("0000000000002"), false);
        assert_eq!(check("0000000000003"), false);
        assert_eq!(check("0000000000004"), false);
        assert_eq!(check("0000000000005"), false);
        assert_eq!(check("0000000000006"), false);
        assert_eq!(check("0000000000007"), false);
        assert_eq!(check("0000000000008"), false);
        assert_eq!(check("0000000000009"), false);
    }

    #[test]
    fn check_static_data() {
        assert_eq!(check("0000000000000"), true);
        assert_eq!(check("0123456789012"), true);
        assert_eq!(check("0123456789013"), false);
        assert_eq!(check("0999999999993"), true);
        assert_eq!(check("0999999999999"), false);
        assert_eq!(check("4459121265748"), true);
        assert_eq!(check("4459121265747"), false);
    }
}
