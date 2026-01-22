import React, { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import axios from 'axios';
import { formatDistanceToNow } from 'date-fns';

function AddressDetail() {
  const { address } = useParams();
  const [addressInfo, setAddressInfo] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    fetchAddress();
  }, [address]);

  const fetchAddress = async () => {
    try {
      const response = await axios.get(`/api/address/${address}`);
      setAddressInfo(response.data);
      setLoading(false);
    } catch (err) {
      setError(err.message);
      setLoading(false);
    }
  };

  const formatQBT = (satoshis) => {
    return (satoshis / 100000000).toFixed(8);
  };

  const shortenHash = (hash) => {
    return `${hash.substring(0, 12)}...${hash.substring(hash.length - 12)}`;
  };

  if (loading) return <div className="loading">Loading address...</div>;
  if (error) return <div className="error">Error: {error}</div>;
  if (!addressInfo) return <div className="error">Address not found</div>;

  return (
    <div>
      <div className="card">
        <h2 style={{ marginBottom: '30px' }}>Address Details</h2>
        
        <div className="detail-row">
          <div className="detail-label">Address:</div>
          <div className="detail-value hash">{addressInfo.address}</div>
        </div>

        <div className="detail-row">
          <div className="detail-label">Balance:</div>
          <div className="detail-value">
            <strong style={{ fontSize: '24px', color: '#667eea' }}>
              {formatQBT(addressInfo.balance)} QBT
            </strong>
          </div>
        </div>

        <div className="detail-row">
          <div className="detail-label">Total Received:</div>
          <div className="detail-value">{formatQBT(addressInfo.total_received)} QBT</div>
        </div>

        <div className="detail-row">
          <div className="detail-label">Total Sent:</div>
          <div className="detail-value">{formatQBT(addressInfo.total_sent)} QBT</div>
        </div>

        <div className="detail-row">
          <div className="detail-label">Transactions:</div>
          <div className="detail-value">
            <span className="badge badge-info">{addressInfo.tx_count}</span>
          </div>
        </div>
      </div>

      {addressInfo.recent_transactions.length > 0 && (
        <div className="card">
          <h3 style={{ marginBottom: '20px' }}>Recent Transactions</h3>
          <table className="table">
            <thead>
              <tr>
                <th>Hash</th>
                <th>Type</th>
                <th>From</th>
                <th>To</th>
                <th>Amount</th>
                <th>Age</th>
              </tr>
            </thead>
            <tbody>
              {addressInfo.recent_transactions.map((tx) => {
                const isIncoming = tx.recipient === addressInfo.address;
                const isOutgoing = tx.sender === addressInfo.address;
                
                return (
                  <tr key={tx.hash}>
                    <td>
                      <Link to={`/transaction/${tx.hash}`} className="hash-short">
                        {shortenHash(tx.hash)}
                      </Link>
                    </td>
                    <td>
                      {isIncoming && <span className="badge badge-success">IN</span>}
                      {isOutgoing && <span className="badge badge-warning">OUT</span>}
                    </td>
                    <td className="hash-short">{shortenHash(tx.sender)}</td>
                    <td className="hash-short">{shortenHash(tx.recipient)}</td>
                    <td style={{ color: isIncoming ? '#2e7d32' : '#c62828' }}>
                      {isIncoming ? '+' : '-'}{formatQBT(tx.amount)} QBT
                    </td>
                    <td>
                      {formatDistanceToNow(new Date(tx.timestamp * 1000), { addSuffix: true })}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}

export default AddressDetail;
