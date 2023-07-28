//! https://datatracker.ietf.org/doc/html/rfc7233

pub static BYTES_UNIT: &[u8] =b"bytes";

pub fn parse(header: &str) {

}

#[cfg(test)]
mod tests {
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
}