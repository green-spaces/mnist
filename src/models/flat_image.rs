#![allow(dead_code)]
/// An image in a single row
///
/// Contains height and width so it can be remapped into a 2d image
pub struct FlatImage {
    bytes: Vec<f32>,
    height: usize,
    width: usize,
}

impl FlatImage {
    pub fn from_bytes(raw_bytes: &[u8], height: usize, width: usize) -> Self {
        // Check image is the correct length
        assert_eq!(raw_bytes.len(), height * width);
        Self {
            bytes: raw_bytes.iter().map(|&i| i as f32).collect(),
            height,
            width,
        }
    }

    pub fn as_bytes(&self) -> &[f32] {
        &self.bytes
    }
}
