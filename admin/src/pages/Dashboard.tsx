function Dashboard() {
    return (
        <div className="min-h-screen bg-gray-100">
            <nav className="bg-white shadow-sm">
                <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div className="flex justify-between h-16">
                        <div className="flex">
                            <div className="flex-shrink-0 flex items-center">
                                <h1 className="text-xl font-bold">FreeRadical CMS</h1>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>

            <div className="py-10">
                <header>
                    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                        <h1 className="text-3xl font-bold leading-tight text-gray-900">
                            Dashboard
                        </h1>
                    </div>
                </header>
                <main>
                    <div className="max-w-7xl mx-auto sm:px-6 lg:px-8">
                        <div className="px-4 py-8 sm:px-0">
                            <div className="border-4 border-dashed border-gray-200 rounded-lg h-96 p-8">
                                <h2 className="text-2xl font-semibold mb-4">Welcome to FreeRadical Admin</h2>
                                <p className="text-gray-600 mb-4">
                                    This is the admin dashboard foundation. Full features coming soon:
                                </p>
                                <ul className="list-disc list-inside space-y-2 text-gray-700">
                                    <li>Page Management</li>
                                    <li>Media Library</li>
                                    <li>WYSIWYG Editor</li>
                                    <li>SEO Preview</li>
                                    <li>Analytics</li>
                                </ul>
                            </div>
                        </div>
                    </div>
                </main>
            </div>
        </div>
    )
}

export default Dashboard
