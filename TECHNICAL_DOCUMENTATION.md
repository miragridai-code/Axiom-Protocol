# Qubit Protocol Technical Documentation

## Overview
Qubit Protocol is a decentralized blockchain platform featuring:
- Groth16 ZK-SNARK privacy for all transactions
- Wesolowski VDF-based time consensus
- Hybrid PoW mining and halving
- AI-powered attack detection and peer scoring
- libp2p networking with external validator support
- Web-based block explorer

## Architecture
- **Core Modules:**
  - `block.rs`: Block structure, validation, mining rewards
  - `transaction.rs`: Transaction format, ZK-SNARK proof, signature
  - `state.rs`: State management, nonce system, rollback
  - `vdf.rs`: Wesolowski VDF implementation
  - `ai_engine.rs`: TensorFlow integration, attack detection
  - `network.rs`: libp2p networking, validator registry
- **Explorer:**
  - Actix-web backend for blocks and state

## ZK-SNARKs
- Groth16 circuits for transaction validity and privacy
- Trusted setup ceremony required
- Proof generation and verification integrated in transaction and block logic

## VDF Consensus
- Wesolowski VDF for time-based block production
- RSA modulus and iterations configurable
- VDF proof required for block validation

## Mining & Halving
- Initial reward: 50,000,000 units
- Halving every 210,000 slots
- Rewards credited to miner address

## AI Security
- TensorFlow model for attack detection
- Peer trust scoring based on network metrics

## Networking
- libp2p for peer discovery, messaging, and validator registration
- Dynamic peer addition and external validator support

## Block Explorer
- REST API endpoints for blocks and state
- Frontend integration recommended (React, Vue, etc.)

## State Management
- Nonce system prevents replay attacks
- Snapshots and rollback for chain recovery

## Launch & Security
- Mainnet launch guide and emergency procedures
- Security audit checklist

## Testing
- Unit and integration tests required for all modules

---
For details, see `README.md`, `MAINNET_LAUNCH.md`, and `SECURITY_AUDIT.md`.