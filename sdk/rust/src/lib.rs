//! Qubit Protocol Rust SDK
//!
//! Provides a high-level interface for interacting with the Qubit blockchain:
//! - Wallet management (key generation, signing)
//! - Transaction creation and broadcasting
//! - Block and transaction queries
//! - ZK-SNARK proof generation for private transactions
//! - VDF verification
//! - Neural Guardian threat detection queries

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

/// Transaction structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,      // 64-char hex address (32 bytes)
    pub recipient: String,   // 64-char hex address
    pub amount: u64,         // Amount in satoshis (1 QBT = 10^8 sats)
    pub fee: u64,            // Transaction fee
    pub nonce: u64,          // Sender nonce to prevent replay attacks
    pub timestamp: u64,      // Unix timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,  // 128-char hex signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zk_proof: Option<String>,   // Optional ZK-SNARK proof for privacy
}

impl Transaction {
    /// Serialize transaction for signing
    pub fn serialize(&self) -> String {
        serde_json::to_string(&serde_json::json!({
            "sender": self.sender,
            "recipient": self.recipient,
            "amount": self.amount,
            "fee": self.fee,
            "nonce": self.nonce,
            "timestamp": self.timestamp,
        }))
        .unwrap()
    }

    /// Compute transaction hash (double SHA-256)
    pub fn hash(&self) -> String {
        let first_hash = Sha256::digest(self.serialize().as_bytes());
        let second_hash = Sha256::digest(&first_hash);
        hex::encode(second_hash)
    }
}


/// Block structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<serde_json::Value>,
    pub previous_hash: String,
    pub merkle_root: String,
    pub nonce: u64,
    pub difficulty: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vdf_output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vdf_proof: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}


/// Wallet for key management and signing
#[derive(Debug, Clone)]
pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
    pub address: String,
}

impl Wallet {
    /// Create new wallet with random private key
    pub fn new() -> Self {
        let private_key = Self::generate_private_key();
        let public_key = Self::derive_public_key(&private_key);
        let address = Self::derive_address(&public_key);
        
        Self {
            private_key,
            public_key,
            address,
        }
    }

    /// Create wallet from existing private key
    pub fn from_private_key(private_key: String) -> Self {
        let public_key = Self::derive_public_key(&private_key);
        let address = Self::derive_address(&public_key);
        
        Self {
            private_key,
            public_key,
            address,
        }
    }

    /// Generate random 256-bit private key
    fn generate_private_key() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let bytes: [u8; 32] = rng.gen();
        hex::encode(bytes)
    }

    /// Derive public key from private key using Ed25519
    fn derive_public_key(private_key: &str) -> String {
        // Simplified: In production, use proper Ed25519 key derivation
        let data = hex::decode(private_key).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&data);
        hasher.update(b"public");
        hex::encode(hasher.finalize())
    }

    /// Derive address from public key (SHA-256 hash)
    fn derive_address(public_key: &str) -> String {
        let data = hex::decode(public_key).unwrap();
        let hash = Sha256::digest(&data);
        hex::encode(hash)
    }

    /// Sign a message with the wallet's private key
    pub fn sign(&self, message: &str) -> String {
        // Simplified Ed25519 signature (production: use ed25519-dalek)
        // This is a simplified demo for testing the SDK structure
        let msg_hash = Sha256::digest(message.as_bytes());
        
        let mut hasher = Sha256::new();
        hasher.update(b"verify:");
        hasher.update(self.public_key.as_bytes());
        hasher.update(&msg_hash);
        let sig_data = hasher.finalize();
        
        // Pad to 64 bytes
        let mut signature = sig_data.to_vec();
        signature.extend_from_slice(&sig_data);
        hex::encode(signature)
    }

    /// Verify a signature
    pub fn verify(message: &str, signature: &str, public_key: &str) -> bool {
        // Simplified verification (production: use proper Ed25519)
        // This is a simplified demo - in production use ed25519-dalek
        let msg_hash = Sha256::digest(message.as_bytes());
        
        let mut hasher = Sha256::new();
        hasher.update(b"verify:");
        hasher.update(public_key.as_bytes());
        hasher.update(&msg_hash);
        let expected_sig = hasher.finalize();
        
        let actual_sig = hex::decode(signature).unwrap_or_default();
        if actual_sig.len() < 32 {
            return false;
        }
        expected_sig.as_slice() == &actual_sig[..32]
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}


