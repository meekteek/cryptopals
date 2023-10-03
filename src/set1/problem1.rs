/// Crypto Challenge Set 1, Problem 1

/// Converts a hexadecimal string into Base64 representation.
///
/// # Panics
///
/// The function will panic if the length of the input hex string is not even
///
/// # Note
///
/// The function relies on `hex_to_bytes` for hex to byte array conversion and `bytes_to_base64` for encoding the byte array into Base64.
/// Ensure the hexadecimal input string does not contain any invalid characters and follows hexadecimal conventions.
pub fn hex_to_base64(s: &str) -> String {
    let bytes = hex_to_bytes(s);
    bytes_to_base64(bytes)
}

/// Converts a hexadecimal string into a vector of bytes (`Vec<u8>`).
///
/// # Panics
/// The function will panic if the input hex string contains characters that are not valid hexadecimal digits
/// or if the length of the hex string is not even.
///
/// # Example
///
/// We will use a hex string that resembles teeek
/// ```
/// use cryptopals::set1::problem1::hex_to_bytes;
/// let bytes = hex_to_bytes("73331c");
/// assert_eq!(bytes, vec![0x73, 0x33, 0x1c]);
/// ```
///
/// # Note
///
/// Each pair of characters in the input hex string is expected to represent a valid byte.
/// Ensure the input string adheres to hexadecimal conventions and contains no invalid characters.
pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let mut output = vec![];
    let chars = hex.chars();
    let mut iter = chars.into_iter();
    // iterate over two characters at a time, convert to byte.
    while let (Some(c1), Some(c2)) = (iter.next(), iter.next()) {
        // combine c1 and c2 into &str
        let combined = format!("{}{}", c1, c2);
        let byte = match u8::from_str_radix(&combined, 16) {
            Ok(v) => v,
            Err(_) => panic!("Invalid hex string"),
        };
        output.push(byte);
    }
    output
}

/// Converts bytes into Base64.
///
/// # Details
///
/// This function performs Base64 encoding by the following:
///
/// 1. Divides the input bytes into 3 byte chunks.
/// 2. Creates a 24-bit number (`triplet`) from 3 byte chunk.
/// 3. `triplet` is then divided into four 6-bit numbers.
/// 4. Each 6-bit number is mapped to Base64 character.
/// 5. If there are are empty bytes, will return padding '='.
///
/// # Examples
///
/// ```rust
/// use cryptopals::set1::problem1::bytes_to_base64;
/// let bytes = vec![0x73, 0x33, 0x1c];
/// let base64_str = bytes_to_base64(bytes);
/// assert_eq!(base64_str, "czMc");
/// ```
///
pub fn bytes_to_base64(bytes: Vec<u8>) -> String {
    let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::new();
    for chunk in bytes.chunks(3) {
        let mut triplet = 0u32;
        for (i, byte) in chunk.iter().enumerate() {
            triplet |= (*byte as u32) << (8 * (2 - i));
        }
        // 0xFC0000 represents 11111100 00000000 00000000
        let first = ((triplet & 0xFC0000) >> 18) as usize;
        // 0x03F000 represents 00000011 11110000 00000000
        let two = ((triplet & 0x03F000) >> 12) as usize;
        // 0x000FC0 represents 00000000 00001111 11000000
        let three = ((triplet & 0x000FC0) >> 6) as usize;
        // 0x00003F represents 00000000 00000000 00111111
        let four = (triplet & 0x00003F) as usize;

        output.push(chars[first] as char);
        output.push(chars[two] as char);

        // if there is two bytes or more
        if chunk.len() > 1 {
            output.push(chars[three] as char);
        } else {
            output.push('=');
        }

        // if there is three bytes or more
        if chunk.len() > 2 {
            output.push(chars[four] as char);
        } else {
            output.push('=');
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_hex_to_base64() {
        let result = hex_to_base64(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
        );
        assert_eq!(
            result,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
