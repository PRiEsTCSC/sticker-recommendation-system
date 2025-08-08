// File: src/pages/Signup.jsx
import React, { useState, useEffect } from 'react'; // Corrected import syntax
import { useNavigate, Link } from 'react-router-dom';
import { motion } from 'framer-motion';
import './Auth.css'; // The shared CSS for auth components
import ThemeToggle from '../components/ThemeToggle';


const SignUp = () => {
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [msg, setMsg] = useState('');
    const [theme, setTheme] = useState('dark'); // Default theme
    const navigate = useNavigate();
    const API_BASE_URL = import.meta.env.VITE_API_URL;

    // Effect to apply theme from localStorage on component mount
    useEffect(() => {
        const storedTheme = localStorage.getItem('theme');
        if (storedTheme) {
            setTheme(storedTheme);
            document.body.classList.add(`theme-${storedTheme}`);
        } else {
            // Default to dark mode if no preference is stored
            setTheme('dark');
            document.body.classList.add('theme-dark');
            localStorage.setItem('theme', 'dark');
        }
    }, []);

    // Effect to update body class when theme state changes
    useEffect(() => {
        document.body.className = ''; // Clear existing classes
        document.body.classList.add(`theme-${theme}`);
        localStorage.setItem('theme', theme); // Persist theme
    }, [theme]);

    const handleThemeToggle = () => {
        setTheme(prevTheme => (prevTheme === 'dark' ? 'light' : 'dark'));
    };

    const handleSubmit = async e => {
        e.preventDefault();
        setMsg('Signing up...');
        try {
            const res = await fetch(`${API_BASE_URL}/v1/auth/register/user`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ username, password })
            });
            const data = await res.json();
            if (res.ok) {
                localStorage.setItem('user_token', data.token);
                localStorage.setItem('username', data.username);
                navigate('/welcome'); // Redirect to welcome page
            } else {
                setMsg(data.error || 'Signup failed');
            }
        } catch (err) {
            setMsg(err.message);
        }
    };

    return (
        <div className="welcome-bg"> {/* Wrapper for theme background */}
        <ThemeToggle />

            <motion.div
                className="auth-container"
                initial={{ opacity: 0, x: 100 }} // Animate from right (for signup from left side)
                animate={{ opacity: 1, x: 0 }}    // Animate to center
                exit={{ opacity: 0, x: -100 }}    // Animate out to left
                transition={{ duration: 0.6, ease: "easeOut" }}
            >
                <motion.h2
                    initial={{ x: 50, opacity: 0 }}
                    animate={{ x: 0, opacity: 1 }}
                    transition={{ delay: 0.2 }}
                >
                    Sign Up
                </motion.h2>
                <motion.form
                    onSubmit={handleSubmit}
                    initial={{ y: 20, opacity: 0 }}
                    animate={{ y: 0, opacity: 1 }}
                    transition={{ delay: 0.4 }}
                >
                    <input
                        type="text"
                        placeholder="Username"
                        name='username'
                        value={username}
                        autoComplete="username"
                        onChange={e => setUsername(e.target.value)}
                        required
                    />
                    <input
                        type="password"
                        placeholder="Password"
                        value={password}
                        // onChange={e => e.target.value = e.target.value.trim().replace(/\s/g, '')}
                        // made the user input password unwritable
                        onChange={e => setPassword(e.target.value)}
                        
                        required
                    />
                    <button type="submit">Sign Up</button>
                </motion.form>
                <motion.p
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    transition={{ delay: 0.6 }}
                >
                    Already have an account?{' '}
                    <Link to="/login" className="link-animate">
                        Log In
                    </Link>
                </motion.p>
                <div className="msg">{msg}</div>
            </motion.div>
        </div>
    );
}

export default React.memo(SignUp);

// export default SignUp;