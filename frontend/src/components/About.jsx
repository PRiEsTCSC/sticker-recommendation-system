// src/pages/About.jsx
import React, {useCallback}from 'react';
import { motion } from 'framer-motion';
import SidePanel from '../components/SidePanel';
import ThemeToggle from '../components/ThemeToggle';
import './About.css';

// export default function About() {
const About = () => {
    // const { theme } = useTheme();
    const updateUI = useCallback(() => {}, []);

    return (
        <div className="about-bg">
            <SidePanel updateUI={updateUI} />
            <ThemeToggle />
            
            <div className="about-container">
                <motion.div
                    className="about-content"
                    initial={{ opacity: 0, y: 50 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ duration: 0.5 }}
                >
                    <h1>About Us</h1>
                    <p className="intro-text">
                        Our application is designed to provide a seamless experience for users to discover and share
                        the latest stickers and content. We focus on creating an intuitive interface that
                        adapts to your preferences and needs.
                    </p>
                    <div className="about-section">
                        <h2>Our Mission</h2>
                        <p>
                            To create a platform where users can easily find and share the most relevant and engaging
                            content, with a focus on simplicity and user experience.
                        </p>
                    </div>
                    <div className="about-section">
                        <h2>Our Vision</h2>
                        <p>
                            To become the go-to platform for discovering and sharing digital content, making it
                            accessible and enjoyable for everyone.
                        </p>
                    </div>
                    <div className="about-section">
                        <h2>Our Team</h2>
                        <p>
                            Our team is composed of passionate developers and designers who are dedicated to
                            creating a platform that enhances the way people interact with digital content.
                        </p>
                    </div>
                </motion.div>
            </div>
        </div>
    );
}

export default React.memo(About);