#!/bin/bash
# AXIOM Protocol - Validator Node Deployment Script
# Deploys a mainnet validator with archive mode and full monitoring

set -e

echo "╔═══════════════════════════════════════════════════════════════════════╗"
echo "║         AXIOM Protocol - Validator Node Deployment                   ║"
echo "║                     Mainnet Network ID: 1                             ║"
echo "╚═══════════════════════════════════════════════════════════════════════╝"
echo ""

# Color codes for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Configuration
VALIDATOR_NAME="${VALIDATOR_NAME:-axiom-validator-1}"
VALIDATOR_ADDRESS="${VALIDATOR_ADDRESS:-}"
DATA_DIR="${DATA_DIR:-./axiom-validator-data}"
RPC_PORT="${RPC_PORT:-8546}"
P2P_PORT="${P2P_PORT:-8545}"
METRICS_PORT="${METRICS_PORT:-9100}"

echo -e "${YELLOW}[1/10] Pre-flight Checks${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check if binary exists
if [ ! -f "target/release/qubit" ]; then
    echo -e "${RED}✗ Binary not found. Building release version...${NC}"
    cargo build --release
else
    echo -e "${GREEN}✓ Binary found: target/release/qubit${NC}"
fi

# Check system requirements
AVAILABLE_DISK=$(df -BG . | tail -1 | awk '{print $4}' | sed 's/G//')
AVAILABLE_RAM=$(free -g | awk '/^Mem:/{print $7}')

echo -e "${GREEN}✓ Available disk space: ${AVAILABLE_DISK}GB${NC}"
echo -e "${GREEN}✓ Available RAM: ${AVAILABLE_RAM}GB${NC}"

if [ "$AVAILABLE_DISK" -lt 50 ]; then
    echo -e "${YELLOW}⚠ Warning: Less than 50GB disk space. Validators need 50GB+ for archive mode.${NC}"
fi

if [ "$AVAILABLE_RAM" -lt 8 ]; then
    echo -e "${YELLOW}⚠ Warning: Less than 8GB RAM available. Recommended: 8GB+${NC}"
fi

echo ""
echo -e "${YELLOW}[2/10] Validator Configuration${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Prompt for validator address if not set
if [ -z "$VALIDATOR_ADDRESS" ]; then
    echo -e "${YELLOW}Validator address not set. Generate one? (y/n)${NC}"
    read -r GENERATE_ADDRESS
    
    if [ "$GENERATE_ADDRESS" = "y" ] || [ "$GENERATE_ADDRESS" = "Y" ]; then
        echo -e "${GREEN}✓ Generating new validator address...${NC}"
        VALIDATOR_ADDRESS=$(./target/release/qubit-wallet create --output address-only 2>/dev/null || echo "axm1validator$(openssl rand -hex 20)")
        echo -e "${GREEN}✓ Generated: $VALIDATOR_ADDRESS${NC}"
        echo -e "${YELLOW}⚠ IMPORTANT: Save this address and backup your keystore!${NC}"
    else
        echo -e "${YELLOW}Enter your validator address (axm1...):${NC}"
        read -r VALIDATOR_ADDRESS
    fi
fi

echo -e "${GREEN}✓ Validator Name: $VALIDATOR_NAME${NC}"
echo -e "${GREEN}✓ Validator Address: $VALIDATOR_ADDRESS${NC}"
echo -e "${GREEN}✓ Data Directory: $DATA_DIR${NC}"
echo -e "${GREEN}✓ RPC Port: $RPC_PORT${NC}"
echo -e "${GREEN}✓ P2P Port: $P2P_PORT${NC}"

echo ""
echo -e "${YELLOW}[3/10] Creating Validator Configuration${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Create validator-specific config
cat > axiom-validator.toml << EOF
# AXIOM Protocol - Validator Configuration
# Network: Mainnet (ID: 1)
# Role: Archive validator with full history

[node]
name = "$VALIDATOR_NAME"
node_type = "archive"
metrics_enabled = true

[network]
listen_address = "/ip4/0.0.0.0/tcp/$P2P_PORT"
bootstrap_peers = []
max_peers = 100
max_inbound_peers = 60
max_outbound_peers = 40
enable_mdns = true
enable_kademlia = true
connection_timeout = 30
gossip_heartbeat = 1
network_id = 1  # Mainnet

[consensus]
vdf_steps = 3600000
pow_difficulty = 1000
block_time_seconds = 3600
difficulty_adjustment_interval = 2016
max_block_size = 1000000
max_transactions_per_block = 10000
min_transaction_fee = 100000000
confirmation_depth = 6

[mining]
enabled = true
threads = 4
miner_address = "$VALIDATOR_ADDRESS"
intensity = 100
min_peers_to_mine = 10

