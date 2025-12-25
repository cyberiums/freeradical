import { useState } from 'react'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import api from '../lib/api'
import { Input } from '../components/ui/Input'
import { Button } from '../components/ui/Button'
import { Card } from '../components/ui/Card'
import { Editor } from '../components/Editor'

export default function PageEditor() {
    const queryClient = useQueryClient()
    const [title, setTitle] = useState('')
    const [url, setUrl] = useState('')
    const [content, setContent] = useState('')
    const [metaTitle, setMetaTitle] = useState('')
    const [metaDescription, setMetaDescription] = useState('')

    const createMutation = useMutation({
        mutationFn: (data: any) => api.pages.create(data),
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['pages'] })
            // Reset form or navigate away
        }
    })

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault()
        createMutation.mutate({
            page_title: title,
            page_url: url,
            page_content: content,
            meta_title: metaTitle,
            meta_description: metaDescription,
            status: 'draft'
        })
    }

    return (
        <div className="max-w-4xl mx-auto p-6 space-y-6">
            <div className="flex justify-between items-center">
                <h1 className="text-3xl font-bold">Create New Page</h1>
                <Button onClick={handleSubmit} disabled={createMutation.isPending}>
                    {createMutation.isPending ? 'Saving...' : 'Save Page'}
                </Button>
            </div>

            <form onSubmit={handleSubmit} className="space-y-6">
                <Card title="Basic Information">
                    <div className="space-y-4">
                        <Input
                            label="Page Title"
                            value={title}
                            onChange={(e) => setTitle(e.target.value)}
                            required
                        />
                        <Input
                            label="URL Slug"
                            value={url}
                            onChange={(e) => setUrl(e.target.value)}
                            placeholder="about-us"
                            required
                        />
                    </div>
                </Card>

                <Card title="Content">
                    <Editor content={content} onChange={setContent} />
                </Card>

                <Card title="SEO Settings">
                    <div className="space-y-4">
                        <Input
                            label="Meta Title"
                            value={metaTitle}
                            onChange={(e) => setMetaTitle(e.target.value)}
                            placeholder="Leave blank to use page title"
                        />
                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-1">
                                Meta Description
                            </label>
                            <textarea
                                value={metaDescription}
                                onChange={(e) => setMetaDescription(e.target.value)}
                                className="w-full px-3 py-2 border border-gray-300 rounded-md"
                                rows={3}
                                placeholder="Brief description for search engines"
                            />
                        </div>
                    </div>
                </Card>
            </form>
        </div>
    )
}
