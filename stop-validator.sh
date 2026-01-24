#!/bin/bash
# Stop AXIOM validator node gracefully

echo "Stopping validator..."
pkill -SIGTERM qubit
echo "✓ Shutdown signal sent"
