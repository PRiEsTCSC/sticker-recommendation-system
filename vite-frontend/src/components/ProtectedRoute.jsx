// src/components/ProtectedRoute.jsx
import React, { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

const ProtectedRoute = ({ children }) => {
    const navigate = useNavigate();

    
    useEffect(() => {
        const userToken = localStorage.getItem('user_token');
        
        if (!userToken) {
            // No token, redirect to login
            navigate('/login');
            return;
        }
        
        // Check if token is expired (JWT format)
        try {
            const tokenParts = userToken.split('.');
            if (tokenParts.length !== 3) {
                throw new Error('Invalid token format');
            }
            
            const payload = JSON.parse(atob(tokenParts[1]));
            const expirationTime = payload.exp * 1000; // Convert to milliseconds
            
            if (Date.now() > expirationTime) {
                // Token has expired
                alert('Session expired, please log in again');
                localStorage.removeItem('user_token');
                localStorage.removeItem('username');
                navigate('/login');
            }
        } catch (error) {
            console.error('Token validation error:', error);
            // Token is invalid, redirect to login
            localStorage.removeItem('user_token');
            localStorage.removeItem('username');
            navigate('/login');
        }
    }, [navigate]);
    
    return children;
};

export default React.memo(ProtectedRoute);