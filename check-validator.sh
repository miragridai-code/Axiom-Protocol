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
RPC_RESPONSE=$(curl -s http://localhost:8546 2>/dev/null)
if [ $? -eq 0 ]; then
    echo "✓ RPC: Responding on port 8546"
else
    echo "✗ RPC: Not responding"
fi

# Check peer connections (if netstat available)
if command -v netstat &> /dev/null; then
    PEER_COUNT=$(netstat -an | grep :8545 | grep ESTABLISHED | wc -l)
    echo "✓ Peers: $PEER_COUNT connected"
fi

# Check disk space
DISK_USAGE=$(df -h "./axiom-validator-data" | tail -1 | awk '{print $5}')
echo "✓ Disk: $DISK_USAGE used"

# Check log for errors
if [ -f "axiom-validator-1.log" ]; then
    ERROR_COUNT=$(grep -i "error\|fatal" "axiom-validator-1.log" | tail -10 | wc -l)
    if [ $ERROR_COUNT -gt 0 ]; then
        echo "⚠ Recent errors: $ERROR_COUNT (check logs)"
    else
        echo "✓ Logs: No recent errors"
    fi
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
