# ONNX Runtime Integration Guide

This document describes how to use ONNX Runtime for AI inference in Qubit Protocol.

## Overview
- The AI attack detection module now uses ONNX Runtime instead of TensorFlow.
- The ONNX environment is managed as a static singleton for safe and efficient inference.
- Models must be exported in ONNX format (e.g., from PyTorch, TensorFlow, or scikit-learn).

## Dependencies
- `onnxruntime = "0.0.14"`
- `ndarray = "0.15"`
- `once_cell = "1.19"`

## Example Usage

```
use ai_engine::AttackDetectionModel;

// Load ONNX model
let mut model = AttackDetectionModel::load("model.onnx").expect("Failed to load model");

// Run inference
let features = vec![0.5, 0.8, 0.2];
let score = model.predict(&features).expect("Inference failed");
println!("Trust score: {}", score);
```

## Model Export
- Export your trained model to ONNX format using your ML framework.
- Place the ONNX file in your project directory (e.g., `model.onnx`).

## Notes
- The ONNX environment is static and thread-safe.
- The model path must be valid and accessible at runtime.
- Inference requires a mutable reference to the model instance.

## Migration Steps
1. Remove TensorFlow dependencies from `Cargo.toml`.
2. Add ONNX Runtime, ndarray, and once_cell dependencies.
3. Update AI logic to use ONNX model loading and inference.
4. Ensure all inference calls use `&mut AttackDetectionModel`.

## Troubleshooting
- If you encounter lifetime errors, ensure the ONNX environment is static and the session is created from it.
- If you see model loading errors, verify the ONNX file path and format.

---
For further details, see `src/ai_engine.rs` and the ONNX Runtime Rust documentation.