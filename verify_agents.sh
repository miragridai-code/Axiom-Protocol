#!/bin/bash
# Quick test script to verify OpenClaw agents start automatically
# Run this after starting the Axiom node

set -e

echo "=========================================="
echo "  OpenClaw Agent Auto-Startup Verification"
echo "=========================================="
echo ""

# Give the node a moment to start agents
sleep 3

echo "🔍 Checking for running agents..."
echo ""

# Array of agent scripts to check
agents=(
    "security_guardian_agent.py"
    "network_booster_agent.py"
    "node_health_monitor.py"
    "ceremony_master.py"
)

found_count=0

for agent in "${agents[@]}"; do
    if pgrep -f "$agent" > /dev/null; then
        pid=$(pgrep -f "$agent" | head -1)
        echo "✅ $agent (PID: $pid)"
        ((found_count++))
    else
        echo "❌ $agent - NOT RUNNING"
    fi
done

echo ""
echo "=========================================="
if [ $found_count -eq 4 ]; then
    echo "✅ All 4 agents running successfully!"
    echo ""
    echo "Agent Details:"
    ps aux | grep -E "security_guardian|network_booster|node_health|ceremony_master" | grep -v grep
    echo ""
    echo "📊 Performance Metrics at: http://127.0.0.1:9090"
else
    echo "⚠️  Only $found_count/4 agents running"
    echo ""
    echo "Troubleshooting:"
    echo "1. Verify Python3 installed: python3 --version"
    echo "2. Check agent files exist: ls -la openclaw/"
    echo "3. Test agent directly: python3 openclaw/security_guardian_agent.py"
    echo "4. Review Axiom node console for error messages"
fi
echo "=========================================="
