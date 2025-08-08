// src/pages/History.jsx
import React, { useState, useEffect , useCallback } from 'react';
import { motion } from 'framer-motion';
import SidePanel from '../components/SidePanel';
import ThemeToggle from '../components/ThemeToggle';
import { useTheme } from '../context/ThemeContext';
import './History.css';

const API_BASE_URL =import.meta.env.VITE_API_URL;


const History = () => {

    const updateUI = useCallback(() => {}, []);
    const [history, setHistory] = useState([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
    
    useEffect(() => {
        const fetchHistory = async () => {
            try {
                const userToken = localStorage.getItem('user_token');
                const username = localStorage.getItem('username');
                
                if (!userToken || !username) {
                    throw new Error('User not authenticated');
                }
                
                const res = await fetch(`${API_BASE_URL}/v1/user/history`, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        'Authorization': `Bearer ${userToken}`
                    },
                    body: JSON.stringify({ username: username })
                });
                
                const data = await res.json();
                
                if (res.ok) {
                    // Process the history data
                    const processedHistory = processHistory(data.history || []);
                    setHistory(processedHistory);
                } else {
                    setError(data.error || 'Failed to fetch history');
                }
            } catch (err) {
                setError('An error occurred while fetching history');
                console.error('History fetch error:', err);
            } finally {
                setLoading(false);
            }
        };
        
        fetchHistory();
    }, []);
    
    // Process history data to group by input_text, filter GIF URLs, and sort by newest first
    const processHistory = (historyItems) => {
        // Sort items by created_at descending (newest first)
        const sortedItems = [...historyItems].sort((a, b) => {
            return new Date(b.created_at) - new Date(a.created_at);
        });
        
        const groupedHistory = {};
        
        sortedItems.forEach(item => {
            const inputText = item.input_text;
            const stickerUrls = item.sticker_url || [];
            
            // Filter out non-GIF URLs and invalid URLs
            const validUrls = stickerUrls
                .filter(url => url && url.trim() !== '')
                .filter(url => url.toLowerCase().endsWith('.gif'));
            
            // Remove duplicates
            const uniqueUrls = [...new Set(validUrls)];
            
            if (uniqueUrls.length > 0) {
                if (!groupedHistory[inputText]) {
                    groupedHistory[inputText] = [];
                }
                groupedHistory[inputText].push({
                    sticker_urls: uniqueUrls,
                    created_at: item.created_at
                });
            }
        });
        
        // Convert to array format for rendering
        return Object.keys(groupedHistory).map(inputText => ({
            input_text: inputText,
            items: groupedHistory[inputText]
        }));
    };
    
    return (
        <div className="history-bg">
            <SidePanel updateUI={updateUI} />
            <ThemeToggle />
            
            <div className="history-container">
                <motion.div
                    className="history-content"
                    initial={{ opacity: 0, y: 50 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ duration: 0.5 }}
                >
                    <h1>History</h1>
                    
                    {loading ? (
                        <div className="loading-spinner">Loading history...</div>
                    ) : error ? (
                        <div className="error-message">{error}</div>
                    ) : history.length === 0 ? (
                        <div className="no-history-message">
                            <p>No history found. Start searching for stickers to see them here.</p>
                        </div>
                    ) : (
                        <div className="history-scroll-container">
                            {history.map((item, index) => (
                                <motion.div
                                    key={index}
                                    className="history-group"
                                    initial={{ opacity: 0, y: 20 }}
                                    animate={{ opacity: 1, y: 0 }}
                                    transition={{ delay: index * 0.05 }}
                                >
                                    <div className="history-group-header">
                                        <h3>{item.input_text}</h3>
                                        <p className="history-date">
                                            {new Date(item.items[0].created_at).toLocaleDateString()}
                                        </p>
                                    </div>
                                    <div className="history-gifs">
                                        {item.items.map((groupItem, groupIndex) => (
                                            groupItem.sticker_urls.map((url, urlIndex) => (
                                                <motion.div
                                                    key={`${index}-${groupIndex}-${urlIndex}`}
                                                    className="history-gif-item"
                                                    initial={{ opacity: 0, scale: 0.8 }}
                                                    animate={{ opacity: 1, scale: 1 }}
                                                    transition={{ delay: groupIndex * 0.05 + urlIndex * 0.02 }}
                                                >
                                                    <img 
                                                        src={url} 
                                                        alt={`Result for ${item.input_text}`} 
                                                    />
                                                </motion.div>
                                            ))
                                        ))}
                                    </div>
                                </motion.div>
                            ))}
                        </div>
                    )}
                </motion.div>
            </div>
        </div>
    );
}

export default React.memo(History);