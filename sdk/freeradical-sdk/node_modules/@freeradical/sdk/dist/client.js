"use strict";
// FreeRadical SDK - Main Client
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.FreeRadicalClient = void 0;
const axios_1 = __importDefault(require("axios"));
class FreeRadicalClient {
    constructor(config) {
        this.client = axios_1.default.create({
            baseURL: config.baseUrl,
            timeout: config.timeout || 10000,
            headers: {
                'Content-Type': 'application/json',
                ...(config.apiKey && { 'X-API-Key': config.apiKey }),
                ...(config.jwt && { 'Authorization': `Bearer ${config.jwt}` })
            }
        });
    }
    // Pages API
    async getPages(options) {
        const response = await this.client.get('/api/pages', { params: options });
        return response.data;
    }
    async getPage(uuid) {
        const response = await this.client.get(`/api/pages/${uuid}`);
        return response.data;
    }
    async createPage(input) {
        const response = await this.client.post('/api/pages', input);
        return response.data;
    }
    async updatePage(uuid, input) {
        const response = await this.client.put(`/api/pages/${uuid}`, input);
        return response.data;
    }
    async deletePage(uuid) {
        await this.client.delete(`/api/pages/${uuid}`);
    }
    // Modules API
    async getModules(pageUuid) {
        const params = pageUuid ? { page_uuid: pageUuid } : {};
        const response = await this.client.get('/api/modules', { params });
        return response.data;
    }
    async getModule(uuid) {
        const response = await this.client.get(`/api/modules/${uuid}`);
        return response.data;
    }
    async createModule(input) {
        const response = await this.client.post('/api/modules', input);
        return response.data;
    }
    async updateModule(uuid, input) {
        const response = await this.client.put(`/api/modules/${uuid}`, input);
        return response.data;
    }
    async deleteModule(uuid) {
        await this.client.delete(`/api/modules/${uuid}`);
    }
    // Media API
    async getMedia(options) {
        const response = await this.client.get('/api/media', { params: options });
        return response.data;
    }
    async uploadMedia(file, filename) {
        const formData = new FormData();
        formData.append('file', file, filename);
        const response = await this.client.post('/api/media/upload', formData, {
            headers: { 'Content-Type': 'multipart/form-data' }
        });
        return response.data;
    }
    async deleteMedia(uuid) {
        await this.client.delete(`/api/media/${uuid}`);
    }
    // Search API
    async search(query, resources) {
        const response = await this.client.get('/api/search', {
            params: { q: query, resources: resources?.join(',') }
        });
        return response.data;
    }
    // Webhooks API
    async getWebhooks() {
        const response = await this.client.get('/api/webhooks');
        return response.data;
    }
    async createWebhook(input) {
        const response = await this.client.post('/api/webhooks', input);
        return response.data;
    }
    async updateWebhook(id, input) {
        const response = await this.client.put(`/api/webhooks/${id}`, input);
        return response.data;
    }
    async deleteWebhook(id) {
        await this.client.delete(`/api/webhooks/${id}`);
    }
    async testWebhook(id) {
        const response = await this.client.post(`/api/webhooks/${id}/test`);
        return response.data;
    }
    // Relationships API
    async createRelationship(input) {
        const response = await this.client.post('/api/relationships', input);
        return response.data;
    }
    async getRelated(resourceType, resourceId) {
        const response = await this.client.get(`/api/relationships/${resourceType}/${resourceId}`);
        return response.data;
    }
    async deleteRelationship(id) {
        await this.client.delete(`/api/relationships/${id}`);
    }
    // Health & Metrics
    async getHealth() {
        const response = await this.client.get('/api/health');
        return response.data;
    }
    async getMetrics() {
        const response = await this.client.get('/api/metrics');
        return response.data;
    }
}
exports.FreeRadicalClient = FreeRadicalClient;
exports.default = FreeRadicalClient;
