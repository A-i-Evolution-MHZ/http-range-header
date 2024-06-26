#![no_main]
use http_range_header::parse_range_header;
use libfuzzer_sys::fuzz_target;
use regex::Regex;

lazy_static::lazy_static! {
    // Should match most valid ranges
    static ref STANDARD_RANGE: Regex = Regex::new("^bytes=((\\d+-\\d+,\\s?)|(\\d+-,\\s?)|(-\\d+,\\s?))*((\\d+-\\d+)|(\\d+-)|(-\\d+))+$").unwrap();
}

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        // Discard valid ranges, this test is looking for invalid ranges which should
        // 1. Either not be parsed, or if it's parsed, not pass validation
        // 2. Not panic
        if STANDARD_RANGE.is_match(s) {
            return;
        }
        if let Ok(parsed) = parse_range_header(s) {
            let v = parsed.validate(u64::MAX);
            assert!(
                parsed.validate(u64::MAX).is_err(),
                "range {:?} accepted as {:?}",
                s,
                v
            );
        }
    }
});
