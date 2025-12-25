import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import axios from 'axios';

export default function Login() {
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
        <>
            {/* Typography: Inter font */}
            <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;900&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet" />

            <div className="min-h-screen flex items-center justify-center p-4" style={{ backgroundColor: '#0F172A', fontFamily: 'Inter, sans-serif' }}>
                <div className="w-full max-w-md">
                    {/* Card with Carbon Gray + Hard Shadow */}
                    <div style={{
                        backgroundColor: '#1E293B',
                        borderRadius: '24px',
                        boxShadow: '8px 8px 0px rgba(15, 23, 42, 0.3)',
                        overflow: 'hidden'
                    }}>
                        {/* Header with Reactive Orange accent */}
                        <div className="px-8 py-10 text-center" style={{
                            background: 'linear-gradient(135deg, #0F172A 0%, #1E293B 100%)',
                            borderBottom: '3px solid #F97316'
                        }}>
                            <div className="inline-block mb-4">
                                <img src="/logo.svg" alt="FreeRadical" className="w-20 h-20 mx-auto" />
                            </div>
                            <h1 className="text-4xl font-black text-white mb-2" style={{
                                letterSpacing: '-0.04em',
                                fontWeight: 900
                            }}>
                                FreeRadical
                            </h1>
                            <p className="text-white text-sm" style={{ color: '#F97316', fontWeight: 600 }}>
                                Full Admin Interface
                            </p>
                        </div>

                        {/* Form */}
                        <div className="px-8 py-10">
                            {error && (
                                <div className="mb-6 p-4 rounded-xl" style={{
                                    backgroundColor: '#991B1B',
                                    borderLeft: '4px solid #DC2626',
                                    boxShadow: '4px 4px 0px rgba(15, 23, 42, 0.2)'
                                }}>
                                    <p className="text-sm text-white font-medium">{error}</p>
                                </div>
                            )}

                            <form onSubmit={handleSubmit} className="space-y-6">
                                <div>
                                    <label className="block text-sm font-semibold text-white mb-2" style={{ letterSpacing: '-0.02em' }}>
                                        Email Address
                                    </label>
                                    <input
                                        type="email"
                                        value={email}
                                        onChange={(e) => setEmail(e.target.value)}
                                        required
                                        style={{
                                            backgroundColor: '#0F172A',
                                            border: '2px solid #334155',
                                            borderRadius: '12px',
                                            color: 'white',
                                            fontFamily: 'JetBrains Mono, monospace'
                                        }}
                                        className="w-full px-4 py-3.5 outline-none transition-all focus:border-[#F97316]"
                                        placeholder="admin@freeradical.dev"
                                    />
                                </div>

                                <div>
                                    <label className="block text-sm font-semibold text-white mb-2" style={{ letterSpacing: '-0.02em' }}>
                                        Password
                                    </label>
                                    <input
                                        type="password"
                                        value={password}
                                        onChange={(e) => setPassword(e.target.value)}
                                        required
                                        style={{
                                            backgroundColor: '#0F172A',
                                            border: '2px solid #334155',
                                            borderRadius: '12px',
                                            color: 'white',
                                            fontFamily: 'JetBrains Mono, monospace'
                                        }}
                                        className="w-full px-4 py-3.5 outline-none transition-all focus:border-[#F97316]"
                                        placeholder="••••••••"
                                    />
                                </div>

                                <button
                                    type="submit"
                                    disabled={loading}
                                    style={{
                                        backgroundColor: '#F97316',
                                        borderRadius: '12px',
                                        boxShadow: loading ? '4px 4px 0px rgba(15, 23, 42, 0.2)' : '8px 8px 0px rgba(15, 23, 42, 0.3)',
                                        fontWeight: 700,
                                        letterSpacing: '-0.02em'
                                    }}
                                    className="w-full mt-8 text-white py-4 px-6 transition-all hover:shadow-[4px_4px_0px_rgba(15,23,42,0.3)] disabled:opacity-50"
                                >
                                    {loading ? 'Signing in...' : 'Sign In'}
                                </button>
                            </form>

                            <div className="mt-8 pt-6 text-center" style={{ borderTop: '1px solid #334155' }}>
                                <a href="/" className="text-sm font-semibold transition-colors" style={{ color: '#F97316' }}>
                                    ← Back to Home
                                </a>
                            </div>
                        </div>
                    </div>

                    {/* Footer with Rust Cyan accent */}
                    <div className="mt-8 text-center space-y-2">
                        <p className="text-sm font-medium text-white flex items-center justify-center">
                            <svg className="w-4 h-4 mr-2" fill="#06B6D4" viewBox="0 0 20 20">
                                <path fillRule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clipRule="evenodd" />
                            </svg>
                            <span style={{ color: '#06B6D4' }}>Secured with JWT & Argon2</span>
                        </p>
                        <p className="text-xs" style={{
                            color: '#334155',
                            fontFamily: 'JetBrains Mono, monospace'
                        }}>
                            v1.0.3 • Powered by Rust & Actix-Web
                        </p>
                    </div>
                </div>
            </div>
        </>
    );
}
