import { useQuery } from '@tanstack/react-query'
import api from '../lib/api'
import { Line } from 'react-chartjs-2'
import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend
} from 'chart.js'

ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend
)

interface AnalyticsData {
    total_views: number
    unique_visitors: number
    today_views: number
    top_pages: Array<{
        page_url: string
        views: number
    }>
    referrers: Array<{
        source: string
        count: number
    }>
}

export default function Analytics() {
    const { data, isLoading } = useQuery({
        queryKey: ['analytics'],
        queryFn: () => api.get('/analytics/summary'),
        refetchInterval: 30000 // Refresh every 30s
    })

    const analytics: AnalyticsData = data?.data || {
        total_views: 0,
        unique_visitors: 0,
        today_views: 0,
        top_pages: [],
        referrers: []
    }

    if (isLoading) return <div>Loading analytics...</div>

    return (
        <div className="p-6 space-y-6">
            <h1 className="text-3xl font-bold">Analytics Dashboard</h1>

            {/* Stats Cards */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                <StatsCard
                    title="Total Views"
                    value={analytics.total_views.toLocaleString()}
                    icon="📊"
                />
                <StatsCard
                    title="Unique Visitors"
                    value={analytics.unique_visitors.toLocaleString()}
                    icon="👥"
                />
                <StatsCard
                    title="Today's Views"
                    value={analytics.today_views.toLocaleString()}
                    icon="📈"
                />
            </div>

            {/* Top Pages */}
            <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
                <h2 className="text-xl font-semibold mb-4">Top Pages</h2>
                <div className="space-y-3">
                    {analytics.top_pages.map((page, idx) => (
                        <div key={idx} className="flex justify-between items-center">
                            <span className="text-sm">{page.page_url}</span>
                            <span className="font-semibold">{page.views} views</span>
                        </div>
                    ))}
                </div>
            </div>

            {/* Referrers */}
            <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
                <h2 className="text-xl font-semibold mb-4">Top Referrers</h2>
                <div className="space-y-3">
                    {analytics.referrers.map((ref, idx) => (
                        <div key={idx} className="flex justify-between items-center">
                            <span className="text-sm">{ref.source || 'Direct'}</span>
                            <span className="font-semibold">{ref.count}</span>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    )
}

function StatsCard({ title, value, icon }: { title: string, value: string, icon: string }) {
    return (
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
            <div className="flex items-center justify-between">
                <div>
                    <p className="text-sm text-gray-600 dark:text-gray-400">{title}</p>
                    <p className="text-3xl font-bold mt-2">{value}</p>
                </div>
                <div className="text-4xl">{icon}</div>
            </div>
        </div>
    )
}
