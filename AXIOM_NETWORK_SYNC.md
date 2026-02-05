# AXIOM Network Synchronization - Quick Start Guide

> **Complete Guide**: For comprehensive network setup, consensus architecture, and troubleshooting, see [docs/NETWORK_CONSENSUS.md](docs/NETWORK_CONSENSUS.md)

## ⚡ 30-Second Quick Start

```bash
# Build the node
cargo build --release

# Run (automatically connects to mainnet bootstrap)
./target/release/axiom-node

# Verify it's syncing
axiom-node status
# Output: Connected Peers: 1+ | Height: Growing | Sync: IN SYNC ✅
```

**Done!** Your node automatically connects to `34.10.172.20:6000` and syncs the blockchain.

---

## Configuration Methods

### 1. Default (config/bootstrap.toml)
```bash
cargo run --release
# Automatically uses config/bootstrap.toml
```

### 2. Environment Variable
```bash
export AXIOM_BOOTSTRAP_PEERS="/ip4/34.10.172.20/tcp/6000"
cargo run --release
```

### 3. All Genesis Miners (5 nodes)
```bash
export AXIOM_BOOTSTRAP_PEERS="192.168.1.100:6000,192.168.1.101:6000,192.168.1.102:6000,192.168.1.103:6000,192.168.1.104:6000"
cargo run --release
```

---

## Verify Syncing

```bash
# Check node status
axiom-node status

# Check connected peers
axiom-node peers

# Continuous monitoring
watch -n 5 'axiom-node status'
```

---

## If Node Doesn't Connect

1. **Check bootstrap connectivity**:
   ```bash
   telnet 34.10.172.20 6000
   # Should connect (Ctrl+] then quit)
   ```

2. **Check firewall**:
   ```bash
   sudo ufw allow 6000/tcp
   ```

