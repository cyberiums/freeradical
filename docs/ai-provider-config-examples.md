# AI Provider Configuration Examples

## Standard Endpoints (Default)

```javascript
// OpenAI
{
  provider: "openai",
  model: "gpt-4",
  endpoint: "https://api.openai.com/v1" // Default
}

// Anthropic
{
  provider: "anthropic",
  model: "claude-3-opus",
  endpoint: "https://api.anthropic.com/v1" // Default
}

// GCP Vertex AI
{
  provider: "gcp",
  model: "gemini-pro",
  endpoint: "https://us-central1-aiplatform.googleapis.com" // Default
  project: "my-project-id"
}

// Azure OpenAI
{
  provider: "azure",
  model: "gpt-4",
  endpoint: "https://{resource-name}.openai.azure.com" // Default
  deployment: "gpt-4-deployment"
}
```

## Custom Endpoints (Override)

### Azure Enterprise Regional Deployment
```javascript
{
  provider: "azure",
  model: "gpt-4-32k",
  endpoint: "https://my-company-eastus2.openai.azure.com", // Custom
  deployment: "gpt-4-32k-prod",
  apiVersion: "2024-02-01"
}
```

### Self-Hosted OpenAI-Compatible
```javascript
{
  provider: "openai",
  model: "llama-2-70b",
  endpoint: "https://ai.mycompany.com/v1", // Self-hosted
  apiKey: "internal-key-123"
}
```

### Corporate Proxy/Gateway
```javascript
{
  provider: "openai",
  model: "gpt-4",
  endpoint: "https://ai-gateway.corp.internal/openai", // Proxy
  apiKey: "corp-gateway-token"
}
```

### Regional Compliance Endpoint
```javascript
{
  provider: "gcp",
  model: "gemini-pro",
  endpoint: "https://europe-west1-aiplatform.googleapis.com", // EU region
  project: "eu-compliant-project"
}
```

## Admin UI Configuration

**Form Fields:**
- Provider Type (dropdown: OpenAI, Anthropic, GCP, Azure, Custom)
- Model Name (text input with suggestions)
- **Endpoint URL** (text input, pre-filled with default, editable)
- API Key (password input, encrypted at rest)
- Additional Config (JSON editor for provider-specific options)
- Test Connection (button)
- Budget Limits (daily/monthly token limits)

**Default Behavior:**
- Endpoint field pre-populated with standard URL
- Admin can override to custom endpoint
- Validation ensures endpoint is valid HTTPS URL
- Test connection validates endpoint + credentials
