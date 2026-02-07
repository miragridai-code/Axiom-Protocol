# Axiom Protocol v2.2.1 Deployment Guide

**Last Updated**: 2024 | **Version**: 2.2.1 | **Status**: Production Ready

---

## Quick Start

Deploy a single Axiom node in 5 minutes:

```bash
# 1. Clone repository
git clone https://github.com/Ghost-84M/Axiom-Protocol.git
cd Axiom-Protocol

# 2. Build release binary
cargo build --release

# 3. Run node with bootstrap connectivity
./target/release/axiom --bootstrap "/dns4/bootstrap-1.axiom.network/tcp/6000"

# Node starts listening on 127.0.0.1:6000
# Dashboard available at http://localhost:8000
```

---

## System Requirements

### Minimum (Testnet)
- **CPU**: 2 cores (1GHz+)
- **RAM**: 2GB
- **Storage**: 20GB SSD
- **Network**: 10 Mbps (100 kbps continuous)
- **OS**: Linux (Ubuntu 20.04+), macOS 12+, or Windows WSL2

### Recommended (Validator)
- **CPU**: 8 cores Intel/AMD (2.4GHz+)
- **RAM**: 16GB
- **Storage**: 100GB NVMe SSD
- **Network**: 100+ Mbps (1 Mbps continuous)
- **OS**: Linux (Ubuntu 22.04 LTS preferred)
- **Uptime**: 99.5% (with redundant connections)

### Production (Genesis Node)
- **CPU**: 16+ cores (3.0GHz+)
- **RAM**: 64GB
- **Storage**: 500GB NVMe SSD (RAID-1 mirrored)
- **Network**: 1Gbps dedicated connection
- **Cooling**: Active cooling required
- **Power**: UPS with 4-hour backup minimum
- **Redundancy**: Dual network interfaces, geographic diversity

---

## Installation

### Ubuntu 22.04 LTS (Recommended)

```bash
# 1. Update system
sudo apt update && sudo apt upgrade -y

# 2. Install dependencies
sudo apt install -y build-essential pkg-config libssl-dev

# 3. Install Rust (if not present)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 4. Clone and build
git clone https://github.com/Ghost-84M/Axiom-Protocol.git
cd Axiom-Protocol
cargo build --release

# Build takes 2-3 minutes on modern hardware
# Binary: ./target/release/axiom (4.0MB)
```

### macOS (M1/M2/Intel)

```bash
# 1. Install Homebrew dependencies
brew install pkg-config openssl

# 2. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. Build (same as Linux)
git clone https://github.com/Ghost-84M/Axiom-Protocol.git
cd Axiom-Protocol
cargo build --release
```

### Docker

```bash
# Build image
docker build -t axiom:2.2.1 .

# Run containerized node
docker run -d \
  --name axiom-node \
  -p 6000:6000 \
  -p 8000:8000 \
  -v axiom-data:/root/.axiom \
  axiom:2.2.1 \
  --bootstrap "/dns4/bootstrap-1.axiom.network/tcp/6000"

# View logs
docker logs -f axiom-node

# Stop node
docker stop axiom-node
```

---

## Node Deployment

### Run a Full Node (Sync Only)

```bash
./target/release/axiom \
  --bootstrap "/dns4/bootstrap-1.axiom.network/tcp/6000" \
  --bootstrap "/dns4/bootstrap-2.axiom.network/tcp/6000"
```

**Output**:
```
[2024-01-15T10:30:45Z] Axiom Node v2.2.1
[2024-01-15T10:30:46Z] Network initialized
[2024-01-15T10:30:47Z] Listening on /ip4/127.0.0.1:6000
[2024-01-15T10:30:48Z] AI Guardian activated
[2024-01-15T10:30:49Z] Syncing blockchain... (height: 0)
```

**Access Dashboard**:
```
Browser: http://localhost:8000
API: http://localhost:8000/api/v1/
WebSocket: ws://localhost:8000/ws
```

### Run as Validator (Block Production)

Validators produce blocks and secure the network. Requirements:

1. **Full 124M AXM stake** held in your validator account
2. **Uptime commitment**: 99.5%+ 24/7
3. **Mainnet readiness**: 7-day testnet run minimum

