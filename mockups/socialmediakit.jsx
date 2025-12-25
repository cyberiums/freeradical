import React, { useState } from 'react';
import { Twitter, Linkedin, Facebook, Youtube, Download, Layout, ShieldCheck, Zap } from 'lucide-react';

const PlatformPreview = ({ platform, width, height, safeZone, children }) => {
    const [showSafeZone, setShowSafeZone] = useState(false);
    const aspectRatio = width / height;

    return (
        <div className="mb-12">
            <div className="flex justify-between items-center mb-4">
                <h3 className="text-xl font-black text-slate-900 flex items-center gap-2">
                    {platform === 'X (Twitter)' && <Twitter className="w-5 h-5" />}
                    {platform === 'LinkedIn' && <Linkedin className="w-5 h-5" />}
                    {platform === 'Facebook' && <Facebook className="w-5 h-5" />}
                    {platform === 'YouTube' && <Youtube className="w-5 h-5" />}
                    {platform}
                    <span className="text-sm font-bold text-slate-400 ml-2">({width}x{height})</span>
                </h3>
                <button
                    onClick={() => setShowSafeZone(!showSafeZone)}
                    className="text-xs font-black uppercase tracking-widest text-orange-600 hover:text-orange-700 flex items-center gap-1"
                >
                    <ShieldCheck className="w-4 h-4" /> {showSafeZone ? 'Hide Safe Zone' : 'Show Safe Zone'}
                </button>
            </div>

            <div
                className="relative bg-slate-900 overflow-hidden border-4 border-slate-200 shadow-xl rounded-xl"
                style={{ aspectRatio: `${width}/${height}`, width: '100%' }}
            >
                {/* Background from generated image */}
                <div
                    className="absolute inset-0 bg-cover bg-center opacity-80"
                    style={{ backgroundImage: `url('http://googleusercontent.com/image_generation_content/0')` }}
                />

                {/* Safe Zone Overlay */}
                {showSafeZone && (
                    <div className="absolute inset-0 z-10 border-2 border-dashed border-cyan-400 pointer-events-none flex items-center justify-center">
                        <div
                            className="bg-cyan-400/10 border-2 border-cyan-400"
                            style={{ width: safeZone.w, height: safeZone.h }}
                        />
                    </div>
                )}

                {/* Content Overlay */}
                <div className="absolute inset-0 flex flex-col items-center justify-center text-white z-20">
                    <div className="flex items-center gap-3 mb-2 scale-110">
                        <div className="w-10 h-10 bg-orange-600 rounded-lg flex items-center justify-center rotate-3 shadow-lg">
                            <Zap className="w-6 h-6" />
                        </div>
                        <span className="text-3xl font-black tracking-tighter">FreeRadical</span>
                    </div>
                    <p className="text-[10px] md:text-sm font-black uppercase tracking-[0.3em] text-orange-400">
                        1000x Faster. SEO Dominant.
                    </p>
                </div>

                {/* Platform Specific Mock UI */}
                {platform === 'X (Twitter)' && (
                    <div className="absolute bottom-4 left-6 w-16 h-16 rounded-full border-4 border-slate-900 bg-slate-700 z-30" />
                )}
                {platform === 'LinkedIn' && (
                    <div className="absolute bottom-[-20px] left-8 w-24 h-24 rounded-lg border-4 border-white bg-slate-700 z-30" />
                )}
            </div>
        </div>
    );
};

const App = () => {
    return (
        <div className="min-h-screen bg-slate-50 p-8 font-sans">
            <header className="max-w-4xl mx-auto mb-12 text-center">
                <h1 className="text-4xl font-black text-slate-900 mb-4">Social Media Asset Kit</h1>
                <p className="text-slate-500 font-medium">Preview and optimize your brand for every platform. Use the Safe Zone overlays to ensure your logo is never obstructed.</p>
            </header>

            <main className="max-w-4xl mx-auto">
                <PlatformPreview
                    platform="X (Twitter)"
                    width={1500}
                    height={500}
                    safeZone={{ w: '70%', h: '80%' }}
                />

                <PlatformPreview
                    platform="LinkedIn"
                    width={1584}
                    height={396}
                    safeZone={{ w: '80%', h: '60%' }}
                />

                <PlatformPreview
                    platform="YouTube"
                    width={2560}
                    height={1440}
                    safeZone={{ w: '60%', h: '30%' }} // Safe area for mobile/desktop overlap
                />

                <div className="mt-16 p-8 bg-white rounded-3xl border border-slate-200 shadow-sm">
                    <h4 className="text-xl font-black text-slate-900 mb-4 flex items-center gap-2">
                        <Download className="w-5 h-5 text-orange-600" /> Implementation Guide
                    </h4>
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-8 text-sm leading-relaxed text-slate-600">
                        <div>
                            <h5 className="font-bold text-slate-900 mb-2">Typography & Overlay</h5>
                            <p>When applying text to these banners, use **Satoshi Black** for the brand name and **Inter Bold** for the tagline. Add a subtle dark gradient (`linear-gradient(rgba(0,0,0,0.6), transparent)`) at the bottom to ensure readability if you place text near the base.</p>
                        </div>
                        <div>
                            <h5 className="font-bold text-slate-900 mb-2">Exporting Assets</h5>
                            <p>For the best results, export these as **PNG-24** files. For YouTube, ensure the core brand message is contained within the central 1546x423px "Text and Logo Safe Area" to prevent cropping on mobile devices.</p>
                        </div>
                    </div>
                </div>
            </main>
        </div>
    );
};

export default App;