// src/pages/Contact.jsx
import React, {useCallback} from 'react';
import { motion } from 'framer-motion';
import SidePanel from '../components/SidePanel';
import ThemeToggle from '../components/ThemeToggle';
import './Contact.css';

// Import images
import githubIcon from '../assets/github.svg';
import emailIcon from '../assets/email.svg';
import phoneIcon from '../assets/phone.svg';




const Contact = () => {
    const updateUI = useCallback(() => {}, []);

    return (
        
        <div className="contact-bg">
            <SidePanel updateUI={updateUI} />
            <ThemeToggle />

            <div className="contact-container">
                <div className='welcome-text2'>
                    <h1>
                        <span>C</span>
                        <span>o</span>
                        <span>n</span>
                        <span>t</span>
                        <span>a</span>
                        <span>c</span>
                        <span>t</span>
                        <span>&nbsp; &nbsp;</span>
                        <span>U</span>
                        <span>s</span>
                    </h1>
                </div>
                
                <motion.ul
                    className="contact-menu"
                    initial={{ opacity: 0, y: 50 }}
                    animate={{ opacity: 1, y: 0 }}
                    transition={{ duration: 0.8, ease: "easeOut" }}
                >
                    <div className="contact-list">
                        <ul>
                            <li>
                                <a href="https://github.com/PRiEsTCSC" target="_blank" rel="noopener noreferrer">
                                    <img src={githubIcon} alt="Github" className="contact-icon" />
                                    <span>- Github</span>
                                </a>
                            </li>

                            <li>
                                <a href="mailto:noblegabriel40@gmail.com">
                                    <img src={emailIcon} alt="Email" className="contact-icon" />
                                    <span>- Email</span>
                                </a>
                            </li>

                            <li>
                                <a href="tel:+2348148184636">
                                    <img src={phoneIcon} alt="Phone" className="contact-icon" />
                                    <span>- Phone</span>
                                </a>
                            </li>
                        </ul>
                    </div>
                </motion.ul>
            </div>
        </div>
    );
}

export default React.memo(Contact);