#![allow(dead_code)]
pub struct FlatImage {
    bytes: Vec<u8>,
    height: usize,
    width: usize,
}

impl FlatImage {
    pub fn from_bytes(raw_bytes: &[u8], height: usize, width: usize) -> Self {
        // Check image is the correct length
        assert_eq!(raw_bytes.len(), height * width);
        Self {
            bytes: raw_bytes.to_owned(),
            height,
            width,
        }
    }
}
