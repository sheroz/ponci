//! https://datatracker.ietf.org/doc/html/rfc7233
//! https://developer.mozilla.org/en-US/docs/Web/HTTP/Range_requests
//! https://http.dev/range-request

use std::ops::Range;
pub static RANGE_UNIT: &str = "bytes";

pub fn parse(range_value: &str, bytes_count: u64) -> Option<Vec<Range<u64>>> {
    if range_value.is_empty() {
        return None;
    }

    let parts = range_value.split("=").map(|p| p.trim()).collect::<Vec<_>>();
    if parts.is_empty() {
        return None;
    }

    if parts[0] != RANGE_UNIT {
        log::error!("Invalid range unit: {}", parts[0]);
        return None;
    }
    let mut result = Vec::<Range<u64>>::new();

    if parts.len() != 2 {
        log::error!("Invalid range: {}", range_value);
        return None;
    }

    let params = parts[1].split(",");
    for param in params {
        let values = param.split("-").map(|v| v.trim()).collect::<Vec<_>>();
        if values.len() != 2 {
            log::error!("Invalid range: {}, param: {}", range_value, param);
            return None;
        }
        let mut range = 0..bytes_count - 1;
        let start = values[0];
        let end = values[1];

        if !start.is_empty() && !end.is_empty() {
            range.start = u64::from_str_radix(start, 10).unwrap();
            range.end = u64::from_str_radix(end, 10).unwrap();
        }
        if start.is_empty() && !end.is_empty() {
            let count = u64::from_str_radix(end, 10).unwrap();
            range.start = bytes_count - count;
        }

        if !start.is_empty() && end.is_empty() {
            range.start = u64::from_str_radix(start, 10).unwrap();
        }

        result.push(range);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    /*

    Examples of byte-ranges-specifier values:

       o  The first 500 bytes (byte offsets 0-499, inclusive):
            bytes=0-499
       o  The second 500 bytes (byte offsets 500-999, inclusive):
            bytes=500-999

    Additional examples, assuming a representation of length 10000:
       The final 500 bytes (byte offsets 9500-9999, inclusive):
            bytes=-500
       Or:
            bytes=9500-

       o  The first and last bytes only (bytes 0 and 9999):
            bytes=0-0,-1

       o  Other valid (but not canonical) specifications of the second 500
          bytes (byte offsets 500-999, inclusive):
            bytes=500-600,601-999
            bytes=500-700,601-999

    */
    #[test]
    fn test1() {
        let range = parse("bytes=0-499", 10000);
        assert_eq!(range.unwrap(), vec![0..499]);
    }

    #[test]
    fn test2() {
        let range = parse("bytes=500-999", 10000);
        assert_eq!(range.unwrap(), vec![500..999]);
    }

    #[test]
    fn test3() {
        let range = parse("bytes=-500", 10000);
        assert_eq!(range.unwrap(), vec![9500..9999]);
    }

    #[test]
    fn test4() {
        let range = parse("bytes=9500-", 10000);
        assert_eq!(range.unwrap(), vec![9500..9999]);
    }

    #[test]
    fn test5() {
        let range = parse("bytes=0-0,-1", 10000);
        assert_eq!(range.unwrap(), vec![0..0, 9999..9999]);
    }

    #[test]
    fn test6() {
        let range = parse("bytes=500-600,601-999", 10000);
        assert_eq!(range.unwrap(), vec![500..999]);
    }

    #[test]
    fn test7() {
        let range = parse("bytes=500-700,601-999", 10000);
        assert_eq!(range.unwrap(), vec![500..999]);
    }
}
