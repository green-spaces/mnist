use std::fs;
use std::path::Path;

use crate::models::{FlatImage, Label};

const IMAGE_FILE_MAGIC_NUMBER: i32 = 2051;
const LABEL_FILE_MAGIC_NUMBER: i32 = 2049;
const EXPECTED_IMAGE_SIZE: i32 = 28;
const IMAGE_HEADER_LENGTH: usize = 16;
const LABEL_HEADER_LENGTH: usize = 8;

/// Load all MNIST images from file
pub fn load_images(path: &Path) -> Vec<FlatImage> {
    let raw_data = fs::read(path).unwrap();
    // Parse header
    let magic_number = i32::from_be_bytes(raw_data[0..4].try_into().unwrap());
    if magic_number != IMAGE_FILE_MAGIC_NUMBER {
        panic!("path ({}) is not a mnist image file", path.display());
    }
    let image_count = i32::from_be_bytes(raw_data[4..8].try_into().unwrap());
    let image_width = i32::from_be_bytes(raw_data[8..12].try_into().unwrap());
    let image_height = i32::from_be_bytes(raw_data[12..16].try_into().unwrap());

    let image_size = image_height * image_width;

    assert_eq!(image_width, EXPECTED_IMAGE_SIZE);
    assert_eq!(image_height, EXPECTED_IMAGE_SIZE);

    let flat_images: Vec<FlatImage> = raw_data[IMAGE_HEADER_LENGTH..]
        .chunks(image_size as usize)
        .map(|raw_image| {
            FlatImage::from_bytes(raw_image, image_height as usize, image_width as usize)
        })
        .collect();

    assert_eq!(flat_images.len(), image_count as usize);

    flat_images
}

/// Load all MNIST images from file
pub fn load_labels(path: &Path) -> Vec<Label> {
    let raw_data = fs::read(path).unwrap();
    // Parse header
    let magic_number = i32::from_be_bytes(raw_data[0..4].try_into().unwrap());
    if magic_number != LABEL_FILE_MAGIC_NUMBER {
        panic!("path ({}) is not a mnist label file", path.display());
    }

    let label_count = i32::from_be_bytes(raw_data[4..8].try_into().unwrap());

    let labels = raw_data[LABEL_HEADER_LENGTH..]
        .iter()
        .map(Label::new)
        .collect::<Vec<_>>();

    assert_eq!(labels.len(), label_count as usize);

    labels
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn load_training_images() {
        let root_dir = env!("CARGO_MANIFEST_DIR");
        let mut training_images_file = PathBuf::new();
        training_images_file.push(root_dir);
        training_images_file.push("data/train-images-idx3-ubyte");

        let images = super::load_images(training_images_file.as_path());
        assert_eq!(images.len(), 60000);
    }

    #[test]
    fn load_test_images() {
        let root_dir = env!("CARGO_MANIFEST_DIR");
        let mut training_images_file = PathBuf::new();
        training_images_file.push(root_dir);
        training_images_file.push("data/t10k-images-idx3-ubyte");

        let images = super::load_images(training_images_file.as_path());
        assert_eq!(images.len(), 10000);
    }

    #[test]
    fn load_training_labels() {
        let root_dir = env!("CARGO_MANIFEST_DIR");
        let mut training_labels_file = PathBuf::new();
        training_labels_file.push(root_dir);
        training_labels_file.push("data/train-labels-idx1-ubyte");

        let labels = super::load_labels(training_labels_file.as_path());
        assert_eq!(labels.len(), 60000);
    }

    #[test]
    fn load_test_labels() {
        let root_dir = env!("CARGO_MANIFEST_DIR");
        let mut test_label_file = PathBuf::new();
        test_label_file.push(root_dir);
        test_label_file.push("data/t10k-labels-idx1-ubyte");

        let labels = super::load_labels(test_label_file.as_path());
        assert_eq!(labels.len(), 10000);
    }

    #[test]
    #[should_panic]
    fn load_labels_from_images_file_panic() {
        let root_dir = env!("CARGO_MANIFEST_DIR");
        let mut training_labels_file = PathBuf::new();
        training_labels_file.push(root_dir);
        training_labels_file.push("data/train-images-idx3-ubyte");

        let labels = super::load_labels(training_labels_file.as_path());
    }

    #[test]
    #[should_panic]
    fn load_images_from_labels_file_panic() {
        let root_dir = env!("CARGO_MANIFEST_DIR");
        let mut training_images_file = PathBuf::new();
        training_images_file.push(root_dir);
        training_images_file.push("data/train-labels-idx1-ubyte");

        let images = super::load_images(training_images_file.as_path());
    }
}
