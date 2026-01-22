# Qubit SDK for JavaScript

JavaScript/Node.js SDK for interacting with the Qubit blockchain.

## Installation

```bash
npm install qubit-sdk
```

## Quick Start

```javascript
const { QubitClient, Wallet, qbtToSats, satsToQbt } = require('qubit-sdk');

(async () => {
    // Initialize client
    const client = new QubitClient('http://localhost:8332');
    
    // Create wallet
    const wallet = new Wallet();  // Or load existing: new Wallet(privateKey)
    console.log(`Address: ${wallet.address}`);
    
    // Check balance
    const balance = await client.getBalance(wallet.address);
    console.log(`Balance: ${satsToQbt(balance)} QBT`);
    
    // Send transaction
    const recipient = 'recipient_address_here';
    const amount = qbtToSats(1.5);  // Send 1.5 QBT
    const txHash = await client.send(wallet, recipient, amount, 1000, true);
    console.log(`Transaction sent: ${txHash}`);
    
    // Get chain info
    const info = await client.getChainInfo();
    console.log(`Chain height: ${info.height}`);
})();
```

## Features

- **Wallet Management**: Generate keys, sign transactions
- **Transactions**: Create, sign, and broadcast transactions
- **Queries**: Get blocks, transactions, balances, chain info
- **ZK-SNARKs**: Generate privacy-preserving proofs
- **VDF Verification**: Verify verifiable delay function proofs
- **Neural Guardian**: Query AI threat detection system

## API Reference

### QubitClient

- `getBalance(address)` - Get balance for an address
- `getNonce(address)` - Get nonce for an address
- `createTransaction(wallet, recipient, amount, fee, useZk)` - Create signed transaction
- `broadcastTransaction(tx)` - Broadcast transaction
- `send(wallet, recipient, amount, fee, useZk)` - Create and broadcast transaction
- `getTransaction(txHash)` - Get transaction by hash
- `getBlock(blockHash, index)` - Get block by hash or index
- `getLatestBlock()` - Get latest block
- `getChainInfo()` - Get blockchain info
- `verifyVdf(output, proof, input, time)` - Verify VDF proof
- `queryNeuralGuardian(peerId)` - Query threat detection

### Wallet

- `new Wallet()` - Generate new wallet
- `new Wallet(privateKey)` - Load existing wallet
- `sign(message)` - Sign a message
- `Wallet.verify(message, signature, publicKey)` - Verify signature

## License

MIT
