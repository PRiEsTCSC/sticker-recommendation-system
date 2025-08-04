// src/pages/Search.jsx
import React, { useState, useCallback } from 'react';
import { debounce } from 'lodash';
import { motion, AnimatePresence } from 'framer-motion';
import { Search, X } from 'lucide-react';
import { ArrowLeft } from 'lucide-react';
import SidePanel from '../components/SidePanel';
import ThemeToggle from '../components/ThemeToggle';
import './Search.css';

const API_BASE_URL = "http://localhost:8080";


const SearchPage = () => {
    const [searchQuery, setSearchQuery] = useState('');
    const [results, setResults] = useState([]);
    const [loading, setLoading] = useState(false);
    const [isSearchBarActive, setIsSearchBarActive] = useState(false);
    const [error, setError] = useState(null);
    const [showLoading, setShowLoading] = useState(false);

    const updateUI = useCallback(() => {}, []);

    // const handleSearch = async (e) => {
    //     e.preventDefault();
    //     if (!searchQuery.trim()) return;

    //     setShowLoading(true); 

    //     setLoading(true);
    //     setError(null);
    //     setResults([]);

    //     try {
    //         const userToken = localStorage.getItem('user_token');
    //         const username = localStorage.getItem('username');

    //         if (!userToken || !username) {
    //             throw new Error('User not authenticated');
    //         }

    //         const res = await fetch(`${API_BASE_URL}/v1/sticker/dashboard-find`, {
    //             method: 'POST',
    //             headers: {
    //                 'Content-Type': 'application/json',
    //                 'Authorization': `Bearer ${userToken}`,
    //             },
    //             body: JSON.stringify({
    //                 username: username,
    //                 input_text: searchQuery,
    //             }),
    //         });

    //         const data = await res.json();

    //         if (res.ok) {
    //             setResults(data.sticker_urls || []);
    //         } else {
    //             setError(data.error || 'Search failed');
    //         }
    //     } catch (err) {
    //         setError('An error occurred while searching');
    //         console.error('Search error:', err);
    //     } finally {
    //         setLoading(false);
    //         setShowLoading(false); // Hide loading modal

    //     }
    // };

    // const handleSearchBarClick = () => {
    //     setIsSearchBarActive(true);
    // };

    // const handleSearchBarClose = () => {
    //     setIsSearchBarActive(false);
    //     setSearchQuery('');
    //     setResults([]);
    //     setError(null);
    // };

    const debouncedSearch = useCallback(
        debounce(async (query) => {
            if (!query.trim()) return;
            setShowLoading(true);
            setLoading(true);
            setError(null);
            setResults([]);

            try {
                const userToken = localStorage.getItem('user_token');
                const username = localStorage.getItem('username');

                if (!userToken || !username) {
                    throw new Error('User not authenticated');
                }

                const res = await fetch(`${API_BASE_URL}/v1/sticker/dashboard-find`, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': `Bearer ${userToken}`,
                    },
                    body: JSON.stringify({
                        username: username,
                        input_text: query,
                    }),
                });

                const data = await res.json();

                if (res.ok) {
                    setResults(data.sticker_urls || []);
                } else {
                    setError(data.error || 'Search failed');
                }
            } catch (err) {
                setError('An error occurred while searching');
                console.error('Search error:', err);
            } finally {
                setLoading(false);
                setShowLoading(false);
            }
        }, 500),
        []
    );

    const handleSearch = useCallback((e) => {
        e.preventDefault();
        debouncedSearch(searchQuery);
    }, [searchQuery, debouncedSearch]);

    const handleSearchBarClick = useCallback(() => {
        setIsSearchBarActive(true);
    }, []);

    const handleSearchBarClose = useCallback(() => {
        setIsSearchBarActive(false);
        setSearchQuery('');
        setResults([]);
        setError(null);
    }, []);
    return (
        <div className="search-bg">
            <SidePanel updateUI={updateUI} />
            <ThemeToggle />


            {/* Loading Modal */}
            <AnimatePresence>
                {showLoading && (
                    <motion.div
                        className="loading-modal"
                        initial={{ opacity: 0 }}
                        animate={{ opacity: 1 }}
                        exit={{ opacity: 0 }}
                        transition={{ duration: 0.3 }}
                    >
                        <div className="spinner"></div>
                        {/* <p className="loading-text">Searching...</p> */}
                    </motion.div>
                )}
            </AnimatePresence>

            <div className="search-container">
                <motion.div
                    className="search-bar-container"
                    initial={{ y: '-500%', opacity: 1 }}
                    animate={isSearchBarActive ? { top: '-500%', opacity: 1 } : { y: 0, opacity: 1 }}
                    exit={{ y: '-500%', opacity: 1 }}
                    transition={{ duration: 0.7 }}
                >
                    <div className="search-input-wrapper">
                        <form onSubmit={handleSearch} className="search-form">
                            <div className='input-form'><input
                                type="text"
                                placeholder="Search..."
                                value={searchQuery}
                                onChange={(e) => setSearchQuery(e.target.value)}
                                disabled={loading}
                                onClick={handleSearchBarClick}
                            /></div>
                            <div className='submit-button'><button type="submit" disabled={loading}>
                                {loading ? '' : <Search size={20} />}
                            </button></div>
                        </form>
                    </div>
                    <motion.button
                        className="search-close-button"
                        onClick={handleSearchBarClose}
                        initial={{ opacity: 0 }}
                        animate={isSearchBarActive ? { opacity: 1, left: '110%' } : { opacity: 0 }}
                        transition={{ duration: 0.3 }}
                    >
                        <X size={24} />
                    </motion.button>
                </motion.div>

                {error && (
                    <motion.div
                        className="error-message"
                        initial={{ opacity: 0, y: 20 }}
                        animate={{ opacity: 1, y: 0 }}
                        transition={{ duration: 0.5 }}
                    >
                        <p>{error}</p>
                    </motion.div>
                )}

                <AnimatePresence>
                    {results.length > 0 && (
                        <motion.div
                            className="results-container"
                            initial={{ opacity: 0, y: 50 }}
                            animate={{ opacity: 1, y: 0 }}
                            transition={{ duration: 0.5 }}
                        >
                            <div className="search-results-header">
                                <button
                                    className="back-button"
                                    onClick={() => {
                                        setIsSearchBarActive(true); // Keep search bar active
                                        setResults([]);
                                        setSearchQuery('');
                                        setError(null);
                                    }}
                                >
                                    <ArrowLeft size={24} /> {/* Back arrow icon */}
                                </button>
                                <div className='sticker-result-text'><h2>Search Results</h2></div>
                            </div>
                            <div className="results-grid">
                                {results.map((url, index) => (
                                    <motion.div
                                        key={index}
                                        className="result-card"
                                        initial={{ opacity: 0, y: 20 }}
                                        animate={{ opacity: 1, y: 0 }}
                                        transition={{ delay: index * 0.1 }}
                                    >
                                        <div className="result-image">
                                            <img src={url} alt="Search result" loading='lazy' />
                                        </div>
                                        <div className="result-content">
                                            <h3>Sticker</h3>
                                            <p>Found sticker related to "{searchQuery}"</p>
                                        </div>
                                    </motion.div>
                                ))}
                            </div>
                        </motion.div>
                    )}
                </AnimatePresence>

                <AnimatePresence>
                    {!isSearchBarActive && results.length === 0 && !error && (
                        <motion.div
                            className="welcome-message"
                            initial={{ opacity: 0 }}
                            animate={{ opacity: 1 }}
                            transition={{ duration: 0.5 }}
                        >
                            <h1>Search for anything</h1>
                            <p>Start typing to find content</p>
                        </motion.div>
                    )}
                </AnimatePresence>
            </div>
        </div>
    );
}

export default React.memo(SearchPage);