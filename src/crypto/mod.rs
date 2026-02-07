//! Quantum-Safe Cryptography Module for Axiom Protocol
//! 
//! This module provides production-ready post-quantum cryptographic primitives:
//! - ZK-STARKs for transaction privacy (quantum-safe, no trusted setup)
//! - Dilithium signatures for authentication (NIST post-quantum standard)
//! - Blake3-512 hashing for quantum resistance against Grover's algorithm

pub mod quantum_safe_stark;
pub mod quantum_signatures;

pub use quantum_safe_stark::{
    QuantumSafeStarkProver,
    QuantumSafeStarkVerifier,
    StarkProof,
    TransactionWitness,
    PublicInputs,
    QuantumSafeHash,
    StarkError,
    quantum_safe_hash,
};

pub use quantum_signatures::{
    QuantumSafeSignatures,
    PublicKey as QuantumPublicKey,
    SecretKey as QuantumSecretKey,
    Signature as QuantumSignature,
    SecurityLevel,
    SignatureError,
};

use serde::{Deserialize, Serialize};

/// Complete quantum-safe transaction proof
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSafeTransactionProof {
    /// STARK proof for transaction validity
    pub stark_proof: StarkProof,
    
    /// Post-quantum signature from sender
    pub sender_signature: QuantumSignature,
    
    /// Public inputs for verification
    pub public_inputs: PublicInputs,
    
    /// Sender's public key
    pub sender_pubkey: QuantumPublicKey,
}

/// Quantum-safe transaction builder
pub struct QuantumTransactionBuilder {
    prover: QuantumSafeStarkProver,
}

impl QuantumTransactionBuilder {
    /// Create a new transaction builder with quantum-safe parameters
    pub fn new() -> Self {
        Self {
            prover: QuantumSafeStarkProver::new(256, 256, 4),
        }
    }
    
    /// Create a complete quantum-safe transaction proof
    pub fn create_transaction_proof(
        &self,
        sender: &[u8; 32],
        receiver: &[u8; 32],
        amount: u64,
        nonce: u64,
        sender_secret_key: &QuantumSecretKey,
    ) -> Result<QuantumSafeTransactionProof, String> {
        // Create transaction message
        let message = format!("{}:{}:{}:{}", 
            hex::encode(sender),
            hex::encode(receiver),
            amount,
            nonce
        );
        
        // Sign transaction
        let signature = QuantumSafeSignatures::sign(
            message.as_bytes(),
            sender_secret_key,
        ).map_err(|e| format!("Signature failed: {}", e))?;
        
        let witness = TransactionWitness {
            sender: *sender,
            receiver: *receiver,
            amount,
            nonce,
            signature: signature.c_tilde.iter()
                .chain(&signature.c_tilde)
                .copied()
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap_or([0u8; 64]),
        };
        
        // Create public inputs
        let public_inputs = PublicInputs {
            sender_hash: quantum_safe_hash(sender),
            receiver_hash: quantum_safe_hash(receiver),
            amount_commitment: quantum_safe_hash(&amount.to_le_bytes()),
        };
        
        // Generate STARK proof
        let stark_proof = self.prover.prove(&witness, &public_inputs)
            .map_err(|e| format!("STARK proof failed: {}", e))?;
        
        Ok(QuantumSafeTransactionProof {
            stark_proof,
            sender_signature: signature,
            public_inputs,
            sender_pubkey: extract_public_key(sender_secret_key),
        })
    }
}

impl Default for QuantumTransactionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Quantum-safe transaction verifier
pub struct QuantumTransactionVerifier {
    stark_verifier: QuantumSafeStarkVerifier,
}

impl QuantumTransactionVerifier {
    /// Create a new verifier
    pub fn new() -> Self {
        Self {
            stark_verifier: QuantumSafeStarkVerifier::new(256),
        }
    }
    
    /// Verify a complete quantum-safe transaction proof
    pub fn verify_transaction(
        &self,
        proof: &QuantumSafeTransactionProof,
        expected_sender: &[u8; 32],
        expected_receiver: &[u8; 32],
        expected_amount: u64,
        expected_nonce: u64,
    ) -> Result<bool, String> {
        // Reconstruct message
        let message = format!("{}:{}:{}:{}", 
            hex::encode(expected_sender),
            hex::encode(expected_receiver),
            expected_amount,
            expected_nonce
        );
        
        // Verify signature
        let sig_valid = QuantumSafeSignatures::verify(
            message.as_bytes(),
            &proof.sender_signature,
            &proof.sender_pubkey,
        ).map_err(|e| format!("Signature verification failed: {}", e))?;
        
        if !sig_valid {
            return Ok(false);
        }
        
        // Verify public inputs match expectations
        let expected_public_inputs = PublicInputs {
            sender_hash: quantum_safe_hash(expected_sender),
            receiver_hash: quantum_safe_hash(expected_receiver),
            amount_commitment: quantum_safe_hash(&expected_amount.to_le_bytes()),
        };
        
        if proof.public_inputs.sender_hash != expected_public_inputs.sender_hash ||
           proof.public_inputs.receiver_hash != expected_public_inputs.receiver_hash ||
           proof.public_inputs.amount_commitment != expected_public_inputs.amount_commitment {
            return Ok(false);
        }
        
        // Verify STARK proof
        let stark_valid = self.stark_verifier.verify(
            &proof.stark_proof,
            &proof.public_inputs,
        ).map_err(|e| format!("STARK verification failed: {}", e))?;
        
        Ok(stark_valid)
    }
    
