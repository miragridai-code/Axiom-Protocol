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
