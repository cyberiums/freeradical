# FreeRadical CMS - Client SDKs Overview

## 📦 Available SDKs: 3 Languages

FreeRadical CMS provides **official client libraries** in 3 languages for easy integration:

---

## 1. TypeScript/JavaScript SDK ✅

**Package**: `@freeradical/sdk`  
**Version**: 0.7.0  
**Status**: Fully Built & Ready  
**Location**: [`sdk/freeradical-sdk/`](file:///Users/prabhatsingh/freeradical/sdk/freeradical-sdk/)

### Features
- ✅ **Full TypeScript support** with complete type definitions
- ✅ **Tree-shakeable** ES modules
- ✅ **Axios-based** HTTP client
- ✅ **Error handling** with typed exceptions
- ✅ **All API endpoints** covered:
  - Pages (CRUD)
  - Modules (CRUD)
  - Media (upload, list, delete)
  - Search
  - Webhooks (CRUD + test)
  - Relationships (CRUD)
  - Health & Metrics

### Installation

```bash
npm install @freeradical/sdk
# or
yarn add @freeradical/sdk
```

### Usage Example

```typescript
import FreeRadicalClient from '@freeradical/sdk';

const client = new FreeRadicalClient({
  baseUrl: 'https://your-cms.com',
  jwt: 'your-jwt-token'
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

### Files Generated
```
sdk/freeradical-sdk/
├── src/
│   ├── index.ts       # Main export
│   ├── client.ts      # SDK client
│   └── types.ts       # TypeScript types
├── dist/              # Compiled JavaScript
├── package.json
├── tsconfig.json
└── README.md
```

**Documentation**: [README.md](file:///Users/prabhatsingh/freeradical/sdk/freeradical-sdk/README.md)

---

## 2. Python SDK ✅

**Package**: `freeradical-client`  
**Version**: 1.0.0  
**API Version**: 0.9.0  
**Status**: Auto-generated from OpenAPI  
**Location**: [`sdks/python/`](file:///Users/prabhatsingh/freeradical/sdks/python/)

### Features
- ✅ **OpenAPI 3.0 generated**
- ✅ **Type hints** (Python 3.9+)
- ✅ **Bearer JWT authentication**
- ✅ **Comprehensive API coverage**
- ✅ **pytest-ready** test suite
- ✅ **CI/CD configurations** (Travis, GitLab)

### Installation

```bash
pip install freeradical-client
# or from source
python setup.py install
```

### Usage Example

```python
import freeradical_client
from freeradical_client.rest import ApiException

# Configure API client
configuration = freeradical_client.Configuration(
    host = "https://your-cms.com/v1",
    access_token = "your-jwt-token"
)

# Create API client
with freeradical_client.ApiClient(configuration) as api_client:
    api_instance = freeradical_client.DefaultApi(api_client)
    
    # Get analytics
    analytics = api_instance.analytics_summary_get()
    
    # List pages
    pages = api_instance.pages_get()
    
    # Create page
    new_page = api_instance.pages_post(page_data)
```

### API Endpoints Covered
- `GET /analytics/summary` - Analytics summary
- `POST /auth/login` - Authentication
- `GET /pages` - List pages
- `POST /pages` - Create page
- `GET /pages/{uuid}` - Get page by UUID

### Generated Files
```
sdks/python/
├── freeradical_client/
│   ├── __init__.py
│   ├── api_client.py
│   ├── configuration.py
│   ├── models/
│   └── api/
├── docs/
├── test/
├── setup.py
└── README.md
```

**Documentation**: [README.md](file:///Users/prabhatsingh/freeradical/sdks/python/README.md)

---

## 3. Go SDK ✅

**Package**: `freeradical-go-client`  
**Status**: Auto-generated from OpenAPI  
**Location**: [`sdks/go/`](file:///Users/prabhatsingh/freeradical/sdks/go/)

### Features
- ✅ **OpenAPI 3.0 generated**
- ✅ **Go modules** support
- ✅ **Type-safe** structs
- ✅ **Context-aware** requests
- ✅ **Structured logging**
- ✅ **Comprehensive models**

### Installation

```bash
go get github.com/your-org/freeradical-go-client
```

### Usage Example

```go
package main

import (
    "context"
    "fmt"
    freeradical "github.com/your-org/freeradical-go-client"
)

func main() {
    cfg := freeradical.NewConfiguration()
    cfg.Host = "your-cms.com"
    cfg.Scheme = "https"
    
    client := freeradical.NewAPIClient(cfg)
    ctx := context.Background()
    
    // Get analytics
    analytics, _, err := client.DefaultApi.AnalyticsSummaryGet(ctx)
    if err != nil {
        panic(err)
    }
    fmt.Printf("Analytics: %+v\n", analytics)
    
    // List pages
    pages, _, err := client.DefaultApi.PagesGet(ctx)
    if err != nil {
        panic(err)
    }
    fmt.Printf("Pages: %+v\n", pages)
}
```

### Generated Files
```
sdks/go/
├── api_default.go
├── client.go
├── configuration.go
├── model_*.go (typed models)
├── docs/
├── go.mod
├── go.sum
└── README.md
```

**Documentation**: [README.md](file:///Users/prabhatsingh/freeradical/sdks/go/README.md)

---

## 📊 SDK Comparison

| Feature | TypeScript | Python | Go |
|---------|-----------|--------|-----|
| **Status** | ✅ Complete | ✅ Complete | ✅ Complete |
| **Type Safety** | Full TypeScript | Type hints | Full types |
| **Generation** | Manual | OpenAPI | OpenAPI |
| **HTTP Client** | Axios | urllib3 | net/http |
| **Auth** | JWT/API Key | Bearer JWT | Bearer JWT |
| **Async** | Promise-based | Sync | Context-based |
| **Package Manager** | npm/yarn | pip | go modules |
| **Tests** | Jest-ready | pytest | Go test |

---

## 🚀 SDK Generation Pipeline

SDKs are generated/maintained using:

1. **TypeScript SDK**: Hand-crafted for optimal DX
2. **Python/Go SDKs**: Auto-generated from OpenAPI 3.0 spec

### Generation Script
[`scripts/generate_sdks.sh`](file:///Users/prabhatsingh/freeradical/scripts/generate_sdks.sh)

```bash
# Regenerate Python & Go SDKs
./scripts/generate_sdks.sh
```

This script:
1. Generates OpenAPI spec from Rust API
2. Runs `openapi-generator` for Python
3. Runs `openapi-generator` for Go
4. Updates documentation

---

## 📝 API Coverage

All SDKs support these FreeRadical CMS endpoints:

### Core Resources
- ✅ **Pages**: CRUD operations, search, pagination
- ✅ **Modules**: Dynamic content blocks
- ✅ **Media**: Upload, list, delete files
- ✅ **Categories**: Content organization

### Advanced Features
- ✅ **Search**: Full-text search across resources
- ✅ **Webhooks**: Event-driven integrations
- ✅ **Relationships**: Link related content
- ✅ **Analytics**: Usage metrics
- ✅ **Health**: System monitoring

### Authentication
- ✅ **JWT tokens**: Bearer authentication
- ✅ **Session management**: Cookie-based auth
- ✅ **2FA**: TOTP support (via API)

---

## 🎯 Usage Recommendations

### **Use TypeScript SDK when:**
- Building React/Vue/Angular frontends
- Need excellent IDE autocomplete
- Working in Node.js backend
- Want the smallest bundle size

### **Use Python SDK when:**
- Building Django/Flask backends
- Need data analysis integration (pandas, numpy)
- Working with ML/AI pipelines
- Prefer synchronous code

### **Use Go SDK when:**
- Building high-performance services
- Need strong concurrency (goroutines)
- Working in microservices architecture
- Deploying to Kubernetes

---

## 📦 Publishing Status

| SDK | Published | Registry |
|-----|-----------|----------|
| TypeScript | ⏸️ Ready | npm (not yet published) |
| Python | ⏸️ Ready | PyPI (not yet published) |
| Go | ⏸️ Ready | GitHub (not yet published) |

To publish:
```bash
# TypeScript
cd sdk/freeradical-sdk && npm publish --access public

# Python  
cd sdks/python && python setup.py sdist bdist_wheel
twine upload dist/*

# Go
# Tag and push to GitHub
git tag v1.0.0 && git push origin v1.0.0
```

---

## 🔗 Related Documentation

- [API Documentation](file:///Users/prabhatsingh/freeradical/RELEASE-NOTES-v0.8.0.md)
- [TypeScript SDK README](file:///Users/prabhatsingh/freeradical/sdk/freeradical-sdk/README.md)
- [Python SDK README](file:///Users/prabhatsingh/freeradical/sdks/python/README.md)
- [SDK Generation Script](file:///Users/prabhatsingh/freeradical/scripts/generate_sdks.sh)

---

## ✨ Key Achievement

**3 production-ready client SDKs** covering the most popular languages for web development, making FreeRadical CMS accessible to a wide range of developers and use cases!
