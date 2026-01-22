# Qubit Explorer Frontend

React-based block explorer frontend for the Qubit blockchain.

## Features

- **Dashboard**: Real-time network statistics and latest blocks
- **Block Explorer**: Browse and search blocks by height or hash
- **Transaction Viewer**: View detailed transaction information
- **Address Lookup**: Check address balances and transaction history
- **Search**: Universal search for blocks, transactions, and addresses
- **Responsive Design**: Works on desktop and mobile devices

## Installation

```bash
cd explorer-frontend
npm install
```

## Development

```bash
npm start
```

Runs the app in development mode at [http://localhost:3000](http://localhost:3000).

The backend API is proxied from `http://localhost:8080`.

## Production Build

```bash
npm run build
```

Creates an optimized production build in the `build/` directory.

## Project Structure

```
src/
├── components/
│   └── Search.js          # Universal search component
├── pages/
│   ├── Dashboard.js       # Main dashboard with stats
│   ├── BlocksList.js      # List of all blocks
│   ├── BlockDetail.js     # Individual block details
│   ├── TransactionDetail.js # Transaction details
│   └── AddressDetail.js   # Address information
├── App.js                 # Main app with routing
├── index.js              # Entry point
└── index.css             # Global styles
```

## API Endpoints Used

- `GET /api/stats` - Network statistics
- `GET /api/blocks?limit=N` - Latest blocks
- `GET /api/block/{id}` - Block by hash or index
- `GET /api/transaction/{hash}` - Transaction details
- `GET /api/address/{address}` - Address information
- `GET /api/search/{query}` - Universal search

## Technologies

- React 18
- React Router v6
- Axios for API calls
- date-fns for date formatting
- Responsive CSS Grid/Flexbox