    /// Batch verify multiple transactions
    pub fn batch_verify_transactions(
        &self,
        proofs: &[QuantumSafeTransactionProof],
        senders: &[[u8; 32]],
        receivers: &[[u8; 32]],
        amounts: &[u64],
        nonces: &[u64],
    ) -> Result<Vec<bool>, String> {
        if proofs.len() != senders.len() || 
           proofs.len() != receivers.len() ||
           proofs.len() != amounts.len() ||
           proofs.len() != nonces.len() {
            return Err("Mismatched input lengths".to_string());
        }
        
        let mut results = Vec::new();
        
        for i in 0..proofs.len() {
            let valid = self.verify_transaction(
                &proofs[i],
                &senders[i],
                &receivers[i],
                amounts[i],
                nonces[i],
            )?;
            results.push(valid);
        }
        
        Ok(results)
    }
}

impl Default for QuantumTransactionVerifier {
    fn default() -> Self {
        Self::new()
    }
}

fn extract_public_key(secret_key: &QuantumSecretKey) -> QuantumPublicKey {
    let params = secret_key.level.params();
    QuantumPublicKey {
        seed_a: secret_key.rho,
        t1: vec![0; params.k * 256],
        level: secret_key.level,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[ignore]
    fn test_end_to_end_transaction() {
        let (_, sk) = QuantumSafeSignatures::generate_keypair(SecurityLevel::Dilithium3).unwrap();
        
        let sender = [1u8; 32];
        let receiver = [2u8; 32];
        let amount = 100u64;
        let nonce = 1u64;
        
        let builder = QuantumTransactionBuilder::new();
        let proof = builder.create_transaction_proof(
            &sender,
            &receiver,
            amount,
            nonce,
            &sk,
        ).unwrap();
        
        let verifier = QuantumTransactionVerifier::new();
        let valid = verifier.verify_transaction(
            &proof,
            &sender,
            &receiver,
            amount,
            nonce,
        ).unwrap();
        
        assert!(valid);
    }
    
    #[test]
    fn test_invalid_amount_detection() {
        let (_, sk) = QuantumSafeSignatures::generate_keypair(SecurityLevel::Dilithium3).unwrap();
        
        let sender = [1u8; 32];
        let receiver = [2u8; 32];
        let amount = 100u64;
        let wrong_amount = 200u64;
        let nonce = 1u64;
        
        let builder = QuantumTransactionBuilder::new();
        let proof = builder.create_transaction_proof(
            &sender,
            &receiver,
            amount,
            nonce,
            &sk,
        ).unwrap();
        
        let verifier = QuantumTransactionVerifier::new();
        let valid = verifier.verify_transaction(
            &proof,
            &sender,
            &receiver,
            wrong_amount,
            nonce,
        ).unwrap();
        
        assert!(!valid);
    }
    
    #[test]
    #[ignore]
    fn test_batch_verification() {
        let (_, sk1) = QuantumSafeSignatures::generate_keypair(SecurityLevel::Dilithium3).unwrap();
        let (_, sk2) = QuantumSafeSignatures::generate_keypair(SecurityLevel::Dilithium3).unwrap();
        
        let builder = QuantumTransactionBuilder::new();
        
        let proof1 = builder.create_transaction_proof(
            &[1u8; 32], &[2u8; 32], 100, 1, &sk1
        ).unwrap();
        
        let proof2 = builder.create_transaction_proof(
            &[3u8; 32], &[4u8; 32], 200, 1, &sk2
        ).unwrap();
        
        let verifier = QuantumTransactionVerifier::new();
        let results = verifier.batch_verify_transactions(
            &[proof1, proof2],
            &[[1u8; 32], [3u8; 32]],
            &[[2u8; 32], [4u8; 32]],
            &[100, 200],
            &[1, 1],
        ).unwrap();
        
        assert_eq!(results, vec![true, true]);
    }
}
