#!/bin/bash
# Backup validator data

BACKUP_DIR="./backups/$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"

echo "Backing up validator data..."

# Backup configuration
cp axiom-validator.toml "$BACKUP_DIR/"

# Backup keystore
if [ -d "./axiom-validator-data/keystore" ]; then
    cp -r "./axiom-validator-data/keystore" "$BACKUP_DIR/"
fi

# Backup state (optional - large file)
# tar -czf "$BACKUP_DIR/state.tar.gz" "./axiom-validator-data/state"

echo "✓ Backup complete: $BACKUP_DIR"