/// Client for interacting with Qubit node RPC API
#[derive(Debug, Clone)]
pub struct QubitClient {
    node_url: String,
    client: reqwest::blocking::Client,
}

impl QubitClient {
    /// Create new client
    pub fn new(node_url: &str) -> Self {
        Self {
            node_url: node_url.trim_end_matches('/').to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Make RPC call to node
    fn rpc_call(&self, method: &str, params: serde_json::Value) -> Result<serde_json::Value, String> {
        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "id": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
            "method": method,
            "params": params,
        });

        let response = self.client
            .post(format!("{}/rpc", self.node_url))
            .json(&payload)
            .send()
            .map_err(|e| format!("Network error: {}", e))?;

        let result: serde_json::Value = response
            .json()
            .map_err(|e| format!("JSON parse error: {}", e))?;

        if let Some(error) = result.get("error") {
            if !error.is_null() {
                return Err(format!("RPC error: {}", error));
            }
        }

        Ok(result.get("result").cloned().unwrap_or(serde_json::json!({})))
    }

    /// Get balance for an address
    pub fn get_balance(&self, address: &str) -> Result<u64, String> {
        let result = self.rpc_call("get_balance", serde_json::json!({"address": address}))?;
        Ok(result.as_u64().unwrap_or(0))
    }

    /// Get current nonce for an address
    pub fn get_nonce(&self, address: &str) -> Result<u64, String> {
        let result = self.rpc_call("get_nonce", serde_json::json!({"address": address}))?;
        Ok(result.as_u64().unwrap_or(0))
    }

    /// Broadcast a signed transaction
    pub fn broadcast_transaction(&self, tx: &Transaction) -> Result<String, String> {
        let result = self.rpc_call("broadcast_transaction", serde_json::to_value(tx).unwrap())?;
        Ok(result.as_str().unwrap_or("").to_string())
    }

    /// Get transaction by hash
    pub fn get_transaction(&self, tx_hash: &str) -> Result<Option<serde_json::Value>, String> {
        match self.rpc_call("get_transaction", serde_json::json!({"hash": tx_hash})) {
            Ok(result) => Ok(Some(result)),
            Err(_) => Ok(None),
        }
    }

    /// Get block by hash or index
    pub fn get_block(&self, block_hash: Option<&str>, index: Option<u64>) -> Result<Option<Block>, String> {
        let mut params = serde_json::Map::new();
        if let Some(hash) = block_hash {
            params.insert("hash".to_string(), serde_json::json!(hash));
        } else if let Some(idx) = index {
            params.insert("index".to_string(), serde_json::json!(idx));
        } else {
            return Err("Must provide either block_hash or index".to_string());
        }

        match self.rpc_call("get_block", serde_json::Value::Object(params)) {
            Ok(result) => {
                let block: Block = serde_json::from_value(result)
                    .map_err(|e| format!("Block parse error: {}", e))?;
                Ok(Some(block))
            }
            Err(_) => Ok(None),
        }
    }

    /// Get the latest block in the chain
    pub fn get_latest_block(&self) -> Result<Block, String> {
        let result = self.rpc_call("get_latest_block", serde_json::json!({}))?;
        serde_json::from_value(result).map_err(|e| format!("Block parse error: {}", e))
    }

    /// Get blockchain info
    pub fn get_chain_info(&self) -> Result<serde_json::Value, String> {
        self.rpc_call("get_chain_info", serde_json::json!({}))
    }

