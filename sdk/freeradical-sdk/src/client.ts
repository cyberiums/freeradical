// FreeRadical SDK - Main Client

import axios, { AxiosInstance, AxiosError } from 'axios';
import {
  FreeRadicalConfig,
  Page,
  CreatePageInput,
  UpdatePageInput,
  Module,
  Media,
  SearchResponse,
  Webhook,
  Relationship,
  PaginationOptions,
  ApiResponse,
  ApiError
} from './types';

export class FreeRadicalClient {
  private client: AxiosInstance;

  constructor(config: FreeRadicalConfig) {
    this.client = axios.create({
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
  async getPages(options?: PaginationOptions): Promise<Page[]> {
    const response = await this.client.get('/api/pages', { params: options });
    return response.data;
  }

  async getPage(uuid: string): Promise<Page> {
    const response = await this.client.get(`/api/pages/${uuid}`);
    return response.data;
  }

  async createPage(input: CreatePageInput): Promise<Page> {
    const response = await this.client.post('/api/pages', input);
    return response.data;
  }

  async updatePage(uuid: string, input: UpdatePageInput): Promise<Page> {
    const response = await this.client.put(`/api/pages/${uuid}`, input);
    return response.data;
  }

  async deletePage(uuid: string): Promise<void> {
    await this.client.delete(`/api/pages/${uuid}`);
  }

  // Modules API
  async getModules(pageUuid?: string): Promise<Module[]> {
    const params = pageUuid ? { page_uuid: pageUuid } : {};
    const response = await this.client.get('/api/modules', { params });
    return response.data;
  }

  async getModule(uuid: string): Promise<Module> {
    const response = await this.client.get(`/api/modules/${uuid}`);
    return response.data;
  }

  async createModule(input: Partial<Module>): Promise<Module> {
    const response = await this.client.post('/api/modules', input);
    return response.data;
  }

  async updateModule(uuid: string, input: Partial<Module>): Promise<Module> {
    const response = await this.client.put(`/api/modules/${uuid}`, input);
    return response.data;
  }

  async deleteModule(uuid: string): Promise<void> {
    await this.client.delete(`/api/modules/${uuid}`);
  }

  // Media API
  async getMedia(options?: PaginationOptions): Promise<Media[]> {
    const response = await this.client.get('/api/media', { params: options });
    return response.data;
  }

  async uploadMedia(file: any, filename?: string): Promise<Media> {
    const formData = new FormData();
    formData.append('file', file, filename);
    const response = await this.client.post('/api/media/upload', formData, {
      headers: { 'Content-Type': 'multipart/form-data' }
    });
    return response.data;
  }

  async deleteMedia(uuid: string): Promise<void> {
    await this.client.delete(`/api/media/${uuid}`);
  }

  // Search API
  async search(query: string, resources?: string[]): Promise<SearchResponse> {
    const response = await this.client.get('/api/search', {
      params: { q: query, resources: resources?.join(',') }
    });
    return response.data;
  }

  // Webhooks API
  async getWebhooks(): Promise<Webhook[]> {
    const response = await this.client.get('/api/webhooks');
    return response.data;
  }

  async createWebhook(input: Webhook): Promise<Webhook> {
    const response = await this.client.post('/api/webhooks', input);
    return response.data;
  }

  async updateWebhook(id: number, input: Partial<Webhook>): Promise<Webhook> {
    const response = await this.client.put(`/api/webhooks/${id}`, input);
    return response.data;
  }

  async deleteWebhook(id: number): Promise<void> {
    await this.client.delete(`/api/webhooks/${id}`);
  }

  async testWebhook(id: number): Promise<any> {
    const response = await this.client.post(`/api/webhooks/${id}/test`);
    return response.data;
  }

  // Relationships API
  async createRelationship(input: Relationship): Promise<Relationship> {
    const response = await this.client.post('/api/relationships', input);
    return response.data;
  }

  async getRelated(resourceType: string, resourceId: string): Promise<any[]> {
    const response = await this.client.get(`/api/relationships/${resourceType}/${resourceId}`);
    return response.data;
  }

  async deleteRelationship(id: number): Promise<void> {
    await this.client.delete(`/api/relationships/${id}`);
  }

  // Health & Metrics
  async getHealth(): Promise<any> {
    const response = await this.client.get('/api/health');
    return response.data;
  }

  async getMetrics(): Promise<any> {
    const response = await this.client.get('/api/metrics');
    return response.data;
  }
}

export default FreeRadicalClient;
