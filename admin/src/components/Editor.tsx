import { useEditor, EditorContent } from '@tiptap/react'
import StarterKit from '@tiptap/starter-kit'
import { Button } from './ui/Button'

interface EditorProps {
    content: string
    onChange: (content: string) => void
}

export function Editor({ content, onChange }: EditorProps) {
    const editor = useEditor({
        extensions: [StarterKit],
        content,
        onUpdate: ({ editor }) => {
            onChange(editor.getHTML())
        },
    })

    if (!editor) {
        return <div>Loading editor...</div>
    }

    return (
        <div className="border rounded-lg overflow-hidden">
            <div className="bg-gray-50 border-b p-2 flex gap-2 flex-wrap">
                <Button
                    size="sm"
                    variant={editor.isActive('bold') ? 'default' : 'outline'}
                    onClick={() => editor.chain().focus().toggleBold().run()}
                >
                    Bold
                </Button>
                <Button
                    size="sm"
                    variant={editor.isActive('italic') ? 'default' : 'outline'}
                    onClick={() => editor.chain().focus().toggleItalic().run()}
                >
                    Italic
                </Button>
                <Button
                    size="sm"
                    variant={editor.isActive('heading', { level: 1 }) ? 'default' : 'outline'}
                    onClick={() => editor.chain().focus().toggleHeading({ level: 1 }).run()}
                >
                    H1
                </Button>
                <Button
                    size="sm"
                    variant={editor.isActive('heading', { level: 2 }) ? 'default' : 'outline'}
                    onClick={() => editor.chain().focus().toggleHeading({ level: 2 }).run()}
                >
                    H2
                </Button>
                <Button
                    size="sm"
                    variant={editor.isActive('bulletList') ? 'default' : 'outline'}
                    onClick={() => editor.chain().focus().toggleBulletList().run()}
                >
                    Bullet List
                </Button>
                <Button
                    size="sm"
                    variant={editor.isActive('orderedList') ? 'default' : 'outline'}
                    onClick={() => editor.chain().focus().toggleOrderedList().run()}
                >
                    Numbered List
                </Button>
            </div>
            <EditorContent
                editor={editor}
                className="prose max-w-none p-4 min-h-[300px]"
            />
        </div>
    )
}
