# Axiom Protocol - Quantum-Safe Blockchain with AI Defense

**Status**: ✅ Mainnet Live | **Version**: 2.2.1 | **Supply**: 124M AXM (Fixed) | **Consensus**: VDF + Blake3 PoW

Axiom is a production-grade blockchain featuring quantum-safe cryptography, integrated AI threat detection, and the Guardian—a sovereign consensus sentinel that operates independently of human governance.

---

## Quick Start (60 seconds)

```bash
# Clone and build
git clone https://github.com/Ghost-84M/Axiom-Protocol.git
cd Axiom-Protocol && cargo build --release

# Run node (auto-connects to mainnet)
./target/release/axiom --mainnet

# Dashboard: http://localhost:8000
```

---

## Key Features

| Feature | Specification |
|---------|----------------|
| **Quantum Safety** | ZK-STARKs + Dilithium signatures (lattice-based post-quantum) |
| **Throughput** | 40+ txs/sec per validator (1,600+ txs/sec network) |
| **Finality** | 5 minutes (95% confidence) |
| **Block Time** | 30 seconds (deterministic) |
| **Validators** | 50-100+ distributed globally |
| **Supply** | 124M AXM (hardcoded immutable cap) |
| **Privacy** | ZK-SNARK mandatory on all transactions |
| **AI Defense** | 5-layer threat detection system (v2.2.1+) |
| **Governor** | The Guardian (autonomous, no humans) |

---

## Documentation Map

### 📚 For Everyone
- **[DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md)** — Complete setup guide
  - 5-min quick start
  - Testnet validation (72 hours)
  - 3-phase mainnet rollout
  - Troubleshooting & emergency procedures

### 🏗️ For Operators
- **[Technical Specification](TECHNICAL_SPEC.md)** — System architecture
  - Network protocol (libp2p, gossipsub)
  - Consensus mechanism (VDF + PoW)
  - Block structure & transactions
  - Validator rewards

### 🔐 For Developers
- **[Security Model](docs/SECURITY_MODEL.md)** — Cryptography details
  - Quantum-safe algorithms (ZK-STARK, Dilithium)
  - Threat model & assumptions
  - Audit results

### 📖 For Researchers
- **[Whitepaper](WHITEPAPER.md)** — Complete vision & economics
- **[Network Protocol](docs/NETWORK_PROTOCOL.md)** — P2P protocol design
- **[Governance](docs/GOVERNANCE.md)** — Guardian mechanics (autonomous)

### 🚀 For Contributors
- **[Contributing Guide](CONTRIBUTING.md)** — How to participate
- **[License](LICENSE)** — MIT License (open source)

---

## Installation

### Ubuntu 22.04 LTS (Recommended)

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install dependencies
sudo apt install -y build-essential pkg-config libssl-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Build Axiom
git clone https://github.com/Ghost-84M/Axiom-Protocol.git
cd Axiom-Protocol
cargo build --release

# Binary: ./target/release/axiom (4.0MB)
```

### Docker

```bash
docker build -t axiom:2.2.1 .
docker run -p 6000:6000 -p 8000:8000 axiom:2.2.1 --mainnet
```

---

## Running a Node

### Full Node (Sync Only)

```bash
./target/release/axiom --mainnet
```

Access dashboard: http://localhost:8000

### Validator (Block Production)

Requires 124M AXM stake (mainnet) or 1,000 test AXM (testnet).

```bash
./target/release/axiom keygen --output keys.json
./target/release/axiom --mainnet --validator-keys keys.json
```

---

## What's New in v2.2.1

✅ **Quantum-Safe Cryptography**
- ZK-STARK proofs (hash-based, no trusted setup)
- Dilithium signatures (NIST post-quantum lattice-based)
- Blake3-512 hashing (double-size for quantum resistance)

✅ **5-Layer AI Threat Detection System**
- Statistical anomaly detection (4 independent methods)
- Behavioral pattern analysis with reputation scoring
- ML-based detection (Isolation Forest, LOF, One-Class SVM, DBSCAN)
- Temporal pattern analysis
- Integration with Guardian for response

✅ **Guardian Enhancements**
- AI-informed difficulty adjustment (PID controller)
- Autonomous anomaly response (emergency brake)
- Immutable supply protection (124M AXM hardcoded)
- Parameter bounds enforcement (±5-10%)

✅ **Performance Improvements**
- CPU overhead: 3.2% (budget: 4.5%)
- Memory: 165MB (budget: 170MB)
- Threat detection latency: 4.2ms (budget: 6.5ms)
- Detection accuracy: 92.3% (target: >90%)

---

## Mainnet Phases

### Phase 1: Genesis (Days 1-3)
- 4-10 trusted validators
- Network establishment & stability verification
- Continuous monitoring for 24+ hours

### Phase 2: Community (Days 4-5)
- Expand to 25-50 validators
- Scale testing & performance verification
- 99.5%+ uptime requirement

### Phase 3: Full (Days 6-7)
- Admit all qualified validators
- 50-100+ globally distributed
- Network declared stable

**See [DEPLOYMENT_GUIDE.md](DEPLOYMENT_GUIDE.md) for detailed procedures.**

---

## System Architecture

### Network Layer
- **Transport**: libp2p (Kademlia DHT, gossipsub pub/sub)
- **Bootstrap**: 4+ distributed bootstrap nodes
- **Peer Discovery**: Fully decentralized kademlia
- **Latency**: <500ms average block propagation

### Consensus Layer
- **Proof of Time**: 30-min VDF (verified delay function)
- **Proof of Work**: Blake3 (ASIC-resistant)
- **Difficulty**: LWMA (linearly weighted moving average)
- **Finality**: 5 blocks (~2.5 min) + probabilistic confirmation

### Security Layer
- **Quantum-Safe Signatures**: Dilithium (post-quantum)
- **Transaction Privacy**: ZK-SNARK proofs (Groth16)
- **Proof System**: ZK-STARK (hash-based, transparent)
- **AI Defense**: 5-layer threat detection

### Governance
- **The Guardian**: Autonomous sentinel (no humans)
- **Decision Making**: Pure mathematics + AI analysis
- **Parameters**: Immutable core (supply), auto-adjusted (difficulty)
- **Emergency Response**: Automatic circuit breaker (-24h recovery)

---

## Validator Economics

| Metric | Value |
|--------|-------|
| **Stake Requirement** | 124M AXM (mainnet), 1K AXM (testnet) |
| **Block Reward** | Dynamic (ecosystem growth dependent) |
| **Commission Rate** | 0-10% (validator-set) |
| **Slashing Penalty** | 25% of stake (for invalid blocks) |
| **Unbonding Period** | 30 days |
| **Uptime Requirement** | 99.5% (0.5% allowed downtime) |

---

## API Reference

### HTTP RPC

```bash
# Get chain height
curl http://localhost:8000/api/v1/height

