// File: src/pages/Login.jsx
import React, { useState } from 'react';
import { useNavigate, Link } from 'react-router-dom';
import { motion } from 'framer-motion';
import './Auth.css';
import { useTheme } from '../context/ThemeContext';
import ThemeToggle from '../components/ThemeToggle';


const Login = () => {
    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    const [msg, setMsg] = useState('');
    const navigate = useNavigate();
    const API_BASE_URL = import.meta.env.VITE_API_URL;

    const handleSubmit = async e => {
        e.preventDefault();
        setMsg('Logging in...');
        try {
            const res = await fetch(`${API_BASE_URL}/v1/auth/login/user`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ username, password })
            });
            const data = await res.json();
            if (res.ok) {
                localStorage.setItem('user_token', data.token);
                localStorage.setItem('username', data.username);
                navigate('/welcome');
            } else {
                setMsg(data.error || 'Invalid credentials');
            }
        } catch (err) {
            setMsg(err.message);
        }
    };

    return (
        <div className="welcome-bg">
            <ThemeToggle />
            {/* Theme toggle is now handled globally */}
            <motion.div
                className="auth-container"
                initial={{ opacity: 0, x: -100 }}
                animate={{ opacity: 1, x: 0 }}
                exit={{ opacity: 0, x: 100 }}
                transition={{ duration: 0.6, ease: "easeOut", 
                // backdropFilter: blur ("10px"),
                width: "100%",
                }}
            >
                <motion.h2
                    initial={{ x: -50, opacity: 0 }}
                    animate={{ x: 0, opacity: 1 }}
                    transition={{ delay: 0.2 }}
                >
                    Login
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
                        autoComplete="username"
                        value={username}
                        onChange={e => setUsername(e.target.value)}
                        required
                    />
                    <input
                        type="password"
                        placeholder="Password"
                        value={password}
                        onChange={e => setPassword(e.target.value)}
                        required
                    />
                    <button type="submit" className='button-auth'>Login</button>
                </motion.form>
                <motion.p
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    transition={{ delay: 0.6 }}
                >
                    Don't have an account?{' '}
                    <Link to="/signup" className="link-animate">
                        Sign Up
                    </Link>
                </motion.p>
                <div className="msg">{msg}</div>
            </motion.div>
        </div>
    );
}
export default React.memo(Login);
// export default Login;