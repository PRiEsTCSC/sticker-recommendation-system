// File: src/components/ThemeToggle.jsx
import React from 'react';
import { useTheme } from '../context/ThemeContext';
import { Sun, MoonStar } from 'lucide-react';
import '../pages/Auth.css'; // Make sure this imports your CSS

const ThemeToggle = () => {
    const { theme, toggleTheme } = useTheme();

    return (
        <div className="theme-toggle-container">
            <div 
                className={`theme-switch ${theme === 'dark' ? 'dark-mode' : ''}`} 
                onClick={toggleTheme}
            >
                <div className="theme-switch-indicator">
                    <Sun size={24} className="theme-switch-icon sun" />
                    <MoonStar size={24} className="theme-switch-icon moon" />
                </div>
                <span className="theme-switch-label light"></span>
                <span className="theme-switch-label dark"></span>
            </div>
        </div>
    );
};

export default React.memo(ThemeToggle);