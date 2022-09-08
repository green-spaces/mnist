use crate::models::FlatImage;

/// A single hidden layer in a neural network
// TODO Add Initializer and Activation function
pub struct Layer {
    weights: Vec<Vec<f32>>,
    bias: Vec<f32>,
}

impl Layer {
    /// Produces a new layer with n input nods and m output nodes
    pub fn new(input: usize, output: usize) -> Self {
        // weights are a matrix
        // Each column of the matrix is stored in a row to make multiplication easier
        let weights = (0..output)
            .into_iter()
            .map(|i| vec![i as f32 + 1.0; input])
            .collect();
        let bias = vec![0.0; output];

        Self { weights, bias }
    }

    pub fn predict(&self, image: &FlatImage) -> Vec<f32> {
        let mut prediction: Vec<f32> = self
            .weights
            .iter()
            .map(|col| {
                col.iter()
                    .zip(image.as_bytes().iter())
                    .map(|(a, b)| a * b)
                    .sum::<f32>()
            })
            .collect();

        prediction
            .iter_mut()
            .zip(self.bias.iter())
            .for_each(|(p, b)| *p += b);

        prediction
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn almost_equal(a: &[f32], b: &[f32]) -> bool {
        let close_enough = 1.0 / 100000000.0;
        let mut res = a.len() == b.len();
        res &= a
            .iter()
            .zip(b.iter())
            .all(|(a_i, b_i)| (a_i - b_i).abs() < close_enough);
        res
    }

    #[test]
    fn new() {
        let input = 2;
        let output = 3;
        let layer = Layer::new(input, output);

        assert_eq!(layer.bias.len(), output);
        assert_eq!(layer.weights.len(), output);
        assert_eq!(layer.weights[0].len(), input);
    }

    #[test]
    fn basic_predict() {
        let image = FlatImage::from_bytes(&vec![1, 2, 3, 4], 2, 2);
        let layer = Layer::new(4, 2);

        let prediction = layer.predict(&image);
        let expected_prediction = vec![10.0, 20.0];
        assert!(almost_equal(&prediction, &expected_prediction));
    }
}
