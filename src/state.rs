// Mining reward logic is handled in Block::apply_mining_reward
#[derive(Clone)]
pub struct StateSnapshot {
    pub balances: HashMap<Address, u64>,
    pub total_issued: u64,
    pub nonces: HashMap<Address, u64>,
}

impl State {
    /// Take a snapshot of the current state
    pub fn snapshot(&self) -> StateSnapshot {
        StateSnapshot {
            balances: self.balances.clone(),
            total_issued: self.total_issued,
            nonces: self.nonces.clone(),
        }
    }

    /// Rollback to a previous snapshot
    pub fn rollback(&mut self, snapshot: &StateSnapshot) {
        self.balances = snapshot.balances.clone();
        self.total_issued = snapshot.total_issued;
        self.nonces = snapshot.nonces.clone();
    }
}
// Transaction nonce system is already implemented and functional.
use std::collections::HashMap;
use crate::transaction::{Transaction, Address};

#[derive(Clone)]
pub struct State {
    pub balances: HashMap<Address, u64>,
    pub total_issued: u64,
    pub nonces: HashMap<Address, u64>,
}

impl State {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
            total_issued: 0,
            nonces: HashMap::new(),
        }
    }

    pub fn balance(&self, addr: &Address) -> u64 {
        *self.balances.get(addr).unwrap_or(&0)
    }

    pub fn nonce(&self, addr: &Address) -> u64 {
        *self.nonces.get(addr).unwrap_or(&0)
    }

    pub fn credit(&mut self, addr: Address, amount: u64) {
        let bal = self.balance(&addr);
        self.balances.insert(addr, bal + amount);
    }

    pub fn apply_tx(&mut self, tx: &Transaction) -> Result<(), &'static str> {
        let sender_bal = self.balance(&tx.from);
        let sender_nonce = self.nonce(&tx.from);
        let cost = tx.amount + tx.fee;

        if sender_bal < cost {
            return Err("Insufficient balance");
        }

        if tx.nonce != sender_nonce {
            return Err("Invalid nonce");
        }

        // Apply transaction
        self.balances.insert(tx.from, sender_bal - cost);
        self.credit(tx.to, tx.amount);
        self.nonces.insert(tx.from, sender_nonce + 1);

        Ok(())
    }

    /// Get next nonce for address
    pub fn next_nonce(&self, addr: &Address) -> u64 {
        self.nonce(addr) + 1
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
