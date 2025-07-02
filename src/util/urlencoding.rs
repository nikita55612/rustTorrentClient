use std::fmt::Write;

pub fn urlencode(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 3);

    for &b in bytes {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'.' | b'-' | b'_' | b'~' => {
                encoded.push(b as char)
            }
            _ => {
                write!(encoded, "%{:02X}", b).unwrap();
            }
        }
    }

    encoded
}
