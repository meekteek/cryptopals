/// Crypto Challenge Set 1, Problem 1

fn hex_to_base64(s: &str) -> String {
    let bytes = hex_to_bytes(s);
    bytes_to_base64(bytes)
}

pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let mut output = vec![];
    let chars = hex.chars();
    let mut iter = chars.into_iter();
    // iterate over two characters at a time, convert to byte.
    while let (Some(c1), Some(c2)) = (iter.next(), iter.next()) {
        // combine c1 and c2 into &str
        let combined = format!("{}{}", c1, c2).as_str();
        let byte = match u8::from_str_radix(combined, 16) {
            Ok(v) => v,
            Err(e) => panic!("Invalid hex string"),
        };
        output.push(byte);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = hex_to_base64_withlib(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
        );
        assert_eq!(
            result,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
