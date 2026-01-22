# Qubit Explorer Backend

REST API backend for the Qubit block explorer built with Actix-Web.

## Features

- RESTful API for blockchain data
- Network statistics endpoint
- Block lookup by height or hash
- Transaction search
- Address balance and history
- Universal search functionality
- CORS enabled for frontend integration
- Mock data for development

## Installation

```bash
cd explorer-backend
cargo build --release
```

## Running

```bash
cargo run --release
```

Server runs on `http://0.0.0.0:8080`

## API Endpoints

### Health Check
```
GET /health
```

### Network Statistics
```
GET /api/stats
```

Returns network information including:
- Block height
- Total transactions
- Circulating supply
- Difficulty
- Hash rate
- Active peers
- Latest blocks

### Get Block
```
GET /api/block/{id}
```

Get block by height (number) or hash (64-char hex string).

**Response:**
```json
{
  "index": 10,
  "hash": "...",
  "previous_hash": "...",
  "timestamp": 1600006000,
  "transactions": [...],
  "miner": "...",
  "difficulty": 1900,
  "nonce": 543210,
  "merkle_root": "...",
  "vdf_output": "...",
  "vdf_proof": "...",
  "size": 2816,
  "reward": 5000000000
}
```

### Get Latest Blocks
```
GET /api/blocks?limit=20
```

Query parameters:
- `limit` (optional, default 20, max 100): Number of blocks to return

### Get Transaction
```
GET /api/transaction/{hash}
```

Get transaction by hash.

**Response:**
```json
{
  "hash": "...",
  "sender": "...",
  "recipient": "...",
  "amount": 1000000000,
  "fee": 1000,
  "timestamp": 1600005970,
  "signature": "...",
  "block_hash": "...",
  "block_index": 10,
  "confirmations": 1,
  "zk_proof": null
}
```

### Get Address
```
GET /api/address/{address}
```

Get address balance and transaction history.

**Response:**
```json
{
  "address": "...",
  "balance": 5000000000,
  "total_received": 10000000000,
  "total_sent": 5000000000,
  "tx_count": 25,
  "recent_transactions": [...]
}
```

### Universal Search
```
GET /api/search/{query}
```

Search for block, transaction, or address. Returns:
- `{ "type": "Block", "data": {...} }` if block found
- `{ "type": "Transaction", "data": {...} }` if transaction found
- `{ "type": "Address", "data": {...} }` if address found
- `{ "type": "NotFound" }` if nothing found

## Development

Run with logging:
```bash
RUST_LOG=info cargo run
```

Run tests:
```bash
cargo test
```

## Integration with Qubit Node

Currently uses mock data. To integrate with actual Qubit node:

1. Replace `AppState` initialization with connection to actual node
2. Update endpoints to query real blockchain data
3. Implement WebSocket support for real-time updates
4. Add caching layer (Redis) for performance

## Technologies

- Actix-Web 4.4 - Web framework
- Actix-CORS - CORS middleware
- Serde - JSON serialization
- Tokio - Async runtime
- Chrono - Date/time handling
