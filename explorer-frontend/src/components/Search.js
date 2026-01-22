import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import axios from 'axios';

function Search() {
  const [query, setQuery] = useState('');
  const [searching, setSearching] = useState(false);
  const navigate = useNavigate();

  const handleSearch = async (e) => {
    e.preventDefault();
    if (!query.trim()) return;

    setSearching(true);
    try {
      const response = await axios.get(`/api/search/${query.trim()}`);
      const result = response.data;

      if (result.type === 'Block') {
        navigate(`/block/${result.data.index}`);
      } else if (result.type === 'Transaction') {
        navigate(`/transaction/${result.data.hash}`);
      } else if (result.type === 'Address') {
        navigate(`/address/${result.data.address}`);
      } else {
        alert('Not found');
      }
    } catch (error) {
      console.error('Search error:', error);
      alert('Search failed. Please try again.');
    } finally {
      setSearching(false);
    }
  };

  return (
    <div className="search-bar">
      <form onSubmit={handleSearch}>
        <input
          type="text"
          className="search-input"
          placeholder="Search by Block Height, Block Hash, Transaction Hash, or Address..."
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          disabled={searching}
        />
      </form>
    </div>
  );
}

export default Search;