[storage]
data_dir = "$DATA_DIR"
cache_size_mb = 2048
compression = true
pruning = "archive"
max_db_size_gb = 0  # Unlimited for validators

[ai]
neural_guardian_enabled = true
threat_threshold = 0.7
model_update_interval = 86400
oracle_enabled = false
min_oracle_stake = 50000000000
oracle_consensus_threshold = 3

[rpc]
enabled = true
listen_address = "0.0.0.0:$RPC_PORT"
cors_allowed_origins = ["*"]
max_connections = 1000
request_timeout = 30
websocket_enabled = true
rate_limit = 200

[logging]
level = "info"
file_enabled = true
log_file = "$VALIDATOR_NAME.log"
max_file_size_mb = 100
max_backups = 10
json_format = false
colored = true
EOF

echo -e "${GREEN}✓ Created: axiom-validator.toml${NC}"

echo ""
echo -e "${YELLOW}[4/10] Initializing Data Directory${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

mkdir -p "$DATA_DIR"
mkdir -p "$DATA_DIR/blocks"
mkdir -p "$DATA_DIR/state"
mkdir -p "$DATA_DIR/chain"
mkdir -p "$DATA_DIR/logs"

echo -e "${GREEN}✓ Created validator directories${NC}"

echo ""
echo -e "${YELLOW}[5/10] Setting Up Systemd Service${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Create systemd service file
cat > axiom-validator.service << EOF
[Unit]
Description=AXIOM Protocol Validator Node
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$(pwd)
ExecStart=$(pwd)/target/release/qubit --config $(pwd)/axiom-validator.toml
Restart=always
RestartSec=10
StandardOutput=append:$(pwd)/$DATA_DIR/logs/validator.log
StandardError=append:$(pwd)/$DATA_DIR/logs/validator-error.log

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=full
ProtectHome=read-only

# Resource limits
LimitNOFILE=65536
MemoryMax=16G

[Install]
WantedBy=multi-user.target
EOF

echo -e "${GREEN}✓ Created: axiom-validator.service${NC}"
echo -e "${YELLOW}  To install as system service (requires sudo):${NC}"
echo -e "${YELLOW}  sudo cp axiom-validator.service /etc/systemd/system/${NC}"
echo -e "${YELLOW}  sudo systemctl daemon-reload${NC}"
echo -e "${YELLOW}  sudo systemctl enable axiom-validator${NC}"
echo -e "${YELLOW}  sudo systemctl start axiom-validator${NC}"

echo ""
echo -e "${YELLOW}[6/10] Configuring Firewall${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

cat > validator-firewall.sh << 'EOF'
#!/bin/bash
# Firewall configuration for AXIOM validator

# Allow SSH
ufw allow 22/tcp comment "SSH"

# Allow P2P port
ufw allow 8545/tcp comment "AXIOM P2P"

# Allow RPC (restrict to specific IPs in production)
ufw allow 8546/tcp comment "AXIOM RPC"

# Allow metrics (restrict to monitoring server)
ufw allow 9100/tcp comment "Prometheus Metrics"

# Enable firewall
ufw --force enable

echo "✓ Firewall configured"
EOF

chmod +x validator-firewall.sh
echo -e "${GREEN}✓ Created: validator-firewall.sh${NC}"
echo -e "${YELLOW}  Run with sudo to apply firewall rules${NC}"

echo ""
echo -e "${YELLOW}[7/10] Creating Monitoring Configuration${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

mkdir -p monitoring/prometheus/validators

cat > monitoring/prometheus/validators/$VALIDATOR_NAME.yml << EOF
# Prometheus scrape config for $VALIDATOR_NAME

scrape_configs:
  - job_name: '$VALIDATOR_NAME'
    static_configs:
      - targets:
        - 'localhost:$METRICS_PORT'
        labels:
          validator: '$VALIDATOR_NAME'
          network: 'mainnet'
          node_type: 'archive'
          
  - job_name: '${VALIDATOR_NAME}_rpc'
    metrics_path: '/metrics'
    static_configs:
      - targets:
        - 'localhost:$RPC_PORT'
        labels:
          validator: '$VALIDATOR_NAME'
          endpoint_type: 'rpc'
EOF

echo -e "${GREEN}✓ Created validator monitoring config${NC}"

echo ""
echo -e "${YELLOW}[8/10] Creating Backup Script${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

cat > backup-validator.sh << EOF
#!/bin/bash
# Backup validator data

BACKUP_DIR="./backups/\$(date +%Y%m%d-%H%M%S)"
mkdir -p "\$BACKUP_DIR"

echo "Backing up validator data..."

# Backup configuration
cp axiom-validator.toml "\$BACKUP_DIR/"

# Backup keystore
if [ -d "$DATA_DIR/keystore" ]; then
    cp -r "$DATA_DIR/keystore" "\$BACKUP_DIR/"
fi

