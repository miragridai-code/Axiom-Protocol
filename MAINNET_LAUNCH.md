# Qubit Protocol Mainnet Launch Guide

## Prerequisites
- Ensure all nodes run the latest release from the main branch
- Each validator must generate their own keypair and register with the network
- Trusted setup ceremony for ZK-SNARKs must be completed
- VDF parameters (RSA modulus, iterations) must be agreed upon and published

## Launch Steps
1. **Trusted Setup**: Run `zk-setup/ceremony.sh` and distribute keys to all validators
2. **Genesis Block**: Publish the genesis block hash and parameters
3. **Validator Registration**: Each validator registers their peer ID using the network API
4. **Network Bootstrap**: Start nodes with bootstrap peer list
5. **Block Explorer**: Deploy the explorer backend and frontend
6. **Monitoring**: Use AI Guardian for attack detection and peer scoring
7. **Public Announcement**: Publish explorer URL and validator list

## Post-Launch
- Monitor network health and validator activity
- Perform regular security audits
- Update documentation and onboard new validators

## Emergency Procedures
- Use state snapshot and rollback for chain recovery
- Validators can vote to freeze or rollback in case of critical bugs

---
For technical details, see `README.md` and `SECURITY.md`.
