// src/context/ThemeContext.jsx
import React, { createContext, useContext, useState, useEffect } from 'react';

const ThemeContext = createContext();

export const useTheme = () => useContext(ThemeContext);

export default function ThemeProvider({ children }) {
    const [theme, setTheme] = useState('dark');

    useEffect(() => {
        const storedTheme = localStorage.getItem('theme') || 'dark';
        setTheme(storedTheme);
        document.body.classList.add(`theme-${storedTheme}`);
    }, []);

    const toggleTheme = () => {
        const newTheme = theme === 'dark' ? 'light' : 'dark';
        setTheme(newTheme);
        document.body.className = '';
        document.body.classList.add(`theme-${newTheme}`);
        localStorage.setItem('theme', newTheme);
    };

    // ✅ Add this explicit setTheme function
    const setThemeExplicit = (newTheme) => {
        setTheme(newTheme);
        document.body.className = '';
        document.body.classList.add(`theme-${newTheme}`);
        localStorage.setItem('theme', newTheme);
    };

    return (
        <ThemeContext.Provider value={{ theme, toggleTheme, setTheme: setThemeExplicit }}>
            {children}
        </ThemeContext.Provider>
    );
}