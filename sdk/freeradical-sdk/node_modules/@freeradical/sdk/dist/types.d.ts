export interface Page {
    uuid: string;
    page_title: string;
    page_url: string;
    page_name?: string;
    content?: string;
    meta_title?: string;
    meta_description?: string;
    meta_keywords?: string;
    og_title?: string;
    og_description?: string;
    og_image?: string;
    status?: string;
    created_at?: string;
    updated_at?: string;
}
export interface CreatePageInput {
    page_title: string;
    page_url: string;
    page_name?: string;
    content?: string;
    meta_title?: string;
    meta_description?: string;
}
export interface UpdatePageInput {
    page_title?: string;
    page_url?: string;
    content?: string;
    meta_title?: string;
    meta_description?: string;
    status?: string;
}
export interface Module {
    uuid: string;
    page_uuid: string;
    title: string;
    content: string;
    field_type?: string;
    field_config?: any;
    validation_rules?: any;
}
export interface Media {
    uuid: string;
    filename: string;
    original_filename: string;
    mime_type: string;
    file_size: number;
    width?: number;
    height?: number;
    storage_path: string;
    cdn_url?: string;
    uploaded_by?: string;
    created_at?: string;
}
export interface SearchResult {
    resource_type: string;
    id: string;
    title: string;
    snippet: string;
}
export interface SearchResponse {
    results: SearchResult[];
    total: number;
    page: number;
    per_page: number;
}
export interface Webhook {
    id?: number;
    url: string;
    events: string[];
    secret?: string;
    active?: boolean;
}
export interface Relationship {
    id?: number;
    source_type: string;
    source_id: string;
    target_type: string;
    target_id: string;
    relationship_type: string;
    metadata?: any;
}
export interface PaginationOptions {
    page?: number;
    per_page?: number;
}
export interface FreeRadicalConfig {
    baseUrl: string;
    apiKey?: string;
    jwt?: string;
    timeout?: number;
}
export interface ApiResponse<T> {
    data: T;
    status: number;
}
export interface ApiError {
    message: string;
    status: number;
    details?: any;
}
