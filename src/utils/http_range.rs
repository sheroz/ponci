//! Implemented according to: Hypertext Transfer Protocol (HTTP/1.1): Range Requests
//! 
//! [RFC7233](https://datatracker.ietf.org/doc/html/rfc7233)
//! 
//! Additional resources:
//! 
//! https://developer.mozilla.org/en-US/docs/Web/HTTP/Range_requests
//! 
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

    if parts.len() != 2 {
        log::error!("Invalid range: {}", range_value);
        return None;
    }
    let mut ranges = Vec::<Range<u64>>::new();

    let params = parts[1].split("/").map(|p| p.trim()).collect::<Vec<_>>();
    if params.is_empty() {
        return None;
    }

    let range_params = params[0].split(",");
    for range_param in range_params {
        let values = range_param.split("-").map(|v| v.trim()).collect::<Vec<_>>();
        if values.len() != 2 {
            log::error!("Invalid range: {}, param: {}", range_value, range_param);
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

        ranges.push(range);
    }

    ranges.sort_by(|a,b| a.start.cmp(&b.start));

    let ranges_count = ranges.len();
    if ranges_count > 1 {
        // merge continuous and overlapping ranges
        let mut retain =vec![true; ranges_count];
        let mut range_last = ranges[0].clone();
        for (index, range) in ranges.iter_mut().enumerate() {
            if index != 0 && (range_last.end + 1) >= range.start {
                range.start = range_last.start;     
                retain[index - 1] = false;
            }
            range_last = range.clone();
        }

        // clean-up merged ranges
        let mut index = 0;
        ranges.retain(|_|{let keep = retain[index]; index += 1; keep});
    }
    
    if log::log_enabled!(log::Level::Trace) {
        log::trace!("Parsed ranges:\n{:#?}", ranges);
    }

    Some(ranges)
}

#[cfg(test)]
mod tests {
    use super::*;
    /// https://datatracker.ietf.org/doc/html/rfc7233#section-4.2
    /// 
    /// Examples of byte-ranges-specifier values:
    ///    -  The first 500 bytes (byte offsets 0-499, inclusive):
    ///        bytes=0-499
    ///    -  The second 500 bytes (byte offsets 500-999, inclusive):
    ///         bytes=500-999
    /// 
    /// Additional examples, assuming a representation of length 10000:
    ///    The final 500 bytes (byte offsets 9500-9999, inclusive):
    ///         bytes=-500
    ///    Or:
    ///         bytes=9500-
    ///    -  The first and last bytes only (bytes 0 and 9999):
    ///         bytes=0-0,-1
    ///    -  Other valid (but not canonical) specifications of the second 500
    ///       bytes (byte offsets 500-999, inclusive):
    ///         bytes=500-600,601-999
    ///         bytes=500-700,601-999

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
        let range = parse("bytes=601-999,500-600", 10000);
        assert_eq!(range.unwrap(), vec![500..999]);
    }

    #[test]
    fn test8() {
        let range = parse("bytes=500-700,601-999", 10000);
        assert_eq!(range.unwrap(), vec![500..999]);
    }

    #[test]
    fn test9() {
        let range = parse("bytes=601-999,500-700", 10000);
        assert_eq!(range.unwrap(), vec![500..999]);
    }

    #[test]
    fn test10() {
        let range = parse("bytes=300-400,400-700,601-999", 10000);
        assert_eq!(range.unwrap(), vec![300..999]);
    }

}
