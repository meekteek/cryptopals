use super::problem1::hex_to_bytes;

/// Xor's two different hex strings and returns result as hex string
///
/// # Panics
/// The function will panic if the input strings are not same length
///  Each pair of strings must be valid hex strings
///
pub fn xor(hex1: &str, hex2: &str) -> String {
    if hex1.len() != hex2.len() {
        panic!("Inputs are different lengths. They must be the same");
    }
    let bytes1 = hex_to_bytes(hex1);
    let bytes2 = hex_to_bytes(hex2);

    let xor_output = bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(a, b)| a ^ b)
        .collect::<Vec<u8>>();
    xor_output
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor() {
        let input1 = "1c0111001f010100061a024b53535009181c";
        let input2 = "686974207468652062756c6c277320657965";
        let result = xor(input1, input2);
        assert_eq!(result, "746865206b696420646f6e277420706c6179");
    }
}