```bash
# Generate validator keypair
./target/release/axiom keygen --output validator-keys.json

# Run validator node
./target/release/axiom \
  --validator-keys validator-keys.json \
  --bootstrap "/dns4/bootstrap-1.axiom.network/tcp/6000"
```

**Validator Status** (check via API):
```bash
# Check validator status
curl http://localhost:8000/api/v1/validator/status

# Response:
# {
#   "address": "axiom1a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0",
#   "is_active": true,
#   "blocks_produced": 1,
#   "stake": 124000000,
#   "uptime_percent": 99.8
# }
```

---

## Testnet Deployment

Deploy test validators on Axiom Testnet for validation before mainnet.

### Phase 1: Setup (24 hours before)

1. **Prepare infrastructure**:
   - Provision VM/server
   - Install dependencies
   - Build binary
   - Test network connectivity

2. **Verify connectivity**:
   ```bash
   # Test bootstrap connections
   nslookup bootstrap-1.axiom.network
   nslookup bootstrap-2.axiom.network
   
   # Test firewall
   telnet bootstrap-1.axiom.network 6000
   ```

3. **Obtain testnet AXM**:
   - Request from faucet: https://testnet.axiom.network/faucet
   - Receive 1,000 test AXM (no mainnet value)
   - Verify receipt (`curl http://localhost:8000/api/v1/balance`)

### Phase 2: Validator Onboarding (48 hours)

```bash
# 1. Start testnet validator
./target/release/axiom \
  --testnet \
  --validator-keys testnet-keys.json \
  --bootstrap "/dns4/testnet-boot-1.axiom.network/tcp/6000"

# 2. Monitor for 24 hours
watch -n 5 'curl -s http://localhost:8000/api/v1/validator/status | jq'

# 3. Verify metrics
# - Block production: ≥1 block/10min
# - Uptime: ≥99%
# - Peer connections: ≥8
# - Chain height: Growing

# 4. Check logs for errors
tail -f /tmp/axiom.log | grep -E "ERROR|WARN"
```

### Phase 3: Stability Testing (72 hours)

```bash
# Monitor continuously during peak/off-peak hours
- Peak: 10k txs/sec handling
- Off-peak: Idle block production
- Network: Peer discovery, block propagation

# Success criteria:
- ✓ 99.5%+ uptime (1 outage max per 200 hours)
- ✓ Block gaps: None
- ✓ Memory stable: <500MB average
- ✓ CPU: <50% sustained
- ✓ Peers: 12+ connections maintained
```

---

## Mainnet Rollout

Axiom mainnet deploys in 3 phases over 7 days. All genesis validators participate in all phases.

### Phase 1: Genesis Validators Only (Days 1-3)

**Objective**: Establish network with 4-10 trusted validators.

**Participants**: Axiom core team + selected early validators

```bash
# 1. Bootstrap genesis block at UTC 2024-02-01T00:00:00Z
# (Network waits for 51% validator participation before proceeding)

# 2. Run genesis node
./target/release/axiom \
  --mainnet \
  --genesis-time 2024-02-01T00:00:00Z \
  --validator-keys mainnet-keys.json \
  --listen-addr 0.0.0.0:6000

# 3. Verify participation
curl http://localhost:8000/api/v1/genesis/status
# Expected:
# {
#   "participating_validators": 6,
#   "required_validators": 4,
#   "genesis_blocked": false,
#   "expected_block_1_time": "2024-02-01T00:10:00Z"
# }
```

**Success Criteria**:
- ✓ Block #1 produced at T+10min
- ✓ All 4+ genesis validators active
- ✓ 20 blocks produced (10 minutes) without gaps
- ✓ Transaction propagation <500ms
- ✓ No consensus faults

**Monitoring** (run continuously):
```bash
# Health check every 10 seconds
while true; do
  curl -s http://localhost:8000/api/v1/health | jq .
  sleep 10
done

# Expected output:
# {
#   "status": "healthy",
#   "chain_height": 180,
#   "block_time_average": 30.2,
#   "peers": 9,
#   "synced": true
# }
```

### Phase 2: Community Expansion (Days 4-5)

**Objective**: Scale to 50% of target validator set (25-50 validators).

**Timeline**:
- Day 4 08:00 UTC: Begin accepting new validator stakes
- Day 4 20:00 UTC: First 25% admitted (~12-15 validators)
- Day 5 08:00 UTC: Next 25% admitted (total ~50% = 25-40 validators)

