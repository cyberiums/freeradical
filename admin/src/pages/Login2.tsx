import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import axios from 'axios';

export default function Login2() {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [error, setError] = useState('');
    const [loading, setLoading] = useState(false);
    const navigate = useNavigate();

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setError('');
        setLoading(true);

        try {
            const response = await axios.post('http://localhost:8000/auth/login', {
                email,
                password,
            });

            if (response.data.token) {
                localStorage.setItem('auth_token', response.data.token);
                localStorage.setItem('user_email', email);
                navigate('/dashboard');
            }
        } catch (err: any) {
            setError(err.response?.data?.message || 'Invalid email or password');
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="min-h-screen bg-gradient-to-br from-indigo-600 via-purple-600 to-pink-500 flex items-center justify-center p-4">
            <div className="w-full max-w-md">
                <div className="bg-white rounded-3xl shadow-2xl overflow-hidden">
                    {/* Header */}
                    <div className="bg-gradient-to-r from-indigo-600 to-purple-600 px-8 py-12 text-center">
                        <div className="inline-block mb-4">
                            <img src="/logo.svg" alt="FreeRadical" className="w-24 h-24 mx-auto" />
                        </div>
                        <h1 className="text-4xl font-bold text-white mb-2">FreeRadical</h1>
                        <p className="text-indigo-100 text-lg">Full Admin Portal</p>
                    </div>

                    {/* Form */}
                    <div className="px-8 py-10">
                        {error && (
                            <div className="mb-6 p-4 bg-red-50 border-l-4 border-red-500 rounded-lg">
                                <p className="text-sm text-red-700 font-medium">{error}</p>
                            </div>
                        )}

                        <form onSubmit={handleSubmit} className="space-y-5">
                            <div>
                                <label className="block text-sm font-semibold text-gray-700 mb-2">
                                    Email Address
                                </label>
                                <input
                                    type="email"
                                    value={email}
                                    onChange={(e) => setEmail(e.target.value)}
                                    required
                                    className="w-full px-4 py-3.5 bg-gray-50 border border-gray-200 rounded-xl focus:bg-white focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none transition-all"
                                    placeholder="admin@freeradical.io"
                                />
                            </div>

                            <div>
                                <label className="block text-sm font-semibold text-gray-700 mb-2">
                                    Password
                                </label>
                                <input
                                    type="password"
                                    value={password}
                                    onChange={(e) => setPassword(e.target.value)}
                                    required
                                    className="w-full px-4 py-3.5 bg-gray-50 border border-gray-200 rounded-xl focus:bg-white focus:ring-2 focus:ring-indigo-500 focus:border-transparent outline-none transition-all"
                                    placeholder="Enter your password"
                                />
                            </div>

                            <button
                                type="submit"
                                disabled={loading}
                                className="w-full mt-8 bg-gradient-to-r from-indigo-600 to-purple-600 text-white py-4 px-6 rounded-xl font-bold text-lg hover:from-indigo-700 hover:to-purple-700 focus:outline-none focus:ring-4 focus:ring-indigo-300 disabled:opacity-50 disabled:cursor-not-allowed transition-all transform hover:scale-105 shadow-lg"
                            >
                                {loading ? (
                                    <span className="flex items-center justify-center">
                                        <svg className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" fill="none" viewBox="0 0 24 24">
                                            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                                            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                        </svg>
                                        Signing in...
                                    </span>
                                ) : (
                                    'Sign In'
                                )}
                            </button>
                        </form>

                        <div className="mt-8 pt-6 border-t border-gray-100 text-center">
                            <a href="/" className="text-sm text-indigo-600 hover:text-indigo-700 font-semibold">
                                ← Back to Homepage
                            </a>
                        </div>
                    </div>
                </div>

                {/* Footer */}
                <div className="mt-8 text-center space-y-2">
                    <p className="text-white text-sm font-medium flex items-center justify-center">
                        <svg className="w-4 h-4 mr-2" fill="currentColor" viewBox="0 0 20 20">
                            <path fillRule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clipRule="evenodd" />
                        </svg>
                        Secured with JWT & Argon2
                    </p>
                    <p className="text-white text-xs opacity-80">
                        v1.0.3 • Powered by Rust & Actix-Web
                    </p>
                </div>
            </div>
        </div>
    );
}