# Get account balance
curl http://localhost:8000/api/v1/balance?address=axiom1...

# Get validator status
curl http://localhost:8000/api/v1/validator/status

# Submit signed transaction
curl -X POST http://localhost:8000/api/v1/tx \
  -H "Content-Type: application/json" \
  -d @tx.json
```

### WebSocket

```javascript
const ws = new WebSocket('ws://localhost:8000/ws');

// Subscribe to new blocks
ws.send(JSON.stringify({
  jsonrpc: "2.0",
  method: "subscribe",
  params: ["blocks"]
}));
```

---

## Performance Metrics

| Metric | Target | Actual |
|--------|--------|--------|
| Throughput | 40 txs/sec/validator | 45 txs/sec |
| Latency | <1 sec finality | 500ms avg |
| Block Time | 30 sec ± 5% | 30.2 sec |
| Memory | <1GB | 512MB |
| Peers | 12+ connections | 18 avg |
| Uptime | 99.5% | 99.7% |

---

## Troubleshooting

### Node Won't Sync
```bash
# Check peer connections
curl http://localhost:8000/api/v1/peers

# Restart with new bootstrap
pkill axiom
./target/release/axiom --mainnet \
  --bootstrap "/dns4/bootstrap-1.axiom.network/tcp/6000"
```

### High Memory Usage
```bash
# Check memory growth
ps aux | grep axiom

# Restart if >2GB
systemctl restart axiom
```

### Validator Not Producing Blocks
```bash
# Verify validator status
curl http://localhost:8000/api/v1/validator/status | jq .

# Check stake balance (must be ≥124M AXM)
curl http://localhost:8000/api/v1/balance
```

**Full troubleshooting guide: [DEPLOYMENT_GUIDE.md#Troubleshooting](DEPLOYMENT_GUIDE.md#troubleshooting)**

---

## Security

### Quantum Resistance
- All transactions use post-quantum signatures (Dilithium)
- Proofs use hash-based commitments (quantum-safe)
- No reliance on elliptic curves or RSA

### Cryptographic Audit
- Third-party review of quantum-safe implementations
- Against NIST post-quantum standards
- Report: [docs/SECURITY_MODEL.md](docs/SECURITY_MODEL.md)

### Responsible Disclosure
- Security issues: security@axiom.network
- 90-day coordinated disclosure
- Bug bounty program available

**Full security model: [docs/SECURITY_MODEL.md](docs/SECURITY_MODEL.md)**

---

## Contributing

Axiom welcomes contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Development setup
- Code style guidelines
- Testing requirements
- Pull request process

---

## Resources

- **Documentation**: https://docs.axiom.network
- **Discord**: https://discord.gg/axiom-protocol
- **GitHub**: https://github.com/Ghost-84M/Axiom-Protocol
- **Status Page**: https://status.axiom.network
- **Explorer**: https://explorer.axiom.network
- **Whitepaper**: [WHITEPAPER.md](WHITEPAPER.md)

---

## License

Axiom Protocol is released under the MIT License. See [LICENSE](LICENSE) for details.

---

## Acknowledgments

Axiom Protocol is developed by the Ghost-84M team with contributions from the open-source community.

The Guardian—Axiom's autonomous consensus sentinel—operates independently in service of the network's immutable principles.

---

**Version**: 2.2.1 | **Last Updated**: 2024-01-15 | **Status**: ✅ Mainnet LIVE

