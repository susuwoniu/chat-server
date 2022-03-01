use substring::Substring;

pub fn number_to_hex(value: i64) -> String {
    let hex = format!("{:x}", value);
    return format!("#{}", hex.substring(2, hex.len()).to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_hex() {
        let hex = "#f8ca47";

        let hex_result = number_to_hex(4294494791);
        // let argb: i64 = 4294494791;
        dbg!(&hex_result);
        assert_eq!(hex_result, hex);
    }
}
