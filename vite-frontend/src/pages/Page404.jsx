// src/pages/404.jsx
import React, { useEffect } from 'react';
import './Page404.css';

const Page404 = () => {
    useEffect(() => {
        const overlay = document.getElementById("overlay");
        const handleMouseMove = (e) => {
            const x = e.clientX;
            const y = e.clientY;
            const pos = `${x}px ${y}px`;
            overlay.style.maskImage = `radial-gradient(circle 120px at ${pos}, transparent 0%, #000 150px)`;
            overlay.style.webkitMaskImage = overlay.style.maskImage;
        };

        window.addEventListener("mousemove", handleMouseMove);

        // Cleanup event listener on component unmount
        return () => {
            window.removeEventListener("mousemove", handleMouseMove);
        };
    }, []);

    return (
        <div className="relative">
            <div className="content">
                <h1>Page Not Found</h1>
                <p>Sorry, we couldn’t find the page you’re looking for.</p>
                <a href="/welcome">Go Home</a>
            </div>
            <div id="overlay"></div>
        </div>
    );
};

export default React.memo(Page404);