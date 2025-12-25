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
        <div style={{
            minHeight: '100vh',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            padding: '16px',
            backgroundColor: '#0F172A',
            fontFamily: 'Inter, sans-serif'
        }}>
            <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;900&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet" />

            <div style={{ width: '100%', maxWidth: '448px' }}>
                <div style={{
                    backgroundColor: '#1E293B',
                    borderRadius: '24px',
                    boxShadow: '8px 8px 0px rgba(15, 23, 42, 0.3)',
                    overflow: 'hidden'
                }}>
                    {/* Header */}
                    <div style={{
                        padding: '40px 32px',
                        textAlign: 'center',
                        background: 'linear-gradient(135deg, #0F172A 0%, #1E293B 100%)',
                        borderBottom: '3px solid #F97316'
                    }}>
                        <div style={{ marginBottom: '16px' }}>
                            <img
                                src="/static/logo.svg"
                                alt="FreeRadical"
                                style={{
                                    width: '80px',
                                    height: '80px',
                                    margin: '0 auto',
                                    display: 'block'
                                }}
                            />
                        </div>
                        <h1 style={{
                            fontSize: '36px',
                            fontWeight: 900,
                            color: 'white',
                            marginBottom: '8px',
                            letterSpacing: '-0.04em'
                        }}>
                            FreeRadical
                        </h1>
                        <p style={{
                            color: '#F97316',
                            fontSize: '14px',
                            fontWeight: 600
                        }}>
                            Full Admin Interface
                        </p>
                    </div>

                    {/* Form */}
                    <div style={{ padding: '40px 32px' }}>
                        {error && (
                            <div style={{
                                marginBottom: '24px',
                                padding: '16px',
                                borderRadius: '12px',
                                backgroundColor: '#991B1B',
                                borderLeft: '4px solid #DC2626',
                                boxShadow: '4px 4px 0px rgba(15, 23, 42, 0.2)'
                            }}>
                                <p style={{
                                    fontSize: '14px',
                                    color: 'white',
                                    fontWeight: 500,
                                    margin: 0
                                }}>{error}</p>
                            </div>
                        )}

                        <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: '24px' }}>
                            <div>
                                <label style={{
                                    display: 'block',
                                    fontSize: '14px',
                                    fontWeight: 600,
                                    color: 'white',
                                    marginBottom: '8px',
                                    letterSpacing: '-0.02em'
                                }}>
                                    Email Address
                                </label>
                                <input
                                    type="email"
                                    value={email}
                                    onChange={(e) => setEmail(e.target.value)}
                                    required
                                    style={{
                                        width: '100%',
                                        padding: '14px 16px',
                                        backgroundColor: '#0F172A',
                                        border: '2px solid #334155',
                                        borderRadius: '12px',
                                        color: 'white',
                                        fontFamily: 'JetBrains Mono, monospace',
                                        fontSize: '14px',
                                        outline: 'none',
                                        transition: 'border-color 0.2s'
                                    }}
                                    onFocus={(e) => e.target.style.borderColor = '#F97316'}
                                    onBlur={(e) => e.target.style.borderColor = '#334155'}
                                    placeholder="admin@freeradical.dev"
                                />
                            </div>

                            <div>
                                <label style={{
                                    display: 'block',
                                    fontSize: '14px',
                                    fontWeight: 600,
                                    color: 'white',
                                    marginBottom: '8px',
                                    letterSpacing: '-0.02em'
                                }}>
                                    Password
                                </label>
                                <input
                                    type="password"
                                    value={password}
                                    onChange={(e) => setPassword(e.target.value)}
                                    required
                                    style={{
                                        width: '100%',
                                        padding: '14px 16px',
                                        backgroundColor: '#0F172A',
                                        border: '2px solid #334155',
                                        borderRadius: '12px',
                                        color: 'white',
                                        fontFamily: 'JetBrains Mono, monospace',
                                        fontSize: '14px',
                                        outline: 'none',
                                        transition: 'border-color 0.2s'
                                    }}
                                    onFocus={(e) => e.target.style.borderColor = '#F97316'}
                                    onBlur={(e) => e.target.style.borderColor = '#334155'}
                                    placeholder="••••••••"
                                />
                            </div>

                            <button
                                type="submit"
                                disabled={loading}
                                style={{
                                    width: '100%',
                                    marginTop: '32px',
                                    padding: '16px 24px',
                                    backgroundColor: '#F97316',
                                    color: 'white',
                                    border: 'none',
                                    borderRadius: '12px',
                                    fontSize: '16px',
                                    fontWeight: 700,
                                    letterSpacing: '-0.02em',
                                    cursor: loading ? 'not-allowed' : 'pointer',
                                    opacity: loading ? 0.5 : 1,
                                    boxShadow: loading ? '4px 4px 0px rgba(15, 23, 42, 0.2)' : '8px 8px 0px rgba(15, 23, 42, 0.3)',
                                    transition: 'all 0.2s'
                                }}
                            >
                                {loading ? 'Signing in...' : 'Sign In'}
                            </button>
                        </form>

                        <div style={{
                            marginTop: '32px',
                            paddingTop: '24px',
                            borderTop: '1px solid #334155',
                            textAlign: 'center'
                        }}>
                            <a href="/" style={{
                                color: '#F97316',
                                fontSize: '14px',
                                fontWeight: 600,
                                textDecoration: 'none'
                            }}>
                                ← Back to Home
                            </a>
                        </div>
                    </div>
                </div>

                {/* Footer */}
                <div style={{
                    marginTop: '32px',
                    textAlign: 'center',
                    display: 'flex',
                    flexDirection: 'column',
                    gap: '8px'
                }}>
                    <p style={{
                        fontSize: '14px',
                        fontWeight: 500,
                        color: 'white',
                        margin: 0,
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center'
                    }}>
                        <svg
                            style={{
                                width: '16px',
                                height: '16px',
                                marginRight: '8px'
                            }}
                            fill="#06B6D4"
                            viewBox="0 0 20 20"
                        >
                            <path fillRule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clipRule="evenodd" />
                        </svg>
                        <span style={{ color: '#06B6D4' }}>Secured with JWT & Argon2</span>
                    </p>
                    <p style={{
                        fontSize: '12px',
                        color: '#334155',
                        fontFamily: 'JetBrains Mono, monospace',
                        margin: 0
                    }}>
                        v1.0.3 • Powered by Rust & Actix-Web
                    </p>
                </div>
            </div>
        </div>
    );
}
