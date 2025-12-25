# @freeradical/sdk

Official TypeScript/JavaScript SDK for FreeRadical CMS

## Installation

```bash
npm install @freeradical/sdk
# or
yarn add @freeradical/sdk
```

## Quick Start

```typescript
import FreeRadicalClient from '@freeradical/sdk';

const client = new FreeRadicalClient({
  baseUrl: 'https://your-cms.com',
  jwt: 'your-jwt-token' // or apiKey
});

// Get all pages
const pages = await client.getPages();

// Create a page
const newPage = await client.createPage({
  page_title: 'My Page',
  page_url: '/my-page',
  content: '<h1>Hello World</h1>'
});

// Search
const results = await client.search('query', ['pages', 'modules']);
```

## API Reference

### Pages

```typescript
// List pages
client.getPages(options?: { page?: number, per_page?: number })

// Get single page
client.getPage(uuid: string)

// Create page
client.createPage(input: CreatePageInput)

// Update page
client.updatePage(uuid: string, input: UpdatePageInput)

// Delete page
client.deletePage(uuid: string)
```

### Modules

```typescript
// List modules
client.getModules(pageUuid?: string)

// Get module
client.getModule(uuid: string)

// Create module
client.createModule(input: Partial<Module>)

// Update module
client.updateModule(uuid: string, input: Partial<Module>)

// Delete module
client.deleteModule(uuid: string)
```

### Media

```typescript
// List media
client.getMedia(options?: PaginationOptions)

// Upload media
client.uploadMedia(file: File | Buffer, filename?: string)

// Delete media
client.deleteMedia(uuid: string)
```

### Search

```typescript
// Search across resources
client.search(query: string, resources?: string[])
```

### Webhooks

```typescript
// List webhooks
client.getWebhooks()

// Create webhook
client.createWebhook(input: Webhook)

// Update webhook
client.updateWebhook(id: number, input: Partial<Webhook>)

// Delete webhook
client.deleteWebhook(id: number)

// Test webhook
client.testWebhook(id: number)
```

### Relationships

```typescript
// Create relationship
client.createRelationship(input: Relationship)

// Get related content
client.getRelated(resourceType: string, resourceId: string)

// Delete relationship
client.deleteRelationship(id: number)
```

### Monitoring

```typescript
// Health check
client.getHealth()

// Metrics
client.getMetrics()
```

## TypeScript Support

Full TypeScript support with complete type definitions:

```typescript
import FreeRadicalClient, { Page, CreatePageInput } from '@freeradical/sdk';

const client = new FreeRadicalClient({ baseUrl: '...' });

const page: Page = await client.getPage('uuid');
const input: CreatePageInput = {
  page_title: 'Title',
  page_url: '/url'
};
```

## Error Handling

```typescript
try {
  const page = await client.getPage('invalid-uuid');
} catch (error) {
  console.error('Error:', error.message);
}
```

## License

MIT
