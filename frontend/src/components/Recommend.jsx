// src/pages/Recommend.jsx
import React, { useState, useEffect, useCallback } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Search, ArrowLeft } from 'lucide-react'; // Add ArrowLeft import
import SidePanel from '../components/SidePanel';
import ThemeToggle from '../components/ThemeToggle';
import './Recommend.css';

const API_BASE_URL = "http://localhost:8080";

const Recommend = () => {

    const updateUI = useCallback(() => { }, []);

    const [results, setResults] = useState([]);
    const [loading, setLoading] = useState(false);
    const [showResults, setShowResults] = useState(false);
    const [error, setError] = useState(null);


    // const handleSearch = async () => {
    //     setLoading(true);
    //     setError(null);
    //     setShowResults(false);

    //     try {
    //         const userToken = localStorage.getItem('user_token');
    //         const username = localStorage.getItem('username');

    //         if (!userToken || !username) {
    //             throw new Error('User not authenticated');
    //         }

    //         const res = await fetch(`${API_BASE_URL}/v1/sticker/dashboard-trending`, {
    //             method: 'POST',
    //             headers: {
    //                 'Content-Type': 'application/json',
    //                 'Authorization': `Bearer ${userToken}`
    //             },
    //             body: JSON.stringify({
    //                 username: username
    //             })
    //         });

    //         const data = await res.json();

    //         if (res.ok) {
    //             setResults(data.sticker_urls || []);
    //             setShowResults(true);
    //         } else {
    //             setError(data.error || 'Failed to fetch recommendations');
    //         }
    //     } catch (err) {
    //         setError('An error occurred while fetching recommendations');
    //         console.error('Recommendation error:', err);
    //     } finally {
    //         setLoading(false);
    //     }
    // };

    const handleSearch = useCallback(async () => {
        setLoading(true);
        setError(null);
        setShowResults(false);

        try {
            const userToken = localStorage.getItem('user_token');
            const username = localStorage.getItem('username');

            if (!userToken || !username) {
                throw new Error('User not authenticated');
            }

            const res = await fetch(`${API_BASE_URL}/v1/sticker/dashboard-trending`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${userToken}`
                },
                body: JSON.stringify({
                    username: username
                })
            });

            const data = await res.json();

            if (res.ok) {
                setResults(data.sticker_urls || []);
                setShowResults(true);
            } else {
                setError(data.error || 'Failed to fetch recommendations');
            }
        } catch (err) {
            setError('An error occurred while fetching recommendations');
            console.error('Recommendation error:', err);
        } finally {
            setLoading(false);
        }
    }, []);
    return (
        <div className="recommend-bg">
            <SidePanel updateUI={updateUI} />
            <ThemeToggle />

            <div className="recommend-container">
                <motion.div
                    className="welcome-message"
                    initial={{ opacity: 1, y: 0 }}
                    animate={showResults ? { opacity: 0, y: -50 } : { opacity: 1, y: 0 }}
                    transition={{ duration: 0.5 }}
                >
                    <div>
                        <div className='text-container'><h1>Discover Trending Stickers</h1></div>
                        <div className='text-container'><p>Explore the latest and trending stickers</p></div>
                    </div>
                </motion.div>

                <motion.div
                    className="search-button-container"
                    initial={{ opacity: 1, y: 0 }}
                    animate={showResults ? { opacity: 0, y: -50 } : { opacity: 1, y: 0 }}
                    transition={{ duration: 0.5 }}
                >
                    <motion.button
                        className="search-button"
                        onClick={handleSearch}
                        whileHover={{ scale: 1.05 }}
                        whileTap={{ scale: 0.95 }}
                        disabled={loading}
                    >
                        {loading ? (
                            <span className="loading-spinner" />
                        ) : (
                            <>
                                <Search size={24} />
                                <span>Start Exploring</span>
                            </>
                        )}
                    </motion.button>
                </motion.div>

                <AnimatePresence>
                    {showResults && results.length > 0 && (
                        <motion.div
                            className="results-container"
                            initial={{ opacity: 0, y: 50 }}
                            animate={{ opacity: 1, y: 0 }}
                            exit={{ opacity: 0, y: 50 }}
                            transition={{ duration: 0.5 }}
                        >
                            {/* BACK BUTTON ADDED HERE */}
                            <div className="search-results-header">
                                <button
                                    className="back-button"
                                    onClick={() => setShowResults(false)}
                                >
                                    <ArrowLeft size={24} />
                                </button>
                                <div className='result-text'>    <h2>Trending Stickers</h2>    </div>
                            </div>
                            <div className="results-grid">
                                {results.map((url, index) => (
                                    <motion.div
                                        key={index}
                                        className="result-card"
                                        initial={{ opacity: 0, y: 20 }}
                                        animate={{ opacity: 1, y: 0 }}
                                        transition={{ delay: index * 0.05 }}
                                    >
                                        <div className="result-image">
                                            <img src={url} alt={`Trending sticker ${index + 1}`} loading='lazy' />
                                        </div>
                                    </motion.div>
                                ))}
                            </div>
                        </motion.div>
                    )}
                </AnimatePresence>

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
            </div>
        </div>
    );
}

export default React.memo(Recommend);