```bash
# Community validators: Submit stake transaction
curl -X POST http://mainnet-validator-portal.axiom.network/api/stake \
  -H "Content-Type: application/json" \
  -d '{
    "validator_address": "axiom1...",
    "stake_amount": 124000000,
    "commission_rate": 0.05
  }'

# System waits for transaction finality (~1 minute)
# Then admits validator to active set
```

**Admission Process**:
1. Validator submits stake transaction (124M AXM)
2. Network verifies balance and valid signature
3. Validator joins consensus immediately
4. Starts producing blocks in next slot

**Expected Network State (Day 5)**:
- Validators: 25-40 active
- Transactions/sec: 800-1,200 (50% capacity)
- Block time: 30 seconds (stable)
- Finality: 5-10 blocks back (~3-5 min)
- Network resilience: Tolerates 1/3 validator failure

### Phase 3: Full Network (Days 6-7)

**Objective**: Admit all remaining validators, reach 100% distributed network.

**Timeline**:
- Day 6 08:00 UTC: Open validator admission to all (50M+ AXM stake required)
- Day 7 00:00 UTC: Close admission window (network stabilizes)
- Day 7 12:00 UTC: Declare mainnet stable

```bash
# Anyone with 50M+ AXM can validate
# Remaining Phase 2 validators automatically admitted
# New validators follow same admission process

# Expected: 50-100+ validators by Day 7
```

**Final Network Specifications**:
- **Validators**: 50-100+ distributed globally
- **Throughput**: 1,600+ txs/sec (40+ per validator)
- **Latency**: 100-500ms block propagation
- **Finality**: 5 minutes (95% confidence)
- **Supply**: 124M AXM (immutable, hardcoded)
- **Decentralization**: No single entity >20% stake

---

## Integration Guide

### Connect External Wallet/Exchange

```bash
# 1. Run full node (archive mode, retains all history)
./target/release/axiom \
  --mainnet \
  --archive-mode \
  --listen-addr 0.0.0.0:6000 \
  --rpc-addr http://0.0.0.0:8000

# 2. Expose RPC API (with rate limiting)
# Behind reverse proxy:
#   - Rate limit: 100 req/sec per IP
#   - Timeout: 30 seconds
#   - Allow methods: eth_*, web3_*, net_*

# 3. Standard EVM RPC endpoints:
# - http://node-ip:8000/api/v1/eth
# - Supports: eth_blockNumber, eth_getBalance, eth_sendTransaction, etc.
```

### Bridge Token Flow

1. **User locks AXM on Axiom mainnet**:
   ```bash
   curl -X POST http://localhost:8000/api/v1/bridge/lock \
     -d '{"amount": 100000000, "destination_chain": "ethereum"}'
   ```

2. **Bridge validator signs multi-sig transaction** (3-of-5 required)

3. **User collects 3+ signatures**, submits to Ethereum contract

4. **Ethereum contract releases equivalent xAXM token**

5. **User receives xAXM on Ethereum**

---

## Performance Specifications

| Metric | Target | Measured |
|--------|--------|----------|
| **Throughput** | 40 txs/sec per core | 45 txs/sec (8-core) |
| **Latency** | <1 second finality | 500ms average |
| **Block Time** | 30 seconds | 30.2 seconds |
| **Block Size** | 5-10MB | 8.5MB average |
| **Validator Count** | 50-100+ | Grows with Phase 3 |
| **Network Sync** | <1 hour from genesis | 45 min (1GB network) |
| **Memory Usage** | <1GB full node | 512MB (cached) |
| **Disk I/O** | <500 IOPS sustained | 380 IOPS measured |

---

## Troubleshooting

### Node Fails to Start

```bash
# 1. Verify ports available
lsof -i :6000
lsof -i :8000

# 2. Check system resources
free -h
df -h

# 3. Clear cache and retry
rm -rf ~/.axiom/database
./target/release/axiom --mainnet
```

### Out of Sync (chain height not advancing)

```bash
# 1. Check peer connections
curl http://localhost:8000/api/v1/peers

# 2. If <8 peers, restart with new bootstrap nodes
pkill axiom
./target/release/axiom \
  --mainnet \
  --bootstrap "/dns4/bootstrap-1.axiom.network/tcp/6000" \
  --bootstrap "/dns4/bootstrap-2.axiom.network/tcp/6000"
```