    /// Create and sign a transaction
    pub fn create_transaction(
        &self,
        wallet: &Wallet,
        recipient: &str,
        amount: u64,
        fee: u64,
        use_zk: bool,
    ) -> Result<Transaction, String> {
        let nonce = self.get_nonce(&wallet.address)?;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut tx = Transaction {
            sender: wallet.address.clone(),
            recipient: recipient.to_string(),
            amount,
            fee,
            nonce,
            timestamp,
            signature: None,
            zk_proof: None,
        };

        // Sign transaction
        tx.signature = Some(wallet.sign(&tx.serialize()));

        // Generate ZK proof if requested
        if use_zk {
            tx.zk_proof = Some(self.generate_zk_proof(&tx, wallet)?);
        }

        Ok(tx)
    }

    /// Generate ZK-SNARK proof for private transaction
    fn generate_zk_proof(&self, tx: &Transaction, wallet: &Wallet) -> Result<String, String> {
        let result = self.rpc_call(
            "generate_zk_proof",
            serde_json::json!({
                "sender": tx.sender,
                "amount": tx.amount,
                "private_key": wallet.private_key,
            }),
        )?;
        Ok(result.get("proof").and_then(|p| p.as_str()).unwrap_or("").to_string())
    }

    /// Verify VDF proof
    pub fn verify_vdf(
        &self,
        vdf_output: &str,
        vdf_proof: &str,
        input_data: &str,
        time_param: u64,
    ) -> Result<bool, String> {
        let result = self.rpc_call(
            "verify_vdf",
            serde_json::json!({
                "output": vdf_output,
                "proof": vdf_proof,
                "input": input_data,
                "time": time_param,
            }),
        )?;
        Ok(result.get("valid").and_then(|v| v.as_bool()).unwrap_or(false))
    }

    /// Query Neural Guardian for threat assessment of a peer
    pub fn query_neural_guardian(&self, peer_id: &str) -> Result<serde_json::Value, String> {
        self.rpc_call("neural_guardian_query", serde_json::json!({"peer_id": peer_id}))
    }

    /// Convenience method to create, sign, and broadcast a transaction
    pub fn send(
        &self,
        wallet: &Wallet,
        recipient: &str,
        amount: u64,
        fee: u64,
        use_zk: bool,
    ) -> Result<String, String> {
        let tx = self.create_transaction(wallet, recipient, amount, fee, use_zk)?;
        self.broadcast_transaction(&tx)
    }
}


/// Convert QBT to satoshis (1 QBT = 10^8 sats)
pub fn qbt_to_sats(qbt: f64) -> u64 {
    (qbt * 100_000_000.0) as u64
}

/// Convert satoshis to QBT
pub fn sats_to_qbt(sats: u64) -> f64 {
    sats as f64 / 100_000_000.0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_creation() {
        let wallet = Wallet::new();
        assert_eq!(wallet.private_key.len(), 64);
        assert_eq!(wallet.public_key.len(), 64);
        assert_eq!(wallet.address.len(), 64);
    }

    #[test]
    fn test_signature_verification() {
        let wallet = Wallet::new();
        let message = "test message";
        let signature = wallet.sign(message);
        assert!(Wallet::verify(message, &signature, &wallet.public_key));
    }

    #[test]
    fn test_transaction_hash() {
        let tx = Transaction {
            sender: "a".repeat(64),
            recipient: "b".repeat(64),
            amount: 100,
            fee: 10,
            nonce: 1,
            timestamp: 1234567890,
            signature: None,
            zk_proof: None,
        };
        let hash = tx.hash();
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_qbt_conversion() {
        assert_eq!(qbt_to_sats(1.0), 100_000_000);
        assert_eq!(qbt_to_sats(0.5), 50_000_000);
        assert_eq!(sats_to_qbt(100_000_000), 1.0);
        assert_eq!(sats_to_qbt(50_000_000), 0.5);
    }
}
