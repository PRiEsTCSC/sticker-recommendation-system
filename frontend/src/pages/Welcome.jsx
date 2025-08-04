import React, { useState, useEffect, memo, useCallback, useMemo } from 'react';
import SidePanel from '../components/SidePanel';
import { motion, AnimatePresence } from 'framer-motion';
import { ResponsiveContainer, RadialBarChart, RadialBar, Legend, Tooltip } from 'recharts';
import { useTheme } from '../context/ThemeContext';
import ThemeToggle from '../components/ThemeToggle';
import './Auth.css';

const toProperCase = str =>
    str
        ? str.replace(/\w\S*/g, txt =>
            txt.charAt(0).toUpperCase() + txt.substr(1).toLowerCase()
        )
        : '';

const chartData = [
    { name: 'Search', count: 120, color: '#8884d8' },
    { name: 'Recommend', count: 85, color: '#82ca9d' },
    { name: 'Profile', count: 45, color: '#ffc658' },
    { name: 'Settings', count: 60, color: '#ff7f0e' },
];

const CircularChart = memo(({ name, count, total, color }) => {
    const data = [
        {
            name: name,
            uv: (count / total) * 100,
            pv: 100,
            fill: color,
            count: count
        }
    ];

    return (
        <div className="chart-card">
            <h3>{name}</h3>
            <ResponsiveContainer width="100%" height="100%">
                <RadialBarChart
                    cx="50%"
                    cy="50%"
                    innerRadius="60%"
                    outerRadius="90%"
                    barSize={10}
                    data={data}
                    startAngle={90}
                    endAngle={-270}
                >
                    <RadialBar
                        minAngle={15}
                        label={{ position: 'insideStart', fill: '#fff', formatter: (value) => `${data[0].count}` }}
                        background
                        clockWise
                        dataKey="uv"
                    />
                    <Tooltip />
                    <Legend
                        iconSize={0}
                        layout="vertical"
                        verticalAlign="middle"
                        align="right"
                        className="chart-legend-hidden"
                    />
                    <text
                        x="50%"
                        y="50%"
                        textAnchor="middle"
                        dominantBaseline="middle"
                        className="chart-label-name"
                        fontWeight="bold"
                    >
                        {name}
                    </text>
                </RadialBarChart>
            </ResponsiveContainer>
            <p className="chart-count-label">Used: {count} times</p>
        </div>
    );
});

CircularChart.displayName = 'CircularChart';

const Welcome = () => {
    const username = localStorage.getItem('username') || 'User';
    const [uiUser, setUiUser] = useState(username);
    const [showCharts, setShowCharts] = useState(false);
    const [welcomeCardAnimation, setWelcomeCardAnimation] = useState({
        opacity: 1,
        y: 0,
        scale: 1,
    });
    const [dashboardTitleVisible, setDashboardTitleVisible] = useState(false);
    const { theme } = useTheme();

    const updateUI = useCallback((token, user) => {
        setUiUser(user);
    }, []);

    const totalUsage = useMemo(() => 120 + 85 + 45 + 60, []);

    useEffect(() => {
        const welcomeCardHideTimer = setTimeout(() => {
            setWelcomeCardAnimation({
                opacity: 0,
                y: -150,
                scale: 0.7,
            });

            const dashboardTitleShowTimer = setTimeout(() => {
                setDashboardTitleVisible(true);
            }, 300);

            const chartShowTimer = setTimeout(() => {
                setShowCharts(true);
            }, 1000);

            return () => {
                clearTimeout(dashboardTitleShowTimer);
                clearTimeout(chartShowTimer);
            };
        }, 3500);

        return () => clearTimeout(welcomeCardHideTimer);
    }, []);

    return (
        <div className="welcome-bg">
            <ThemeToggle />
            <SidePanel updateUI={updateUI} />
            <AnimatePresence>
                {dashboardTitleVisible && (
                    <motion.div
                        className="dashboard-title-card"
                        initial={{ opacity: 0, y: 50 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: -50 }}
                        transition={{ duration: 0.8, ease: "easeOut" }}
                    >
                        <h1>{toProperCase(uiUser)}'s Dashboard</h1>
                    </motion.div>
                )}
            </AnimatePresence>

            <motion.div
                className="welcome-content glass-card"
                initial={{ opacity: 1, y: 0, scale: 1 }}
                animate={welcomeCardAnimation}
                transition={{ duration: 1.0, ease: "easeOut" }}
            >
                <motion.h1
                    initial={{ scale: 0.7, opacity: 0 }}
                    animate={{ scale: 1, opacity: 1 }}
                    transition={{ delay: 0.2 }}
                >
                    <div className='welcome-text2'>
                        <h1>
                            <span>W</span>
                            <span>E</span>
                            <span>L</span>
                            <span>C</span>
                            <span>O</span>
                            <span>M</span>
                            <span>E</span>
                            <span>!</span>
                        </h1>
                    </div>
                </motion.h1>
                <motion.p initial={{ opacity: 0 }} animate={{ opacity: 1 }} transition={{ delay: 0.1 }}>
                    <div className='welcome-text'>
                        <p>This is your dashboard where you can view your history and explore recommendations.</p>
                    </div>
                </motion.p>
            </motion.div>

            <AnimatePresence>
                {showCharts && (
                    <motion.div
                        className="charts-container"
                        initial={{ opacity: 0, y: 50 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: 50 }}
                        transition={{ duration: 0.8, ease: "easeOut" }}
                    >
                        <h2>Usage Overview</h2>
                        <div className="charts-grid">
                            {chartData.map((chart) => (
                                <CircularChart
                                    key={chart.name}
                                    name={chart.name}
                                    count={chart.count}
                                    total={totalUsage}
                                    color={chart.color}
                                />
                            ))}
                        </div>
                    </motion.div>
                )}
            </AnimatePresence>
        </div>
    );
};

Welcome.displayName = 'Welcome';
export default memo(Welcome);