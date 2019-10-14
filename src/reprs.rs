use crate::error::{Error, Result};

pub fn parse_num(num_str: &str) -> Result<i64> {
    let mut radix: Option<u32> = None;
    let mut slice = num_str;
    if slice.starts_with("0x") {
        slice = &slice[2..];
        radix = Some(16);
    } else if slice.ends_with('h') {
        slice = &slice[..slice.len() - 1];
        radix = Some(16);
    } else if slice.starts_with("0b") {
        slice = &slice[2..];
        radix = Some(2);
    } else if slice.ends_with('b') {
        slice = & slice[..slice.len() - 1];
        radix = Some(2);
    }
    if let Some(radix) = radix {
        i64::from_str_radix(slice, radix).map_err(|_| Error::NumParseError(num_str))
    } else {
        [10, 16].iter()
            .find_map(|&radix| i64::from_str_radix(slice, radix).ok())
            .ok_or(Error::NumParseError(num_str))
    }
}

#[test]
fn test_parse_hex() {
    assert_eq!(parse_num("0xc").unwrap(), 12);
    assert_eq!(parse_num("12h").unwrap(), 18);
    assert_eq!(parse_num("0xCaFeBaBe123").unwrap(), 13949712720163);
    assert_eq!(parse_num("dEaDbEeFh").unwrap(), 3735928559);
    assert!(parse_num("0xCh").is_err());

    assert_eq!(parse_num("Cc").unwrap(), 204);
    assert_eq!(parse_num("123abc").unwrap(), 1194684);
}

#[test]
fn test_parse_bin() {
    assert_eq!(parse_num("0b101").unwrap(), 5);
    assert_eq!(parse_num("011101110b").unwrap(), 238);
    assert_eq!(parse_num("000110101001011110b").unwrap(), 27230);
    assert!(parse_num("0b101b").is_err());
    assert!(parse_num("1010012b").is_err());
}

#[test]
fn test_parse_dec() {
    assert_eq!(parse_num("01010").unwrap(), 1010);
    assert_eq!(parse_num("1234").unwrap(), 1234);
    assert_eq!(parse_num("-4321").unwrap(), -4321);
}
