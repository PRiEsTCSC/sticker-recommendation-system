// // File: src/components/SidePanel.jsx
// import React, { useState } from 'react';
// import { NavLink, useNavigate } from 'react-router-dom'; // Import useNavigate
// import { motion, AnimatePresence } from 'framer-motion';
// import {
//     InfoCircleFill,
//     StarFill,
//     SearchHeartFill,
//     PersonFill,
//     ClockFill,
//     HouseFill,
//     TelephoneFill
// } from 'react-bootstrap-icons';
// import { Menu, X } from 'lucide-react';

// import './SidePanel.css';

// const links = [
//     { to: '/welcome', label: 'Home', icon: HouseFill },
//     { to: '/profile', label: 'Profile', icon: PersonFill },
//     { to: '/search', label: 'Search', icon: SearchHeartFill },
//     { to: '/recommend', label: 'Recommend', icon: StarFill },
//     { to: '/contact', label: 'Contact', icon: TelephoneFill },
//     { to: '/history', label: 'History', icon: ClockFill },
//     { to: '/about', label: 'About', icon: InfoCircleFill },
// ];


// const SidePanel = ({ updateUI }) => {
//     const [open, setOpen] = useState(true);
//     const navigate = useNavigate(); // Initialize useNavigate

import React, { useState, useCallback, useMemo } from 'react';
import { NavLink, useNavigate } from 'react-router-dom';
import { motion, AnimatePresence } from 'framer-motion';
import { Info, Star, Search, User, Clock, Home, Phone, Menu, X } from 'lucide-react';
import './SidePanel.css';

const SidePanel = ({ updateUI }) => {
    const [open, setOpen] = useState(true);
    const navigate = useNavigate();

    const links = useMemo(() => [
        { to: '/welcome', label: 'Home', icon: Home },
        { to: '/profile', label: 'Profile', icon: User },
        { to: '/search', label: 'Search', icon: Search },
        { to: '/recommend', label: 'Recommend', icon: Star },
        { to: '/contact', label: 'Contact', icon: Phone },
        { to: '/history', label: 'History', icon: Clock },
        { to: '/about', label: 'About', icon: Info },
    ], []);
    const panelVariants = {
        closed: { x: -250, transition: { type: 'tween', duration: 0.3, ease: 'easeOut' } },
        open: { x: 0, transition: { type: 'tween', duration: 0.3, ease: 'easeOut' } }
    };

    // const handleLogout = () => {
    //     // Clear local storage items
    //     localStorage.removeItem('user_token');
    //     localStorage.removeItem('username');
    //     localStorage.removeItem('theme'); // Also clear the theme on logout for a fresh start

    //     // If you have a global UI update function, call it (e.g., in App.js)
    //     if (updateUI) {
    //         updateUI(null, null);
    //     }

    //     // Send message if part of a browser extension
    //     if (window.chrome && chrome.runtime && chrome.runtime.sendMessage) {
    //         chrome.runtime.sendMessage({ action: 'updateMenu' });
    //     }

    //     // Redirect to the login page
    //     navigate('/login'); // Use navigate to go to the login route
    // };

    const handleLogout = useCallback(() => {
        localStorage.removeItem('user_token');
        localStorage.removeItem('username');
        localStorage.removeItem('theme');

        if (updateUI) {
            updateUI(null, null);
        }

        if (window.chrome && chrome.runtime && chrome.runtime.sendMessage) {
            chrome.runtime.sendMessage({ action: 'updateMenu' });
        }

        navigate('/login');
    }, [navigate, updateUI]);

    return (
        <div className="side-panel-container">
            <AnimatePresence>
                {open && (
                    <motion.div
                        className="side-panel"
                        initial="closed"
                        animate="open"
                        exit="closed"
                        variants={panelVariants}
                        onMouseLeave={() => setOpen(false)}
                    >
                        <div
                            className="menu-toggle"
                            onClick={() => setOpen(!open)}
                            aria-label="Toggle menu"
                        >
                            <X size={24} color="#fff" />
                        </div>
                        <nav className="menu-items">
                            {links.map(({ to, label, icon: Icon }) => (
                                <NavLink key={to} to={to} className={({ isActive }) => isActive ? 'active' : ''}>
                                    <span className="icon-container"><Icon /></span>
                                    {label}
                                </NavLink>
                            ))}
                        </nav>
                        <button className="logout-btn" onClick={handleLogout}>
                            Logout
                        </button>
                    </motion.div>
                )}
            </AnimatePresence>
            {!open && (
                <div
                    className="closed-menu"
                    onMouseEnter={() => setOpen(true)}
                >
                    <div
                        className="menu-toggle"
                        aria-label="Toggle menu"
                    >
                        <Menu size={24} color="#fff" />
                    </div>
                    {links.map(({ to, icon: Icon }) => (
                        <NavLink key={to} to={to}>
                            <span className="icon-container"><Icon /></span>
                        </NavLink>
                    ))}
                </div>
            )}
        </div>
    );
}
export default React.memo(SidePanel);
// export default SidePanel;