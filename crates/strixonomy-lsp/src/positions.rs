/// Convert a byte column offset within `line` to an LSP UTF-16 code unit offset.
pub fn byte_col_to_utf16(line: &str, byte_col: usize) -> u32 {
    let mut utf16 = 0u32;
    for (byte_idx, ch) in line.char_indices() {
        if byte_idx >= byte_col {
            break;
        }
        utf16 += ch.len_utf16() as u32;
    }
    utf16
}

/// Convert an LSP UTF-16 code unit offset within `line` to a byte offset.
pub fn utf16_offset_to_byte(line: &str, utf16_col: u32) -> usize {
    let mut utf16_seen = 0u32;
    for (byte_idx, ch) in line.char_indices() {
        if utf16_seen >= utf16_col {
            return byte_idx;
        }
        utf16_seen += ch.len_utf16() as u32;
    }
    line.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf16_round_trip_ascii() {
        let line = "ex:Person a owl:Class";
        let byte = 3;
        let utf16 = byte_col_to_utf16(line, byte);
        assert_eq!(utf16_offset_to_byte(line, utf16), byte);
    }

    #[test]
    fn utf16_round_trip_multibyte_label() {
        let line = "rdfs:label \"日本語\"";
        let byte = line.find('日').expect("multibyte char");
        let utf16 = byte_col_to_utf16(line, byte);
        assert_eq!(utf16_offset_to_byte(line, utf16), byte);
    }
}
