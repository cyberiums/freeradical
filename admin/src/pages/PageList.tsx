import { useState } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import api from '../lib/api'
import { Button } from '../components/ui/Button'

interface Page {
    uuid: string
    page_title: string
    page_url: string
    status: string
    created_at: string
}

export default function PageList() {
    const queryClient = useQueryClient()
    const [page] = useState(1)

    const { data, isLoading } = useQuery({
        queryKey: ['pages', page],
        queryFn: () => api.pages.list({ page, perPage: 20 })
    })

    const deleteMutation = useMutation({
        mutationFn: (uuid: string) => api.pages.delete(uuid),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['pages'] })
        }
    })

    if (isLoading) return <div>Loading...</div>

    const pages: Page[] = data?.data?.pages || []

    return (
        <div className="space-y-4">
            <div className="flex justify-between items-center">
                <h1 className="text-2xl font-bold">Pages</h1>
                <Button>Create New Page</Button>
            </div>

            <div className="bg-white shadow rounded-lg">
                <table className="min-w-full divide-y divide-gray-200">
                    <thead className="bg-gray-50">
                        <tr>
                            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                Title
                            </th>
                            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                URL
                            </th>
                            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                Status
                            </th>
                            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                Actions
                            </th>
                        </tr>
                    </thead>
                    <tbody className="bg-white divide-y divide-gray-200">
                        {pages.map((page: Page) => (
                            <tr key={page.uuid}>
                                <td className="px-6 py-4 whitespace-nowrap">{page.page_title}</td>
                                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                    {page.page_url}
                                </td>
                                <td className="px-6 py-4 whitespace-nowrap">
                                    <span className="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800">
                                        {page.status}
                                    </span>
                                </td>
                                <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">
                                    <Button variant="ghost" size="sm" className="mr-2">
                                        Edit
                                    </Button>
                                    <Button
                                        variant="destructive"
                                        size="sm"
                                        onClick={() => deleteMutation.mutate(page.uuid)}
                                    >
                                        Delete
                                    </Button>
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    )
}
