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
        <>
            <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;900&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet" />

            <div className="min-h-screen flex items-center justify-center p-4" style={{ backgroundColor: '#0F172A', fontFamily: 'Inter, sans-serif' }}>
                <div className="w-full max-w-md">
                    <div style={{
                        backgroundColor: '#1E293B',
                        borderRadius: '24px',
                        boxShadow: '8px 8px 0px rgba(249, 115, 22, 0.2)',
                        overflow: 'hidden'
                    }}>
                        {/* Header */}
                        <div className="px-8 py-12 text-center relative" style={{
                            background: 'linear-gradient(135deg, #F97316 0%, #EA580C 100%)'
                        }}>
                            <div className="absolute top-4 right-4 w-16 h-16 rounded-full" style={{
                                background: 'radial-gradient(circle, rgba(255,255,255,0.1) 0%, transparent 70%)'
                            }}></div>

                            <div className="inline-block mb-4 relative z-10">
                                <img src="/logo.svg" alt="FreeRadical" className="w-24 h-24 mx-auto" style={{
                                    filter: 'drop-shadow(4px 4px 0px rgba(15, 23, 42, 0.3))'
                                }} />
                            </div>
                            <h1 className="text-5xl font-black text-white mb-2" style={{
                                letterSpacing: '-0.04em',
                                fontWeight: 900,
                                textShadow: '4px 4px 0px rgba(15, 23, 42, 0.3)'
                            }}>
                                FreeRadical
                            </h1>
                            <p className="text-white text-base font-bold" style={{ letterSpacing: '0.05em' }}>
                                ADMIN PORTAL
                            </p>
                        </div>

                        {/* Form */}
                        <div className="px-8 py-10">
                            {error && (
                                <div className="mb-6 p-4 rounded-xl" style={{
                                    backgroundColor: '#7F1D1D',
                                    border: '2px solid #991B1B',
                                    boxShadow: '4px 4px 0px rgba(15, 23, 42, 0.2)'
                                }}>
                                    <div className="flex items-center">
                                        <svg className="w-5 h-5 mr-2" fill="#DC2626" viewBox="0 0 20 20">
                                            <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clipRule="evenodd" />
                                        </svg>
                                        <p className="text-sm text-white font-semibold">{error}</p>
                                    </div>
                                </div>
                            )}

                            <form onSubmit={handleSubmit} className="space-y-5">
                                <div>
                                    <label className="block text-sm font-bold text-white mb-2" style={{
                                        letterSpacing: '-0.02em',
                                        textTransform: 'uppercase',
                                        fontSize: '11px',
                                        color: '#F97316'
                                    }}>
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
                                            fontFamily: 'JetBrains Mono, monospace',
                                            fontSize: '14px'
                                        }}
                                        className="w-full px-4 py-4 outline-none transition-all focus:border-[#F97316] focus:shadow-[0_0_0_3px_rgba(249,115,22,0.1)]"
                                        placeholder="admin@freeradical.dev"
                                    />
                                </div>

                                <div>
                                    <label className="block text-sm font-bold text-white mb-2" style={{
                                        letterSpacing: '-0.02em',
                                        textTransform: 'uppercase',
                                        fontSize: '11px',
                                        color: '#F97316'
                                    }}>
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
                                            fontFamily: 'JetBrains Mono, monospace',
                                            fontSize: '14px'
                                        }}
                                        className="w-full px-4 py-4 outline-none transition-all focus:border-[#F97316] focus:shadow-[0_0_0_3px_rgba(249,115,22,0.1)]"
                                        placeholder="••••••••"
                                    />
                                </div>

                                <button
                                    type="submit"
                                    disabled={loading}
                                    style={{
                                        background: 'linear-gradient(135deg, #F97316 0%, #EA580C 100%)',
                                        borderRadius: '12px',
                                        boxShadow: loading ? 'none' : '8px 8px 0px rgba(15, 23, 42, 0.4)',
                                        fontWeight: 900,
                                        letterSpacing: '0.05em',
                                        textTransform: 'uppercase',
                                        fontSize: '14px'
                                    }}
                                    className="w-full mt-8 text-white py-5 px-6 transition-all hover:shadow-[4px_4px_0px_rgba(15,23,42,0.4)] active:shadow-none active:translate-x-1 active:translate-y-1 disabled:opacity-50 disabled:cursor-not-allowed"
                                >
                                    {loading ? (
                                        <span className="flex items-center justify-center">
                                            <svg className="animate-spin -ml-1 mr-3 h-5 w-5" fill="none" viewBox="0 0 24 24">
                                                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                                                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                            </svg>
                                            Authenticating...
                                        </span>
                                    ) : 'Access System'}
                                </button>
                            </form>

                            <div className="mt-8 pt-6 text-center" style={{ borderTop: '1px solid #334155' }}>
                                <a href="/" className="text-sm font-bold transition-colors inline-flex items-center" style={{ color: '#06B6D4', letterSpacing: '0.05em' }}>
                                    <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" strokeWidth={2}>
                                        <path strokeLinecap="square" strokeLinejoin="miter" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
                                    </svg>
                                    HOMEPAGE
                                </a>
                            </div>
                        </div>
                    </div>

                    {/* Footer */}
                    <div className="mt-8 text-center space-y-3">
                        <div className="flex items-center justify-center space-x-4">
                            <div className="flex items-center" style={{ color: '#06B6D4' }}>
                                <svg className="w-4 h-4 mr-2" fill="currentColor" viewBox="0 0 20 20" strokeWidth={2}>
                                    <path fillRule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clipRule="evenodd" />
                                </svg>
                                <span className="text-xs font-bold" style={{ letterSpacing: '0.05em' }}>JWT + ARGON2</span>
                            </div>
                            <div style={{ color: '#334155' }}>|</div>
                            <div className="text-xs font-mono" style={{ color: '#334155', fontFamily: 'JetBrains Mono' }}>
                                v1.0.3
                            </div>
                        </div>
                        <p className="text-xs font-bold" style={{
                            color: '#334155',
                            letterSpacing: '0.1em'
                        }}>
                            RUST • ACTIX-WEB • DIESEL
                        </p>
                    </div>
                </div>
            </div>
        </>
    );
}
