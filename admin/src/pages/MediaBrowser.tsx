import { useState } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import api from '../lib/api'
import { Button } from '../components/ui/Button'
import { Card } from '../components/ui/Card'

interface MediaFile {
    uuid: string
    filename: string
    file_path: string
    file_size: number
    width: number
    height: number
    alt_text: string
    created_at: string
}

export default function MediaBrowser() {
    const queryClient = useQueryClient()
    const [selectedFile, setSelectedFile] = useState<File | null>(null)
    const [altText, setAltText] = useState('')

    const { data, isLoading } = useQuery({
        queryKey: ['media'],
        queryFn: () => api.media.list()
    })

    const uploadMutation = useMutation({
        mutationFn: (formData: FormData) => api.media.upload(formData),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['media'] })
            setSelectedFile(null)
            setAltText('')
        }
    })

    const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (e.target.files && e.target.files[0]) {
            setSelectedFile(e.target.files[0])
        }
    }

    const handleUpload = () => {
        if (!selectedFile) return

        const formData = new FormData()
        formData.append('file', selectedFile)
        formData.append('alt_text', altText)

        uploadMutation.mutate(formData)
    }

    const media: MediaFile[] = data?.data || []

    if (isLoading) return <div>Loading...</div>

    return (
        <div className="p-6 space-y-6">
            <div className="flex justify-between items-center">
                <h1 className="text-2xl font-bold">Media Library</h1>
            </div>

            <Card title="Upload New Media">
                <div className="space-y-4">
                    <div>
                        <input
                            type="file"
                            accept="image/*"
                            onChange={handleFileSelect}
                            className="block w-full text-sm"
                        />
                    </div>
                    {selectedFile && (
                        <>
                            <input
                                type="text"
                                placeholder="Alt text for accessibility"
                                value={altText}
                                onChange={(e) => setAltText(e.target.value)}
                                className="w-full px-3 py-2 border rounded-md"
                            />
                            <Button onClick={handleUpload} disabled={uploadMutation.isPending}>
                                {uploadMutation.isPending ? 'Uploading...' : 'Upload'}
                            </Button>
                        </>
                    )}
                </div>
            </Card>

            <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
                {media.map((item: MediaFile) => (
                    <div key={item.uuid} className="border rounded-lg overflow-hidden hover:shadow-lg transition">
                        <img
                            src={item.file_path}
                            alt={item.alt_text || item.filename}
                            className="w-full h-48 object-cover"
                        />
                        <div className="p-3">
                            <p className="text-sm font-medium truncate">{item.filename}</p>
                            <p className="text-xs text-gray-500">
                                {item.width} × {item.height}
                            </p>
                            <p className="text-xs text-gray-500">
                                {(item.file_size / 1024).toFixed(0)} KB
                            </p>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    )
}
