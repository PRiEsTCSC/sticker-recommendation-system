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