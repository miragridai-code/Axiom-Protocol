import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import axios from 'axios';
import { formatDistanceToNow } from 'date-fns';

function Dashboard() {
  const [stats, setStats] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    fetchStats();
    const interval = setInterval(fetchStats, 10000); // Refresh every 10 seconds
    return () => clearInterval(interval);
  }, []);

  const fetchStats = async () => {
    try {
      const response = await axios.get('/api/stats');
      setStats(response.data);
      setLoading(false);
    } catch (err) {
      setError(err.message);
      setLoading(false);
    }
  };

  const formatQBT = (satoshis) => {
    return (satoshis / 100000000).toLocaleString(undefined, {
      minimumFractionDigits: 2,
      maximumFractionDigits: 8
    });
  };

  const shortenHash = (hash) => {
    return `${hash.substring(0, 8)}...${hash.substring(hash.length - 8)}`;
  };

  if (loading) return <div className="loading">Loading...</div>;
  if (error) return <div className="error">Error: {error}</div>;
  if (!stats) return null;

  return (
    <div>
      <div className="stats-grid">
        <div className="stat-card">
          <div className="stat-label">Block Height</div>
          <div className="stat-value highlight">{stats.height.toLocaleString()}</div>
        </div>
        <div className="stat-card">
          <div className="stat-label">Total Transactions</div>
          <div className="stat-value">{stats.total_transactions.toLocaleString()}</div>
        </div>
        <div className="stat-card">
          <div className="stat-label">Circulating Supply</div>
          <div className="stat-value">{formatQBT(stats.circulating_supply)} QBT</div>
        </div>
        <div className="stat-card">
          <div className="stat-label">Network Difficulty</div>
          <div className="stat-value">{stats.difficulty.toLocaleString()}</div>
        </div>
        <div className="stat-card">
          <div className="stat-label">Hash Rate</div>
          <div className="stat-value">{(stats.hash_rate / 1000000).toFixed(2)} MH/s</div>
        </div>
        <div className="stat-card">
          <div className="stat-label">Active Peers</div>
          <div className="stat-value">{stats.peers}</div>
        </div>
        <div className="stat-card">
          <div className="stat-label">Mempool Size</div>
          <div className="stat-value">{stats.mempool_size}</div>
        </div>
        <div className="stat-card">
          <div className="stat-label">Avg Block Time</div>
          <div className="stat-value">{stats.average_block_time}s</div>
        </div>
      </div>

      <div className="card">
        <h2 style={{ marginBottom: '20px' }}>Latest Blocks</h2>
        <table className="table">
          <thead>
            <tr>
              <th>Height</th>
              <th>Hash</th>
              <th>Age</th>
              <th>Transactions</th>
              <th>Miner</th>
              <th>Reward</th>
            </tr>
          </thead>
          <tbody>
            {stats.latest_blocks.map((block) => (
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
                  <span className="badge badge-info">{block.tx_count}</span>
                </td>
                <td className="hash-short">{shortenHash(block.miner)}</td>
                <td>{formatQBT(block.reward)} QBT</td>
              </tr>
            ))}
          </tbody>
        </table>
        <div style={{ textAlign: 'center', marginTop: '20px' }}>
          <Link to="/blocks">
            <button className="btn btn-primary">View All Blocks</button>
          </Link>
        </div>
      </div>
    </div>
  );
}

export default Dashboard;