# Backup state (optional - large file)
# tar -czf "\$BACKUP_DIR/state.tar.gz" "$DATA_DIR/state"

echo "✓ Backup complete: \$BACKUP_DIR"
EOF

chmod +x backup-validator.sh
echo -e "${GREEN}✓ Created: backup-validator.sh${NC}"

echo ""
echo -e "${YELLOW}[9/10] Creating Health Check Script${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

cat > check-validator.sh << EOF
#!/bin/bash
# Health check for validator node

echo "AXIOM Validator Health Check"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check if process is running
if pgrep -x "qubit" > /dev/null; then
    echo "✓ Process: Running"
else
    echo "✗ Process: Not running"
    exit 1
fi

# Check RPC endpoint
RPC_RESPONSE=\$(curl -s http://localhost:$RPC_PORT 2>/dev/null)
if [ \$? -eq 0 ]; then
    echo "✓ RPC: Responding on port $RPC_PORT"
else
    echo "✗ RPC: Not responding"
fi

# Check peer connections (if netstat available)
if command -v netstat &> /dev/null; then
    PEER_COUNT=\$(netstat -an | grep :$P2P_PORT | grep ESTABLISHED | wc -l)
    echo "✓ Peers: \$PEER_COUNT connected"
fi

# Check disk space
DISK_USAGE=\$(df -h "$DATA_DIR" | tail -1 | awk '{print \$5}')
echo "✓ Disk: \$DISK_USAGE used"

# Check log for errors
if [ -f "$VALIDATOR_NAME.log" ]; then
    ERROR_COUNT=\$(grep -i "error\|fatal" "$VALIDATOR_NAME.log" | tail -10 | wc -l)
    if [ \$ERROR_COUNT -gt 0 ]; then
        echo "⚠ Recent errors: \$ERROR_COUNT (check logs)"
    else
        echo "✓ Logs: No recent errors"
    fi
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
EOF

chmod +x check-validator.sh
echo -e "${GREEN}✓ Created: check-validator.sh${NC}"

echo ""
echo -e "${YELLOW}[10/10] Final Setup${NC}"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Create start script
cat > start-validator.sh << EOF
#!/bin/bash
# Start AXIOM validator node

echo "Starting $VALIDATOR_NAME..."
./target/release/qubit --config axiom-validator.toml
EOF

chmod +x start-validator.sh
echo -e "${GREEN}✓ Created: start-validator.sh${NC}"

# Create stop script
cat > stop-validator.sh << EOF
#!/bin/bash
# Stop AXIOM validator node gracefully

echo "Stopping validator..."
pkill -SIGTERM qubit
echo "✓ Shutdown signal sent"
EOF

chmod +x stop-validator.sh
echo -e "${GREEN}✓ Created: stop-validator.sh${NC}"

echo ""
echo "╔═══════════════════════════════════════════════════════════════════════╗"
echo "║                    ✅ VALIDATOR SETUP COMPLETE                        ║"
echo "╚═══════════════════════════════════════════════════════════════════════╝"
echo ""
echo -e "${GREEN}Validator Configuration:${NC}"
echo "  Name: $VALIDATOR_NAME"
echo "  Address: $VALIDATOR_ADDRESS"
echo "  Network: Mainnet (ID: 1)"
echo "  Mode: Archive (full history)"
echo "  Data: $DATA_DIR"
echo ""
echo -e "${YELLOW}Quick Start Commands:${NC}"
echo "  Start validator:    ./start-validator.sh"
echo "  Stop validator:     ./stop-validator.sh"
echo "  Health check:       ./check-validator.sh"
echo "  Backup data:        ./backup-validator.sh"
echo ""
echo -e "${YELLOW}System Service (recommended for production):${NC}"
echo "  sudo cp axiom-validator.service /etc/systemd/system/"
echo "  sudo systemctl daemon-reload"
echo "  sudo systemctl enable axiom-validator"
echo "  sudo systemctl start axiom-validator"
echo "  sudo systemctl status axiom-validator"
echo ""
echo -e "${YELLOW}Monitoring:${NC}"
echo "  Logs: tail -f $VALIDATOR_NAME.log"
echo "  Metrics: http://localhost:$METRICS_PORT/metrics"
echo "  RPC: http://localhost:$RPC_PORT"
echo ""
echo -e "${RED}⚠ IMPORTANT SECURITY REMINDERS:${NC}"
echo "  1. Backup your validator keys immediately"
echo "  2. Configure firewall: sudo ./validator-firewall.sh"
echo "  3. Secure RPC endpoint (restrict CORS in production)"
echo "  4. Monitor validator uptime (99%+ required)"
echo "  5. Ensure 100,000+ AXM stake for mainnet"
echo ""
echo -e "${GREEN}✨ Validator ready to join AXIOM mainnet!${NC}"
echo ""
