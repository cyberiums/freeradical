import { createContext, useContext, useState, useEffect, ReactNode } from 'react'
import api from '../lib/api'

interface User {
    id: number
    email: string
    first_name: string
    last_name: string
}

interface AuthContextType {
    user: User | null
    login: (email: string, password: string) => Promise<void>
    logout: () => void
    isAuthenticated: boolean
}

const AuthContext = createContext<AuthContextType | undefined>(undefined)

export function AuthProvider({ children }: { children: ReactNode }) {
    const [user, setUser] = useState<User | null>(null)
    const [isAuthenticated, setIsAuthenticated] = useState(false)

    useEffect(() => {
        // Check for existing token
        const token = localStorage.getItem('auth_token')
        if (token) {
            // Validate token and fetch user
            api.auth.me()
                .then(response => {
                    setUser(response.data)
                    setIsAuthenticated(true)
                })
                .catch(() => {
                    localStorage.removeItem('auth_token')
                })
        }
    }, [])

    const login = async (email: string, password: string) => {
        const response = await api.auth.login(email, password)
        const { token, user } = response.data

        localStorage.setItem('auth_token', token)
        setUser(user)
        setIsAuthenticated(true)
    }

    const logout = () => {
        localStorage.removeItem('auth_token')
        setUser(null)
        setIsAuthenticated(false)
    }

    return (
        <AuthContext.Provider value={{ user, login, logout, isAuthenticated }}>
            {children}
        </AuthContext.Provider>
    )
}

export function useAuth() {
    const context = useContext(AuthContext)
    if (context === undefined) {
        throw new Error('useAuth must be used within an AuthProvider')
    }
    return context
}
