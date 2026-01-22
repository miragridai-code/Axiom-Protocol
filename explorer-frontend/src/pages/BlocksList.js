import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import axios from 'axios';
import { formatDistanceToNow } from 'date-fns';

function BlocksList() {
  const [blocks, setBlocks] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchBlocks();
  }, []);

  const fetchBlocks = async () => {
    try {
      const response = await axios.get('/api/blocks?limit=50');
      setBlocks(response.data);
      setLoading(false);
    } catch (error) {
      console.error('Error fetching blocks:', error);
      setLoading(false);
    }
  };

  const formatQBT = (satoshis) => {
    return (satoshis / 100000000).toFixed(2);
  };

  const shortenHash = (hash) => {
    return `${hash.substring(0, 10)}...${hash.substring(hash.length - 10)}`;
  };

  if (loading) return <div className="loading">Loading blocks...</div>;

  return (
    <div className="card">
      <h2 style={{ marginBottom: '20px' }}>All Blocks</h2>
      <table className="table">
        <thead>
          <tr>
            <th>Height</th>
            <th>Hash</th>
            <th>Age</th>
            <th>Transactions</th>
            <th>Miner</th>
            <th>Size</th>
            <th>Reward</th>
          </tr>
        </thead>
        <tbody>
          {blocks.map((block) => (
            <tr key={block.index}>
              <td>
                <Link to={`/block/${block.index}`}>
                  <strong>{block.index}</strong>
                </Link>
              </td>
              <td>
                <Link to={`/block/${block.hash}`} className="hash-short">
                  {shortenHash(block.hash)}
                </Link>
              </td>
              <td>{formatDistanceToNow(new Date(block.timestamp * 1000), { addSuffix: true })}</td>
              <td>
                <span className="badge badge-info">{block.transactions.length}</span>
              </td>
              <td className="hash-short">{shortenHash(block.miner)}</td>
              <td>{(block.size / 1024).toFixed(2)} KB</td>
              <td>{formatQBT(block.reward)} QBT</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default BlocksList;
