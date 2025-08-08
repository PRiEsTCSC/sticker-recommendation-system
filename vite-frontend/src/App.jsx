// File: src/App.jsx
import React, {Suspense, lazy} from 'react';
import { Routes, Route, Navigate, useLocation } from 'react-router-dom';
import { AnimatePresence } from 'framer-motion';
import ThemeProvider from './context/ThemeContext';
import ProtectedRoute from './components/ProtectedRoute';

// Lazy load pages and components
const Login = lazy(() => import('./pages/Login'));
const Signup = lazy(() => import('./pages/Signup'));
const Welcome = lazy(() => import('./pages/Welcome'));
const Profile = lazy(() => import('./components/Profile'));
const Recommend = lazy(() => import('./components/Recommend'));
const Contact = lazy(() => import('./components/Contact'));
const SearchPage = lazy(() => import('./components/Search'));
const Page404 = lazy(() => import('./pages/Page404'));
const About = lazy(() => import('./components/About'));
const History = lazy(() => import('./components/History'));


// A simple loading indicator for the Suspense fallback
const LoadingIndicator = () => (
  <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh', background: '#1a1a1a', color: 'white' }}>
    <h2>Loading...</h2>
  </div>
);


export default function App() {
  const token = localStorage.getItem('user_token');
  const location = useLocation();

  return (
    <ThemeProvider>
      <AnimatePresence exitBeforeEnter>
      <Suspense fallback={<LoadingIndicator />}>
          <Routes location={location} key={location.pathname}>
          <Route path="/login" element={!token ? <Login /> : <Navigate to="/welcome" />} />
          <Route path="/signup" element={!token ? <Signup /> : <Navigate to="/welcome" />} />
          <Route path="/welcome" element={token ? <ProtectedRoute><Welcome /> </ProtectedRoute>: <Navigate to="/login" />} />
          <Route path="/home" element={<Navigate to={token ? '/welcome' : '/login'} />} />
          <Route path="/profile" element={<ProtectedRoute><Profile /></ProtectedRoute>} />
          <Route path="/search" element={<ProtectedRoute><SearchPage /></ProtectedRoute>} />
          <Route path="/recommend" element={<ProtectedRoute><Recommend /></ProtectedRoute>} />
          <Route path="/contact" element={<ProtectedRoute><Contact /></ProtectedRoute>} />
          <Route path="/about" element={<ProtectedRoute><About /></ProtectedRoute>} />
          <Route path="/history" element={<ProtectedRoute><History /></ProtectedRoute>} />

          {/* Fallback route for 404 */}
          <Route path="/404" element={<Page404 />} />
          <Route path="*" element={<Navigate to="/404" replace />} />

        </Routes>
        </Suspense>
      </AnimatePresence>
    </ThemeProvider>

  );
}