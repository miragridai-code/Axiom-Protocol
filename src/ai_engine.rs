use onnxruntime::{environment::Environment, session::Session, tensor::OrtOwnedTensor, LoggingLevel};
use once_cell::sync::Lazy;

/// AI Attack Detection Model
/// Static ONNX environment for all model sessions.
/// Uses once_cell::sync::Lazy to ensure only one global environment is created.
static ONNX_ENV: Lazy<Environment> = Lazy::new(|| {
    Environment::builder()
        .with_name("qubit-onnx-env")
        .with_log_level(LoggingLevel::Warning)
        .build()
        .expect("Failed to initialize ONNX environment")
});

pub struct AttackDetectionModel {
    session: Session<'static>,
}

impl AttackDetectionModel {
    /// Load an ONNX model from file
    /// Load an ONNX model from a static file path. The ONNX environment is static and shared.
    /// The model_path must have a 'static lifetime (e.g., a global constant or string literal).
    pub fn load(model_path: &'static str) -> Result<Self, Box<dyn std::error::Error + 'static>> {
        let session = ONNX_ENV
            .new_session_builder()?
            .with_model_from_file(model_path)?;
        Ok(Self { session })
    }

    /// Run inference on network metrics
    pub fn predict(&mut self, features: &[f32]) -> Result<f32, Box<dyn std::error::Error>> {
        // ONNX expects input as ndarray
        let input_shape = vec![1, features.len()];
        let input_array = ndarray::Array::from_shape_vec(input_shape.clone(), features.to_vec())?;
        let outputs: Vec<OrtOwnedTensor<f32, _>> = self.session.run(vec![input_array])?;
        Ok(outputs[0].as_slice().unwrap()[0])
    }
}

/// Collect and label network data for training
pub fn collect_network_metrics() -> Vec<(Vec<f32>, f32)> {
    // Placeholder: collect metrics such as peer count, block time, tx rate, etc.
    vec![ (vec![0.5, 0.8, 0.2], 1.0), (vec![0.1, 0.2, 0.9], 0.0) ]
}

/// Dynamic reputation scoring based on AI outputs
pub fn calculate_peer_trust_score(model: &mut AttackDetectionModel, metrics: &[f32]) -> Result<f32, Box<dyn std::error::Error>> {
    // Require &mut AttackDetectionModel for ONNX inference
    let score = model.predict(metrics)?;
    Ok(score)
}


pub struct NeuralGuardian {
    weights: [f32; 3],
    learning_rate: f32,
}

impl NeuralGuardian {
    pub fn new() -> Self {
        Self {
            weights: [0.5, 0.3, 0.2],
            learning_rate: 0.01,
        }
    }

    pub fn predict_trust(&self, time_delta: f32, consistency: f32, depth: f32) -> bool {
        let score = (time_delta * self.weights[0]) + 
                    (consistency * self.weights[1]) + 
                    (depth * self.weights[2]);
        score > 0.4
    }

    pub fn train(&mut self, inputs: [f32; 3], target: f32) {
        for i in 0..3 {
            let prediction = (inputs[0] * self.weights[0]) + (inputs[1] * self.weights[1]) + (inputs[2] * self.weights[2]);
            let error = target - prediction;
            self.weights[i] += self.learning_rate * error * inputs[i];
        }
    }
}

impl Default for NeuralGuardian {
    fn default() -> Self {
        Self::new()
    }
}