3. **See detailed troubleshooting**: [docs/NETWORK_CONSENSUS.md](docs/NETWORK_CONSENSUS.md#troubleshooting)

---

## 🔗 Bootstrap Node Details

- **IP**: 34.10.172.20
- **Port**: 6000  
- **PeerId**: 12D3KooWAzD3QjhHMamey1XuysPovzwXyAZy9VzpZmQN7GkrURWU
- **Region**: GCP (Google Cloud Platform)
- **Uptime SLA**: 99.9%

---

## Emergency Recovery

If your node has forked from mainnet (different chain history):

```bash
# Delete local chain
pkill axiom-node
rm -rf ~/.axiom/blocks/

# Restart (will sync fresh from bootstrap)
axiom-node
```

---

## For Genesis Miners (5-Node Setup)

See comprehensive guide: [docs/NETWORK_CONSENSUS.md](docs/NETWORK_CONSENSUS.md)


```
🌍 Bootstrap Configuration:
   📌 Using config/bootstrap.toml with server bootstrap node
🔗 Peer connected: 12D3KooWAzD3QjhHMamey1XuysPovzwXyAZy9VzpZmQN7GkrURWU
⛓️  Height: X | Diff: 1000
🌐 Network Status:
   ├─ Connected Peers: 1
   └─ Address: /ip4/34.10.172.20/tcp/6000/p2p/12D3KooWAzD3QjhHMamey1XuysPovzwXyAZy9VzpZmQN7GkrURWU
```

---

### Method 2: Interactive Setup Script

```bash
bash connect_to_bootstrap.sh
```

Choose option 1 to auto-update your config, and the script handles everything.

---

### Method 3: Environment Variable Override

```bash
export AXIOM_BOOTSTRAP_PEERS="/ip4/34.10.172.20/tcp/6000/p2p/12D3KooWAzD3QjhHMamey1XuysPovzwXyAZy9VzpZmQN7GkrURWU"
cargo run --release
```

---

### Method 4: Docker

```bash
docker build -t axiom-node .
docker run -e AXIOM_BOOTSTRAP_PEERS="/ip4/34.10.172.20/tcp/6000/p2p/12D3KooWAzD3QjhHMamey1XuysPovzwXyAZy9VzpZmQN7GkrURWU" axiom-node
```

---

### Method 5: Kubernetes

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: axiom-bootstrap
data:
  AXIOM_BOOTSTRAP_PEERS: "/ip4/34.10.172.20/tcp/6000/p2p/12D3KooWAzD3QjhHMamey1XuysPovzwXyAZy9VzpZmQN7GkrURWU"
---
apiVersion: v1
kind: Pod
metadata:
  name: axiom-node
spec:
  containers:
  - name: axiom
    image: axiom-node:latest
    envFrom:
    - configMapRef:
        name: axiom-bootstrap
```

---

## Network Synchronization Flow

### Initial Connection

```
Your Node                          Bootstrap Node (34.10.172.20:6000)
    |                                       |
    +------- TCP Connect Port 6000 ------->|
    |                                       |
    |<----- Peer ID & Address Info --------|
    |                                       |
    +------- Request Chain Data ---------->|
    |                                       |
    |<----- Send Blocks (1 to N) ----------|
    |                                       |
    +------- Acknowledgement ------------->|
    |                                       |
    ✅ SYNCED                              ✅ CONNECTED
```

### Continuous Synchronization

```
Time T₀: Your Node Height = 1
         Bootstrap Node Height = 10
         
Time T₁: Your Node Height = 5 (syncing...)
         Bootstrap Node Height = 10
         
Time T₂: Your Node Height = 10 (synced!)
         Bootstrap Node Height = 10
         
Time T₃: New Block Mined
         Your Node Height = 11
         Bootstrap Node Height = 11
         ✅ FULLY SYNCHRONIZED
```

---

## Verifying Synchronization

### Check Your Node's Status

```bash
# View logs while running
tail -f ~/.axiom/logs.txt

# Look for these indicators:
# "Peer connected: 12D3KooW..."     → Connected to bootstrap
# "New height: X"                    → Syncing blocks
# "Synced complete chain"            → Sync complete
```

### Monitor in Real-Time

```bash
# Start your node in one terminal
./target/release/axiom

# In another terminal, check peer status
watch -n 5 'ps aux | grep axiom'
```

### Expected Synchronization Stages

```
Stage 1: Connection
---------
🔍 Checking bootstrap connectivity...
🔗 Peer connected: 12D3KooWAzD3QjhHMamey1XuysPovzwXyAZy9VzpZmQN7GkrURWU
✅ Bootstrap connected

Stage 2: Block Exchange
---------
📥 Requesting chain from peer: 12D3KooW...
🔁 Synced complete chain from peer. New height: X
✅ Blocks received

Stage 3: Synchronized
---------
⛓️  Height: X | Diff: 1000 | Trend: STABLE ↔️
🌐 Connected Peers: 1
✅ Fully synchronized
```

---

## Troubleshooting Synchronization

### Issue: Can't Connect to Bootstrap

```bash
# Check firewall
sudo ufw status | grep 6000

# Verify connectivity
ping 34.10.172.20
telnet 34.10.172.20 6000

# If telnet succeeds: Connected!
# If telnet fails: Check:
#   1. GCP firewall rules allow port 6000
#   2. Bootstrap node is running
#   3. Your network allows outbound port 6000
```

### Issue: Connected but Not Syncing

```bash
# Check your node logs
tail -f ~/.axiom/logs.txt | grep -E "Height|Peer|Sync"

# Possible causes:
# 1. Bootstrap node is catching up (wait a bit)
# 2. Network latency (usually resolves itself)
# 3. Firewall blocking responses (check inbound rules)
```

### Issue: Height Stuck at 1

```bash
# The bootstrap node may still be mining initial blocks
# Wait for bootstrap node to catch up:

# SSH into bootstrap server
ssh user@34.10.172.20

# Check bootstrap node status
sudo journalctl -u axiom-bootstrap -f | grep Height

# Wait until Height increases
```

### Issue: Peer Connected but Height Not Changing

```bash
# This may indicate network partition
# Try these steps:

# 1. Check if bootstrap node is healthy
ping 34.10.172.20

# 2. Restart your node
pkill -f "target/release/axiom"
sleep 2
./target/release/axiom

# 3. Monitor logs
tail -f ~/.axiom/logs.txt
```

---

## Network Statistics

### Bootstrap Node Specs

```
├─ CPU:           2-4 cores (Google Cloud n1-standard-2)
├─ Memory:        8 GB RAM
├─ Disk:          50 GB SSD (for blockchain state)
├─ Bandwidth:     10+ Mbps
├─ Network:       Public IP 34.10.172.20
├─ Uptime:        24/7 (systemd service)
└─ Sync Status:   ✅ ACTIVE
```

### Expected Node Performance

```
├─ Connection Time:       < 5 seconds
├─ Block Sync Time:       Depends on network (1000 blocks ≈ few minutes)
├─ CPU Usage:             50-100% during sync, 10-20% idle
├─ Memory Usage:          100-300 MB during operation
├─ Disk I/O:             High during sync, low during mining
└─ Network Bandwidth:    2-5 Mbps during sync, <0.5 Mbps idle
```

---

## Multi-Node Setup

### Running Multiple Nodes on Same Machine

**Node 1 (Bootstrap):**
```bash
./target/release/axiom           # Uses port 6000
```

**Node 2 (Client, in another terminal):**
```bash
export AXIOM_BOOTSTRAP_PEERS="/ip4/34.10.172.20/tcp/6000/p2p/12D3KooWAzD3QjhHMamey1XuysPovzwXyAZy9VzpZmQN7GkrURWU"
./target/release/axiom           # Auto-selects port 6001
```

**Node 3 (Client, in another terminal):**
```bash
export AXIOM_BOOTSTRAP_PEERS="/ip4/34.10.172.20/tcp/6000/p2p/12D3KooWAzD3QjhHMamey1XuysPovzwXyAZy9VzpZmQN7GkrURWU"
./target/release/axiom           # Auto-selects port 6002
```

All three nodes will sync with the bootstrap node!

---

## Automation Scripts

### Check Sync Status

```bash
#!/bin/bash
# save as: check_sync.sh

while true; do
    clear
    echo "=== AXIOM NODE SYNC STATUS ==="
    echo "Timestamp: $(date)"
    echo ""
    tail -n 50 ~/.axiom/logs.txt | grep -E "Height:|Peer|Synced" || echo "Logs not available"
    sleep 5
done
```

### Auto-Restart on Crash

```bash
#!/bin/bash
# save as: keep_running.sh

while true; do
    ./target/release/axiom
    echo "Node stopped, restarting in 5 seconds..."
    sleep 5
done
```

---

## Production Checklist

- [ ] Bootstrap node running with systemd service
- [ ] All nodes configured with bootstrap IP: 34.10.172.20
- [ ] All nodes using port: 6000
- [ ] Firewall allows port 6000 (TCP)
- [ ] All nodes show "Peer connected" in logs
- [ ] All nodes have matching Height values
- [ ] Monitoring/alerting configured
- [ ] Backup system in place
- [ ] Logs being archived

---

## Support & Monitoring

### View Detailed Logs

```bash
# Bootstrap node (on server)
sudo journalctl -u axiom-bootstrap -f --lines=100

# Local node
tail -f ~/.axiom/logs.txt
```

### Monitor Network Health

```bash
# Check connected peers
grep "Peer connected" ~/.axiom/logs.txt | wc -l

# Check sync progress
grep "Height:" ~/.axiom/logs.txt | tail -20
```

### Contact Support

If issues persist:
1. Check BOOTSTRAP_DEPLOYMENT.md for detailed instructions
2. Review logs for error messages
3. Verify network connectivity to 34.10.172.20:6000
4. Open GitHub issue: https://github.com/Ghost-84M/Axiom-Protocol

---

**Last Updated:** February 5, 2026  
**Axiom Protocol Version:** 2.0.0  
**Bootstrap Node Status:** ✅ Active & Synchronized  
**Network:** Mainnet (GCP)
