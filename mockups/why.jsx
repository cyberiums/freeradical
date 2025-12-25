import React from 'react';
import {
    Zap,
    Globe,
    Server,
    Cpu,
    DollarSign,
    ArrowDown,
    TrendingUp,
    Waves,
    Layers,
    ShieldCheck,
    AlertTriangle
} from 'lucide-react';

const InfographicSection = ({ icon: Icon, title, subtitle, children, colorClass = "text-orange-500", bgClass = "bg-slate-900" }) => (
    <div className={`p-8 rounded-[2rem] ${bgClass} border border-slate-800 relative overflow-hidden group`}>
        <div className="relative z-10">
            <div className={`w-14 h-14 ${colorClass.replace('text', 'bg')}/10 rounded-2xl flex items-center justify-center mb-6 border border-white/10`}>
                <Icon className={`w-8 h-8 ${colorClass}`} />
            </div>
            <h3 className="text-2xl font-black text-white mb-2 uppercase tracking-tight">{title}</h3>
            <p className="text-slate-400 font-bold text-sm mb-6 uppercase tracking-widest">{subtitle}</p>
            {children}
        </div>
    </div>
);

const App = () => {
    return (
        <div className="min-h-screen bg-[#0F172A] p-6 lg:p-12 font-sans text-white">
            <div className="max-w-6xl mx-auto">
                {/* Header */}
                <header className="text-center mb-20">
                    <div className="inline-flex items-center gap-2 bg-orange-500/10 border border-orange-500/20 px-4 py-2 rounded-full text-[10px] font-black uppercase tracking-[0.3em] text-orange-500 mb-6">
                        Technical Insight Report
                    </div>
                    <h1 className="text-5xl lg:text-7xl font-black tracking-tighter mb-6">
                        The <span className="text-orange-500">Billion-Dollar</span> Second
                    </h1>
                    <p className="text-xl text-slate-400 max-w-2xl mx-auto font-medium">
                        How FreeRadical's Rust-native architecture recovers the performance lost to legacy "Framework Taxes."
                    </p>
                </header>

                {/* Phase 1: Physical Reality */}
                <div className="grid grid-cols-1 lg:grid-cols-3 gap-8 mb-8">
                    <InfographicSection
                        icon={Waves}
                        title="Undersea Infrastructure"
                        subtitle="The $600M+ Mile"
                    >
                        <p className="text-slate-500 text-sm leading-relaxed mb-4">
                            Private fiber cables (Curie, Dunant) cost billions to reduce trans-Atlantic latency by just ~20ms.
                        </p>
                        <div className="h-2 w-full bg-slate-800 rounded-full overflow-hidden">
                            <div className="bg-orange-500 h-full w-full" />
                        </div>
                    </InfographicSection>

                    <InfographicSection
                        icon={Server}
                        title="Edge Collation"
                        subtitle="Massive Global CAPEX"
                    >
                        <p className="text-slate-500 text-sm leading-relaxed mb-4">
                            Tens of thousands of PoP nodes leased to bring content "closer" to the user's ISP.
                        </p>
                        <div className="flex gap-1">
                            {[...Array(8)].map((_, i) => (
                                <div key={i} className="h-6 w-full bg-orange-500/40 rounded-sm" />
                            ))}
                        </div>
                    </InfographicSection>

                    <InfographicSection
                        icon={DollarSign}
                        title="Infrastructure ROI"
                        subtitle="The Cost of 1 Second"
                    >
                        <div className="text-4xl font-black text-white mb-2">$Billions</div>
                        <p className="text-slate-500 text-xs font-bold uppercase tracking-widest">
                            Spent to fight the laws of physics.
                        </p>
                    </InfographicSection>
                </div>

                {/* The Bottleneck (Visual Bridge) */}
                <div className="relative py-16 flex flex-col items-center">
                    <div className="w-px h-20 bg-gradient-to-b from-orange-500 to-red-600" />
                    <div className="bg-red-600/10 border-2 border-red-600/50 p-6 rounded-3xl flex items-center gap-6 max-w-2xl w-full my-8">
                        <div className="bg-red-600 p-4 rounded-2xl shadow-lg shadow-red-900/40">
                            <AlertTriangle className="w-8 h-8 text-white" />
                        </div>
                        <div>
                            <h4 className="text-xl font-black text-white uppercase italic">The PHP Framework Tax</h4>
                            <p className="text-red-200/60 text-sm font-medium">
                                Legacy interpreted code kills 90% of the speed gained by physical hardware.
                            </p>
                        </div>
                    </div>
                    <div className="w-px h-20 bg-gradient-to-b from-red-600 to-cyan-500" />
                </div>

                {/* Phase 2: The Rust Solution */}
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 items-stretch">
                    <div className="bg-slate-900 border-2 border-cyan-500/30 rounded-[3rem] p-10 flex flex-col">
                        <div className="flex justify-between items-start mb-10">
                            <div className="bg-cyan-500 p-4 rounded-2xl shadow-xl shadow-cyan-900/20">
                                <Cpu className="w-10 h-10 text-slate-950" />
                            </div>
                            <div className="text-right">
                                <span className="text-[10px] font-black uppercase tracking-widest text-cyan-500 block mb-1">FreeRadical Engine</span>
                                <span className="text-2xl font-black text-white italic">Rust-Native</span>
                            </div>
                        </div>
                        <div className="space-y-6 flex-grow">
                            <h4 className="text-3xl font-black leading-none">Zero-Cost Abstractions. <br /><span className="text-cyan-500 underline decoration-cyan-900 decoration-4">Zero Framework Tax.</span></h4>
                            <p className="text-slate-400 font-medium">
                                Unlike PHP, which must be interpreted at runtime, FreeRadical is compiled to raw machine code. We execute at the speed of the fiber optics, not the speed of an interpreter.
                            </p>
                            <div className="grid grid-cols-2 gap-4">
                                <div className="bg-slate-800/50 p-4 rounded-2xl border border-slate-700">
                                    <div className="text-cyan-500 font-black text-xl">300%</div>
                                    <div className="text-[10px] uppercase font-black text-slate-500 tracking-tighter">Minimum Jump</div>
                                </div>
                                <div className="bg-slate-800/50 p-4 rounded-2xl border border-slate-700">
                                    <div className="text-cyan-500 font-black text-xl">1500%</div>
                                    <div className="text-[10px] uppercase font-black text-slate-500 tracking-tighter">Peak Improvement</div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div className="bg-slate-950 rounded-[3rem] p-10 flex flex-col justify-center border border-slate-800">
                        <div className="text-center mb-10">
                            <h4 className="text-xs font-black uppercase tracking-[0.4em] text-slate-500 mb-4">Performance Benchmark</h4>
                            <div className="flex items-center justify-center gap-12">
                                <div className="flex flex-col items-center">
                                    <div className="w-12 h-40 bg-slate-800 rounded-t-xl relative overflow-hidden flex flex-col justify-end">
                                        <div className="h-full bg-slate-700 w-full" />
                                    </div>
                                    <span className="text-[10px] mt-4 font-black uppercase text-slate-500 tracking-widest">Legacy PHP</span>
                                </div>
                                <div className="flex flex-col items-center">
                                    <div className="w-12 h-40 bg-slate-800 rounded-t-xl relative overflow-hidden flex flex-col justify-end">
                                        <div className="h-[8%] bg-orange-500 w-full animate-pulse" />
                                    </div>
                                    <span className="text-[10px] mt-4 font-black uppercase text-orange-500 tracking-widest">FreeRadical</span>
                                </div>
                            </div>
                        </div>
                        <div className="space-y-4">
                            <div className="flex items-center gap-4 bg-slate-900 p-4 rounded-2xl">
                                <ShieldCheck className="w-6 h-6 text-cyan-500" />
                                <span className="text-sm font-bold text-slate-300">Memory Safe (Rust) = No Bloat</span>
                            </div>
                            <div className="flex items-center gap-4 bg-slate-900 p-4 rounded-2xl">
                                <Zap className="w-6 h-6 text-cyan-500" />
                                <span className="text-sm font-bold text-slate-300">Sub-100ms Core Web Vitals</span>
                            </div>
                            <div className="flex items-center gap-4 bg-slate-900 p-4 rounded-2xl">
                                <TrendingUp className="w-6 h-6 text-cyan-500" />
                                <span className="text-sm font-bold text-slate-300">Exponential SEO Ranking Lift</span>
                            </div>
                        </div>
                    </div>
                </div>

                {/* Footer */}
                <footer className="mt-20 text-center border-t border-slate-800 pt-12">
                    <div className="flex items-center justify-center gap-3 text-2xl font-black italic mb-4 opacity-40">
                        <div className="w-8 h-8 bg-slate-100 rounded-lg flex items-center justify-center text-slate-900">
                            <Zap className="w-5 h-5" />
                        </div>
                        <span>FreeRadical</span>
                    </div>
                    <p className="text-[10px] font-black text-slate-600 uppercase tracking-[0.5em]">The Rust Advantage. Engineered for the Instant Web.</p>
                </footer>
            </div>
        </div>
    );
};

export default App;