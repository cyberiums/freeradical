import React from 'react'

interface CardProps {
    children: React.ReactNode
    className?: string
    title?: string
}

export function Card({ children, className = '', title }: CardProps) {
    return (
        <div className={`bg-white shadow rounded-lg ${className}`}>
            {title && (
                <div className="px-6 py-4 border-b border-gray-200">
                    <h3 className="text-lg font-medium text-gray-900">{title}</h3>
                </div>
            )}
            <div className="p-6">
                {children}
            </div>
        </div>
    )
}
