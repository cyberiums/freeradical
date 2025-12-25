import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import Login from './pages/Login'
import Login2 from './pages/Login2'
import Dashboard from './pages/Dashboard'

const queryClient = new QueryClient()

function App() {
    // TODO: Add authentication state management
    const isAuthenticated = false

    return (
        <QueryClientProvider client={queryClient}>
            <BrowserRouter>
                <Routes>
                    <Route path="/login" element={<Login />} />
                    <Route path="/login2" element={<Login2 />} />
                    <Route
                        path="/dashboard"
                        element={isAuthenticated ? <Dashboard /> : <Navigate to="/login" />}
                    />
                    <Route path="/" element={<Navigate to="/dashboard" />} />
                </Routes>
            </BrowserRouter>
        </QueryClientProvider>
    )
}

export default App
