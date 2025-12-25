# FreeRadical CMS - SDK Developer Guide

## 📚 Table of Contents

1. [Introduction](#introduction)
2. [Available SDKs](#available-sdks)
3. [Building a New SDK](#building-a-new-sdk)
4. [SDK Architecture](#sdk-architecture)
5. [API Reference](#api-reference)
6. [Authentication](#authentication)
7. [Error Handling](#error-handling)
8. [Testing](#testing)
9. [Publishing](#publishing)
10. [Contributing](#contributing)

---

## Introduction

This guide is for developers who want to:
- **Use** existing FreeRadical CMS client SDKs
- **Build** new SDKs for other languages
- **Contribute** to existing SDK maintenance

FreeRadical CMS provides official SDKs in **TypeScript**, **Python**, and **Go**, with a clear pattern for adding more languages.

---

## Available SDKs

### 1. TypeScript/JavaScript SDK

**Package**: `@freeradical/sdk`  
**Status**: ✅ Production Ready  
**Location**: [`sdk/freeradical-sdk/`](file:///Users/prabhatsingh/freeradical/sdk/freeradical-sdk/)

```bash
npm install @freeradical/sdk
```

**Quick Start**:
```typescript
import FreeRadicalClient from '@freeradical/sdk';

const client = new FreeRadicalClient({
  baseUrl: 'https://your-cms.com',
  jwt: 'your-jwt-token'
});

const pages = await client.getPages();
```

See: [TypeScript SDK README](file:///Users/prabhatsingh/freeradical/sdk/freeradical-sdk/README.md)

---

### 2. Python SDK

**Package**: `freeradical-client`  
**Status**: ✅ Production Ready  
**Location**: [`sdks/python/`](file:///Users/prabhatsingh/freeradical/sdks/python/)

```bash
pip install freeradical-client
```

**Quick Start**:
```python
import freeradical_client

config = freeradical_client.Configuration(
    host="https://your-cms.com/v1",
    access_token="your-jwt"
)

with freeradical_client.ApiClient(config) as api_client:
    api = freeradical_client.DefaultApi(api_client)
    pages = api.pages_get()
```

See: [Python SDK README](file:///Users/prabhatsingh/freeradical/sdks/python/README.md)

---

### 3. Go SDK

**Package**: `freeradical-go-client`  
**Status**: ✅ Production Ready  
**Location**: [`sdks/go/`](file:///Users/prabhatsingh/freeradical/sdks/go/)

```bash
go get github.com/your-org/freeradical-go-client
```

**Quick Start**:
```go
import freeradical "github.com/your-org/freeradical-go-client"

cfg := freeradical.NewConfiguration()
cfg.Host = "your-cms.com"

client := freeradical.NewAPIClient(cfg)
pages, _, err := client.DefaultApi.PagesGet(context.Background())
```

See: [Go SDK README](file:///Users/prabhatsingh/freeradical/sdks/go/README.md)

---

## Building a New SDK

### Step 1: Choose Your Approach

**Option A: OpenAPI Generator** (Recommended for most languages)

1. Generate OpenAPI spec from Rust API
2. Use `openapi-generator` to scaffold SDK
3. Customize and test

**Option B: Manual Implementation** (For optimal DX)

1. Hand-craft client library
2. Mirror TypeScript SDK architecture
3. Provide idiomatic language patterns

---

### Step 2: Using OpenAPI Generator

#### Prerequisites
```bash
npm install -g @openapitools/openapi-generator-cli
```

#### Generate OpenAPI Spec
```bash
# From FreeRadical root
cargo build
cargo run --bin generate-openapi > api-spec.yml
```

#### Generate SDK

**Ruby Example**:
```bash
openapi-generator generate \
  -i api-spec.yml \
  -g ruby \
  -o sdks/ruby \
  --additional-properties=gemName=freeradical_client
```

**PHP Example**:
```bash
openapi-generator generate \
  -i api-spec.yml \
  -g php \
  -o sdks/php \
  --additional-properties=packageName=FreeRadical\\Client
```

**Supported Languages**: Java, C#, Rust, Kotlin, Swift, Dart, and [50+ more](https://openapi-generator.tech/docs/generators)

---

## SDK Architecture

### Core Components

Every SDK should implement:

#### 1. **Client Class**
Main entry point for all API calls

```typescript
class FreeRadicalClient {
  constructor(config: ClientConfig);
  
  // Resource methods
  getPages(options?): Promise<Page[]>;
  createPage(input): Promise<Page>;
  // ... etc
}
```

#### 2. **Configuration**
Authentication and endpoint settings

```typescript
interface ClientConfig {
  baseUrl: string;
  jwt?: string;        // JWT token
  apiKey?: string;     // Alternative auth
  timeout?: number;
}
```

#### 3. **Type Definitions**
Strongly typed models for requests/responses

```typescript
interface Page {
  uuid: string;
  page_title: string;
  page_url: string;
  content: string;
  // ... etc
}
```

#### 4. **Error Handling**
Consistent error types

```typescript
class FreeRadicalError extends Error {
  statusCode: number;
  response?: any;
}
```

---

## API Reference

### Authentication Endpoints

#### Login
```http
POST /user/login
Content-Type: application/json

{
  "username": "user",
  "password": "pass",
  "two_factor_code": "123456" // Optional
}

Response: JWT token in cookie
```

#### 2FA Setup
```http
GET /user/{username}/2fa/setup
Authorization: Bearer <jwt>

Response: {
  "secret": "...",
  "qr": "data:image/png;base64,..."
}
```

---

### Resource Endpoints

#### Pages

**List Pages**
```http
GET /page?page=1&per_page=10
Authorization: Bearer <jwt>
```

**Create Page**
```http
POST /page
Content-Type: application/json
Authorization: Bearer <jwt>

{
  "page_title": "My Page",
  "page_url": "/my-page",
  "content": "<h1>Content</h1>"
}
```

**Update Page**
```http
PUT /page/{uuid}
Content-Type: application/json
Authorization: Bearer <jwt>

{ "page_title": "Updated Title" }
```

**Delete Page**
```http
DELETE /page/{uuid}
Authorization: Bearer <jwt>
```

---

#### Modules

**List Modules**
```http
GET /module?page_uuid={uuid}
Authorization: Bearer <jwt>
```

**Create Module**
```http
POST /module
Content-Type: application/json
Authorization: Bearer <jwt>

{
  "page_uuid": "...",
  "module_title": "Hero Section",
  "module_type": "hero",
  "module_data": "{...}"
}
```

---

#### Media

**Upload Media**
```http
POST /media
Content-Type: multipart/form-data
Authorization: Bearer <jwt>

file: <binary>
```

**List Media**
```http
GET /media?page=1&per_page=20
Authorization: Bearer <jwt>
```

**Delete Media**
```http
DELETE /media/{uuid}
Authorization: Bearer <jwt>
```

---

#### Search

**Search Content**
```http
GET /search?q={query}&resources=pages,modules
Authorization: Bearer <jwt>
```

---

#### Webhooks

**List Webhooks**
```http
GET /webhooks
Authorization: Bearer <jwt>
```

**Create Webhook**
```http
POST /webhooks
Content-Type: application/json
Authorization: Bearer <jwt>

{
  "url": "https://example.com/webhook",
  "events": ["page.created", "page.updated"],
  "secret": "webhook-secret"
}
```

---

### Payment Endpoints (Phase 8)

**Create Payment**
```http
POST /payments/create
Content-Type: application/json
Authorization: Bearer <jwt>

{
  "provider": "stripe",
  "amount_cents": 5000,
  "currency": "USD",
  "metadata": {}
}
```

**List Providers**
```http
GET /payments/providers

Response: {
  "providers": ["stripe", "paypal", "square"]
}
```

---

## Authentication

### JWT Tokens

FreeRadical uses **JWT (JSON Web Tokens)** for API authentication.

**Obtaining a Token**:
```bash
curl -X POST https://cms.com/user/login \
  -H "Content-Type: application/json" \
  -d '{"username":"user","password":"pass"}'
  
# Response includes Set-Cookie with JWT
```

**Using in SDK**:
```typescript
const client = new FreeRadicalClient({
  baseUrl: 'https://cms.com',
  jwt: 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...'
});
```

**Token Refresh**:
- Tokens expire after configured duration
- Re-authenticate to get new token
- Store token securely (never in version control)

---

## Error Handling

### Standard Error Responses

**400 Bad Request**:
```json
{
  "code": 400,
  "error": "Bad Request",
  "message": "Invalid input data"
}
```

**401 Unauthorized**:
```json
{
  "code": 401,
  "error": "Unauthorized",
  "message": "Invalid or missing JWT token"
}
```

**404 Not Found**:
```json
{
  "code": 404,
  "error": "Not Found",
  "message": "Resource not found"
}
```

**500 Internal Server Error**:
```json
{
  "code": 500,
  "error": "Internal Server Error",
  "message": "An unexpected error occurred"
}
```

### SDK Error Handling Pattern

```typescript
try {
  const page = await client.getPage('invalid-uuid');
} catch (error) {
  if (error instanceof FreeRadicalError) {
    console.error(`Error ${error.statusCode}: ${error.message}`);
    console.error('Response:', error.response);
  }
}
```

---

## Testing

### Unit Tests

Test SDK methods independently:

```typescript
describe('FreeRadicalClient', () => {
  it('should fetch pages', async () => {
    const client = new FreeRadicalClient({
      baseUrl: 'http://localhost:8000',
      jwt: 'test-token'
    });
    
    const pages = await client.getPages();
    expect(pages).toBeInstanceOf(Array);
  });
});
```

### Integration Tests

Test against live API:

```typescript
describe('Integration Tests', () => {
  let client: FreeRadicalClient;
  
  beforeAll(() => {
    client = new FreeRadicalClient({
      baseUrl: process.env.TEST_API_URL,
      jwt: process.env.TEST_JWT
    });
  });
  
  it('should create and delete page', async () => {
    const page = await client.createPage({
      page_title: 'Test Page',
      page_url: '/test'
    });
    
    expect(page.uuid).toBeTruthy();
    
    await client.deletePage(page.uuid);
  });
});
```

### Mock Responses

Provide mock server for testing:

```typescript
import nock from 'nock';

nock('http://localhost:8000')
  .get('/page')
  .reply(200, [{ uuid: '123', page_title: 'Test' }]);
```

---

## Publishing

### NPM (TypeScript/JavaScript)

```bash
cd sdk/freeradical-sdk

# Update version
npm version patch

# Build
npm run build

# Publish
npm publish --access public
```

### PyPI (Python)

```bash
cd sdks/python

# Build distributions
python setup.py sdist bdist_wheel

# Upload to PyPI
twine upload dist/*
```

### Go Modules

```bash
# Tag version
git tag v1.0.0

# Push to GitHub
git push origin v1.0.0

# Users install via:
# go get github.com/your-org/freeradical-go-client@v1.0.0
```

---

## Contributing

### SDK Development Workflow

1. **Fork Repository**
2. **Create Feature Branch**: `git checkout -b sdk/add-ruby-client`
3. **Implement SDK** following architecture guidelines
4. **Add Tests** (minimum 80% coverage)
5. **Update Documentation**
6. **Submit Pull Request**

### Code Style

- Follow language-specific conventions (PEP8 for Python, etc.)
- Use consistent naming (camelCase vs snake_case per language)
- Add JSDoc/docstrings for all public methods
- Include usage examples in README

### Versioning

Follow [Semantic Versioning](https://semver.org/):
- **Major**: Breaking API changes
- **Minor**: New features, backward compatible
- **Patch**: Bug fixes

---

## Support

- **Issues**: [GitHub Issues](https://github.com/your-org/freeradical/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-org/freeradical/discussions)
- **Email**: support@freeradical.dev

---

## Additional Resources

- [Core Developer Guide](./CORE_DEVELOPER_GUIDE.md)
- [API Architecture](../architecture/api_design.md)
- [Payment System](../architecture/payment_architecture.md)
- [Plugin System](../architecture/plugin_system.md)

---

**Happy SDK Building!** 🚀
