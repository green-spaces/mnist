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
        let weights = (0..output).into_iter().map(|_| vec![1.0; input]).collect();
        let bias = vec![0.0; output];

        Self { weights, bias }
    }

    pub fn predict(&self, image: FlatImage) -> Vec<f32> {
        let mut prediction = vec![0.0; self.weights[0].len()];
        self.weights.iter().for_each(|col| {
            col.iter()
                .zip(image.as_bytes().iter())
                .enumerate()
                .for_each(|(idx, (a, b))| {
                    prediction[idx] += a * b;
                })
        });

        prediction
            .iter_mut()
            .zip(self.bias.iter())
            .for_each(|(p, b)| *p += b);

        prediction
    }
}