### High Memory Usage

```bash
# 1. Check memory growth
watch -n 5 'ps aux | grep axiom'

# 2. If >2GB, likely cache leak - restart
systemctl restart axiom

# 3. Enable memory limits (systemd)
[Service]
MemoryMax=1G
MemoryHigh=900M
```

### Validator Not Producing Blocks

```bash
# 1. Verify validator keys loaded
curl http://localhost:8000/api/v1/validator/status | jq .address

# 2. Check stake balance
curl http://localhost:8000/api/v1/balance | jq .

# Must have: ≥124M AXM

# 3. Check if active
curl http://localhost:8000/api/v1/validator/status | jq .is_active

# 4. Restart if inactive
systemctl restart axiom
```

---

## Emergency Procedures

### Network Fork or Consensus Failure

**If >33% validators produce conflicting blocks**:

```bash
# 1. STOP your validator immediately
systemctl stop axiom

# 2. Broadcast alert to chain
# (Messaging protocol for validators)

# 3. Wait for on-chain governance vote (10 blocks = 5 minutes)

# 4. Download canonical chain state
curl https://checkpoints.axiom.network/mainnet/latest

# 5. Restart validator with canonical state
rm -rf ~/.axiom/database/*
# (Restore from checkpoint)
systemctl start axiom
```

### Validator Isolation (DDoS/Attack)

**If you cannot reach bootstrap nodes**:

```bash
# 1. Fall back to trusted peer list
./target/release/axiom \
  --mainnet \
  --validator-keys keys.json \
  --trusted-peer /ip4/10.0.1.5/tcp/6000/p2p/Qm... \
  --trusted-peer /ip4/10.0.1.6/tcp/6000/p2p/Qm...

# 2. Verify at least 2 trusted peers reachable
# System pauses block production until threshold met

# 3. Contact validators on emergency channel
# (Discord #emergency, direct messages)
```

### Validator Slashing (Invalid Block)

**If you published invalid block (cryptographic error)**:

```bash
# 1. Your 124M stake automatically slashed
# 2. Kicked from active validator set
# 3. Can re-stake after cooling period (30 days)
# 4. Review logs to understand failure:

grep -i "invalid" /tmp/axiom.log | tail -50
```

---

## Deployment Checklist

### Pre-Deployment (1 week before)

- [ ] Infrastructure provisioned (CPU, RAM, network)
- [ ] OS installed and hardened (Ubuntu 22.04 LTS)
- [ ] Firewall rules configured (6000, 8000 open)
- [ ] Backup strategy defined (daily snapshots)
- [ ] Monitoring setup (Prometheus, alerting)
- [ ] Validator keypair generated and backed up (4+ redundant copies)
- [ ] 124M AXM stake prepared and accessible
- [ ] Testnet run completed (7+ days)

### Day of Deployment

- [ ] Time synchronized with NTP (`ntpq -p`)
- [ ] Binary built fresh: `cargo build --release`
- [ ] Binary checksum verified: `sha256sum axiom`
- [ ] Bootstrap connectivity verified
- [ ] Binary started successfully
- [ ] Dashboard accessible at http://localhost:8000
- [ ] Logs monitored for errors (tail -f)
- [ ] Peer connections established (≥8 peers)

### First 24 Hours

- [ ] Block production confirmed (≥1 block every 10 min)
- [ ] Uptime: 100% (zero restarts)
- [ ] Memory stable (<500MB)
- [ ] CPU usage: <50%
- [ ] Peer count: 12+
- [ ] All health checks passing

### First Week

- [ ] Continuous uptime: ≥99.5%
- [ ] No missed blocks
- [ ] Validator rewards accruing
- [ ] Daily log review: no errors
- [ ] Weekly checkpoint backup

---

## Support & Resources

- **Discord**: https://discord.gg/axiom-protocol
- **Docs**: https://docs.axiom.network
- **GitHub**: https://github.com/Ghost-84M/Axiom-Protocol
- **Status Page**: https://status.axiom.network
- **Security Report**: https://axiom.network/security

---

**Version**: 2.2.1 | **Last Updated**: 2024 | **Mainnet Status**: LIVE
