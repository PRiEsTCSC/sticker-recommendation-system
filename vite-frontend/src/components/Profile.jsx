// src/pages/Profile.jsx
import React, { useState, useEffect , useCallback} from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { User, Pencil, CheckCircle, XCircle } from 'lucide-react'; // Import icons
import SidePanel from '../components/SidePanel';
import ThemeToggle from '../components/ThemeToggle';
import { useTheme } from '../context/ThemeContext';
// import '../pages/Auth.css'; // CORRECTED PATH for Auth.css
import './Profile.css'; // CORRECTED PATH for Profile.css

const API_BASE_URL = import.meta.env.VITE_API_URL;

const toProperCase = str =>
    str
        ? str.replace(/\w\S*/g, txt =>
            txt.charAt(0).toUpperCase() + txt.substr(1).toLowerCase()
        )
        : '';


const Profile = () => {
    console.log("Profile component rendered."); // THIS IS CRUCIAL FOR DEBUGGING

    const [username, setUsername] = useState(localStorage.getItem('username') || 'User');
    const [newUsername, setNewUsername] = useState('');
    const [isEditingUsername, setIsEditingUsername] = useState(false);
    const [message, setMessage] = useState('');

    const { theme, setTheme } = useTheme(); // ✅ Now `setTheme` is available

    // Memoize updateUI to prevent unnecessary re-renders of SidePanel
    const updateUI = useCallback(() => {}, []);



    useEffect(() => {
        setNewUsername(username); // Initialize newUsername with current username when editing starts
    }, [isEditingUsername, username]);

    const handleUsernameChange = useCallback( async (e) => {
        e.preventDefault();
        setMessage('Updating username...');
        const userToken = localStorage.getItem('user_token');

        if (!userToken) {
            setMessage('Error: Not authenticated. Please log in.');
            return;
        }

        if (newUsername.trim() === '' || newUsername === username) {
            setMessage('New username cannot be empty or the same as current.');
            return;
        }

        try {
            const res = await fetch(`${API_BASE_URL}/v1/user/update-username`, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${userToken}`
                },
                body: JSON.stringify({ new_username: newUsername })
            });

            const data = await res.json();

            if (res.ok) {
                localStorage.setItem('username', newUsername); // Update local storage
                setUsername(newUsername); // Update state
                setMessage(''); // Clear message on success
                setIsEditingUsername(false); // Exit edit mode
            } else {
                setMessage(data.error || 'Failed to update username.');
            }
        } catch (err) {
            setMessage(`Network error: ${err.message}`);
        }
    }, [newUsername, username]);


    return (
        <div className="welcome-bg"> {/* Uses auth-page-wrapper for full screen background */}
            <SidePanel updateUI={updateUI} /> {/* SidePanel on all screens */}
            <ThemeToggle /> {/* ThemeToggle on all screens */}

            <motion.div
                className="profile-container" // Reusing glass-card for consistent look
                initial={{ opacity: 0, y: -50 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: 50 }}
                transition={{ duration: 0.6 }}
            >
                <div className="profile-header">
                    <User size={60} />
                    <motion.h2
                        initial={{ scale: 0.8, opacity: 0 }}
                        animate={{ scale: 1, opacity: 1 }}
                        transition={{ delay: 0.2 }}
                    >
                            <div className='username-text'>{toProperCase(username)}</div>
                        <div className='pencil'><Pencil
                            size={20}
                            className="edit-icon"
                            onClick={() => setIsEditingUsername(!isEditingUsername)}
                        /></div>
                    </motion.h2>
                </div>

                <AnimatePresence>
                    {isEditingUsername && (
                        <motion.form
                            className="username-edit-form"
                            initial={{ opacity: 0, height: 0 }}
                            animate={{ opacity: 1, height: 'auto' }}
                            exit={{ opacity: 0, height: 0 }}
                            transition={{ duration: 0.3, ease: 'easeInOut' }}
                            onSubmit={handleUsernameChange}
                        >
                            <input
                                type="text"
                                value={newUsername}
                                onChange={(e) => setNewUsername(e.target.value)}
                                placeholder="New Username"
                                required
                            />
                            <div className="form-actions">
                                <motion.button
                                    type="submit"
                                    className="button-auth" // Reuse button style
                                    whileHover={{ scale: 1.05 }}
                                    whileTap={{ scale: 0.95 }}
                                >
                                    <CheckCircle size={20} /> Rename
                                </motion.button>
                                <motion.button
                                    type="button"
                                    className="button-cancel" // New cancel button style
                                    onClick={() => {
                                        setIsEditingUsername(false);
                                        setNewUsername(username); // Reset to current username
                                        setMessage(''); // Clear any messages
                                    }}
                                    whileHover={{ scale: 1.05 }}
                                    whileTap={{ scale: 0.95 }}
                                >
                                    <XCircle size={20} /> Cancel
                                </motion.button>
                            </div>
                        </motion.form>
                    )}
                </AnimatePresence>

                {message && (
                    <motion.p
                        className="msg"
                        initial={{ opacity: 0, y: -10 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: -10 }}
                        transition={{ duration: 0.3 }}
                    >
                        {message}
                    </motion.p>
                )}

                <motion.p
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    transition={{ delay: 0.4 }}
                >
                    Manage your profile settings here.
                </motion.p>
            </motion.div>
        </div>
    );
}


export default React.memo(Profile);