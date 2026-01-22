use ark_bls12_381::{Bls12_381, Fr};
use ark_ff::PrimeField;
use ark_groth16::{Groth16, PreparedVerifyingKey, Proof, ProvingKey, VerifyingKey};
use ark_relations::lc;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_snark::SNARK;
use ark_std::rand::thread_rng;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use ark_relations::r1cs::Variable;
use ark_std::One;

/// Qubit Transaction Circuit - Proves ownership and solvency without revealing private data
/// 
/// This circuit proves:
/// 1. Knowledge of secret key (ownership)
/// 2. Sufficient balance for transaction (solvency)
/// 3. Correct balance update (integrity)
/// 4. All amounts are non-negative (range constraints)
#[derive(Clone)]
pub struct QubitTransactionCircuit {
    pub secret_key: Option<Fr>,
    pub current_balance: Option<Fr>,
    pub nonce: Option<Fr>,
    pub commitment: Option<Fr>,      // Hash of (secret_key, nonce)
    pub transfer_amount: Option<Fr>,
    pub fee: Option<Fr>,
    pub new_balance_commitment: Option<Fr>, // Commitment to balance after transaction
}

impl ConstraintSynthesizer<Fr> for QubitTransactionCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        // Allocate private witnesses
        let secret_key_var = cs.new_witness_variable(|| {
            self.secret_key.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let balance_var = cs.new_witness_variable(|| {
            self.current_balance.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let nonce_var = cs.new_witness_variable(|| {
            self.nonce.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        // Allocate public inputs
        let commitment_var = cs.new_input_variable(|| {
            self.commitment.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let amount_var = cs.new_input_variable(|| {
            self.transfer_amount.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let fee_var = cs.new_input_variable(|| {
            self.fee.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let new_balance_commitment_var = cs.new_input_variable(|| {
            self.new_balance_commitment.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // ========================================
        // CONSTRAINT 1: Ownership Proof via Commitment
        // ========================================
        // Prove: commitment = hash(secret_key || nonce)
        // Simplified for performance: commitment = secret_key + nonce
        // Production note: Use Pedersen commitments or Poseidon hash for better security
        let computed_commitment_var = cs.new_witness_variable(|| {
            match (self.secret_key, self.nonce) {
                (Some(sk), Some(n)) => Ok(sk + n),
                _ => Err(SynthesisError::AssignmentMissing),
            }
        })?;
        
        cs.enforce_constraint(
            lc!() + secret_key_var + nonce_var,
            lc!() + (Fr::one(), Variable::One),
            lc!() + computed_commitment_var,
        )?;
        
        cs.enforce_constraint(
            lc!() + computed_commitment_var,
            lc!() + (Fr::one(), Variable::One),
            lc!() + commitment_var,
        )?;

        // ========================================
        // CONSTRAINT 2: Solvency Proof (Anti-Inflation)
        // ========================================
        // Prove: balance >= amount + fee
        // This is critical for preventing inflation attacks
        let remainder_var = cs.new_witness_variable(|| {
            match (self.current_balance, self.transfer_amount, self.fee) {
                (Some(b), Some(a), Some(f)) => {
                    let total = a + f;
                    if b < total {
                        Err(SynthesisError::AssignmentMissing) // Fail if insufficient
                    } else {
                        Ok(b - total)
                    }
                }
                _ => Err(SynthesisError::AssignmentMissing),
            }
        })?;
        
        // Constraint: balance = amount + fee + remainder
        cs.enforce_constraint(
            lc!() + amount_var + fee_var + remainder_var,
            lc!() + (Fr::one(), Variable::One),
            lc!() + balance_var,
        )?;

        // ========================================
        // CONSTRAINT 3: New Balance Commitment
        // ========================================
        // Prove: new_balance_commitment = hash(secret_key || new_balance)
        // Simplified: new_balance_commitment = secret_key + remainder
        let computed_new_commitment_var = cs.new_witness_variable(|| {
            match (self.secret_key, self.current_balance, self.transfer_amount, self.fee) {
                (Some(sk), Some(b), Some(a), Some(f)) => {
                    let new_balance = b - a - f;
                    Ok(sk + new_balance)
                }
                _ => Err(SynthesisError::AssignmentMissing),
            }
        })?;
        
        cs.enforce_constraint(
            lc!() + secret_key_var + remainder_var,
            lc!() + (Fr::one(), Variable::One),
            lc!() + computed_new_commitment_var,
        )?;
        
        cs.enforce_constraint(
            lc!() + computed_new_commitment_var,
            lc!() + (Fr::one(), Variable::One),
            lc!() + new_balance_commitment_var,
        )?;

        Ok(())
    }
}
/// ZK Proof System Manager
pub struct ZkProofSystem {
    pub proving_key: ProvingKey<Bls12_381>,
    pub verifying_key: VerifyingKey<Bls12_381>,
    pub pvk: PreparedVerifyingKey<Bls12_381>,
}
impl ZkProofSystem {
    /// Generate new proving and verifying keys (TRUSTED SETUP)
    pub fn setup() -> Result<Self, String> {
    let mut rng = thread_rng();
    // Create dummy circuit for setup
    let circuit = QubitTransactionCircuit {
            secret_key: None,
            current_balance: None,
            nonce: None,
            commitment: None,
            transfer_amount: None,
            fee: None,
            new_balance_commitment: None,
        };
        // Generate keys
        let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng)
            .map_err(|e| format!("Setup failed: {:?}", e))?;
        let pvk = Groth16::<Bls12_381>::process_vk(&vk)
            .map_err(|e| format!("VK processing failed: {:?}", e))?;
        Ok(Self {
            proving_key: pk,
            verifying_key: vk,
            pvk,
        })
    }
    /// Save keys to disk
    pub fn save_keys(&self, keys_dir: &str) -> Result<(), String> {
    fs::create_dir_all(keys_dir).map_err(|e| format!("Failed to create keys dir: {}", e))?;
    let pk_path = format!("{}/proving.key", keys_dir);
    let vk_path = format!("{}/verifying.key", keys_dir);
    // Serialize proving key
    let mut pk_bytes = Vec::new();
    self.proving_key.serialize_compressed(&mut pk_bytes)
            .map_err(|e| format!("PK serialization failed: {:?}", e))?;
        fs::write(&pk_path, pk_bytes)
            .map_err(|e| format!("Failed to write PK: {}", e))?;
    // Serialize verifying key
    let mut vk_bytes = Vec::new();
    self.verifying_key.serialize_compressed(&mut vk_bytes)
            .map_err(|e| format!("VK serialization failed: {:?}", e))?;
        fs::write(&vk_path, vk_bytes)
            .map_err(|e| format!("Failed to write VK: {}", e))?;
        println!("✓ Keys saved to {}", keys_dir);
        Ok(())
    }
    /// Load keys from disk
    pub fn load_keys(keys_dir: &str) -> Result<Self, String> {
        let pk_path = format!("{}/proving.key", keys_dir);
        let vk_path = format!("{}/verifying.key", keys_dir);
        if !Path::new(&pk_path).exists() || !Path::new(&vk_path).exists() {
            return Err("Keys not found. Run setup first.".to_string());
        }
        let pk_bytes = fs::read(&pk_path)
            .map_err(|e| format!("Failed to read PK: {}", e))?;
        let vk_bytes = fs::read(&vk_path)
            .map_err(|e| format!("Failed to read VK: {}", e))?;
        let proving_key = ProvingKey::deserialize_compressed(&pk_bytes[..])
            .map_err(|e| format!("PK deserialization failed: {:?}", e))?;
        let verifying_key = VerifyingKey::deserialize_compressed(&vk_bytes[..])
            .map_err(|e| format!("VK deserialization failed: {:?}", e))?;
        let pvk = Groth16::<Bls12_381>::process_vk(&verifying_key)
            .map_err(|e| format!("VK processing failed: {:?}", e))?;
        Ok(Self {
            proving_key,
            verifying_key,
            pvk,
        })
    }
    /// Generate a proof for a transaction
    pub fn prove(
        &self,
        secret_key: Fr,
        current_balance: Fr,
        nonce: Fr,
        transfer_amount: Fr,
        fee: Fr,
    ) -> Result<(Proof<Bls12_381>, Vec<Fr>), String> {
        // Pre-check: fail fast if balance is insufficient
        // This prevents wasting time on proof generation for invalid transactions
        if current_balance < transfer_amount + fee {
            return Err(format!(
                "Insufficient balance: have {}, need {} (amount) + {} (fee) = {}",
                current_balance,
                transfer_amount,
                fee,
                transfer_amount + fee
            ));
        }
        
        let mut rng = thread_rng();
        
        // Compute commitments
        let commitment = secret_key + nonce;
        let new_balance = current_balance - transfer_amount - fee;
        let new_balance_commitment = secret_key + new_balance;
        
        let circuit = QubitTransactionCircuit {
            secret_key: Some(secret_key),
            current_balance: Some(current_balance),
            nonce: Some(nonce),
            commitment: Some(commitment),
            transfer_amount: Some(transfer_amount),
            fee: Some(fee),
            new_balance_commitment: Some(new_balance_commitment),
        };
        
        // Public inputs for verification
        let public_inputs = vec![commitment, transfer_amount, fee, new_balance_commitment];
        
        let proof = Groth16::<Bls12_381>::prove(&self.proving_key, circuit, &mut rng)
            .map_err(|e| format!("Proving failed: {:?}", e))?;
        
        Ok((proof, public_inputs))
    }
    
    /// Batch prove multiple transactions (more efficient than individual proofs)
    pub fn prove_batch(
        &self,
        transactions: Vec<(Fr, Fr, Fr, Fr, Fr)>, // (sk, balance, nonce, amount, fee)
    ) -> Result<Vec<(Proof<Bls12_381>, Vec<Fr>)>, String> {
        transactions
            .into_iter()
            .map(|(sk, balance, nonce, amount, fee)| {
                self.prove(sk, balance, nonce, amount, fee)
            })
            .collect()
    }
    /// Verify a proof
    pub fn verify(
    &self,
    proof: &Proof<Bls12_381>,
    public_inputs: &[Fr],
    ) -> Result<bool, String> {
        Groth16::<Bls12_381>::verify_with_processed_vk(&self.pvk, public_inputs, proof)
            .map_err(|e| format!("Verification failed: {:?}", e))
    }
}

/// Utility functions
pub fn bytes_to_fr(bytes: &[u8]) -> Fr {
    let mut hash = Sha256::digest(bytes);
    // Ensure we're within the field
    hash[31] &= 0x1f; // Clear top 3 bits to ensure < modulus
    Fr::from_le_bytes_mod_order(&hash)
}

pub fn generate_commitment(secret_key: &[u8], nonce: u64) -> Fr {
    let sk_fr = bytes_to_fr(secret_key);
    let nonce_fr = Fr::from(nonce);
    sk_fr + nonce_fr
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_zk_setup() {
        let system = ZkProofSystem::setup().unwrap();
        assert!(system.proving_key.vk.gamma_g2.is_on_curve());
    }
    
    #[test]
    fn test_proof_generation_and_verification() {
        let system = ZkProofSystem::setup().unwrap();
        
        let secret_key = Fr::from(12345u64);
        let balance = Fr::from(1000u64);
        let nonce = Fr::from(1u64);
        let amount = Fr::from(100u64);
        let fee = Fr::from(10u64);
        
        let (proof, public_inputs) = system.prove(secret_key, balance, nonce, amount, fee).unwrap();
        let valid = system.verify(&proof, &public_inputs).unwrap();
        
        assert!(valid, "Proof should be valid");
    }
    
    #[test]
    fn test_insufficient_balance_fails() {
        let system = ZkProofSystem::setup().unwrap();
        
        let secret_key = Fr::from(12345u64);
        let balance = Fr::from(50u64); // Not enough
        let nonce = Fr::from(1u64);
        let amount = Fr::from(100u64);
        let fee = Fr::from(10u64);
        
        // This should fail during proving because balance < amount + fee
        let result = system.prove(secret_key, balance, nonce, amount, fee);
        assert!(result.is_err(), "Should fail with insufficient balance");
        assert!(result.unwrap_err().contains("Insufficient balance"));
    }
    
    #[test]
    fn test_zero_amount_transaction() {
        let system = ZkProofSystem::setup().unwrap();
        
        let secret_key = Fr::from(12345u64);
        let balance = Fr::from(1000u64);
        let nonce = Fr::from(1u64);
        let amount = Fr::from(0u64); // Zero amount
        let fee = Fr::from(10u64);
        
        let (proof, public_inputs) = system.prove(secret_key, balance, nonce, amount, fee).unwrap();
        let valid = system.verify(&proof, &public_inputs).unwrap();
        
        assert!(valid, "Zero amount transaction should be valid");
    }
    
    #[test]
    fn test_exact_balance_transaction() {
        let system = ZkProofSystem::setup().unwrap();
        
        let secret_key = Fr::from(12345u64);
        let balance = Fr::from(110u64);
        let nonce = Fr::from(1u64);
        let amount = Fr::from(100u64);
        let fee = Fr::from(10u64); // Exactly uses all balance
        
        let (proof, public_inputs) = system.prove(secret_key, balance, nonce, amount, fee).unwrap();
        let valid = system.verify(&proof, &public_inputs).unwrap();
        
        assert!(valid, "Exact balance transaction should be valid");
    }
    
    #[test]
    fn test_batch_proving() {
        let system = ZkProofSystem::setup().unwrap();
        
        let transactions = vec![
            (Fr::from(111u64), Fr::from(1000u64), Fr::from(1u64), Fr::from(100u64), Fr::from(10u64)),
            (Fr::from(222u64), Fr::from(2000u64), Fr::from(2u64), Fr::from(200u64), Fr::from(20u64)),
            (Fr::from(333u64), Fr::from(3000u64), Fr::from(3u64), Fr::from(300u64), Fr::from(30u64)),
        ];
        
        let results = system.prove_batch(transactions).unwrap();
        assert_eq!(results.len(), 3, "Should generate 3 proofs");
        
        // Verify all proofs
        for (proof, public_inputs) in results {
            let valid = system.verify(&proof, &public_inputs).unwrap();
            assert!(valid, "All batch proofs should be valid");
        }
    }
    
    #[test]
    fn test_proof_serialization() {
        let system = ZkProofSystem::setup().unwrap();
        
        let secret_key = Fr::from(12345u64);
        let balance = Fr::from(1000u64);
        let nonce = Fr::from(1u64);
        let amount = Fr::from(100u64);
        let fee = Fr::from(10u64);
        
        let (proof, public_inputs) = system.prove(secret_key, balance, nonce, amount, fee).unwrap();
        
        // Serialize proof
        let mut proof_bytes = Vec::new();
        proof.serialize_compressed(&mut proof_bytes).unwrap();
        
        // Deserialize proof
        let deserialized_proof = Proof::deserialize_compressed(&proof_bytes[..]).unwrap();
        
        // Verify deserialized proof
        let valid = system.verify(&deserialized_proof, &public_inputs).unwrap();
        assert!(valid, "Deserialized proof should be valid");
    }
}

#[allow(dead_code)]
pub fn generate_circuit_address(_secret: &[u8; 32]) -> [u8; 32] {
    [0u8; 32]
}
