use sha2::{Digest, Sha256};

pub(crate) fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sha256_output_has_correct_length() {
        let hash = sha256(b"test");
        assert_eq!(hash.len(), 32);
    }
    #[test]
    fn sha256_is_deterministic() {
        let h1 = sha256(b"data");
        let h2 = sha256(b"data");
        assert_eq!(h1, h2);
    }
    #[test]
    fn sha256_known_value() {
        let hash = sha256(b"abc");
        let expected = [
            0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae,
            0x22, 0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61,
            0xf2, 0x00, 0x15, 0xad,
        ];
        assert_eq!(hash, expected);
    }
}
