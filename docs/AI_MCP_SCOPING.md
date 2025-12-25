# AI/MCP Data Scoping - Security Model

## Overview

All AI features in FreeRadical CMS implement **user-scoped data access** to ensure users can only access and manipulate data they have permission to view.

---

## Core Principle

> **Each user's AI interactions are scoped to their accessible data only.**

No user can use AI features to access, search, or generate recommendations for content they don't have permission to see.

---

## Access Control Model

### User Roles & Permissions

| Role | Can Read | Can Write | Can Configure AI |
|------|----------|-----------|------------------|
| **Guest** | Published content only | ❌ No | ❌ No |
| **User** | Published + Own content | ✅ Own content | ❌ No |
| **Admin** | All content | ✅ All content | ✅ Yes |

### Data Scopes

```rust
pub enum AIScope {
    ReadPublic,   // Published content (everyone)
    ReadOwn,      // Own content (authenticated)
    WriteOwn,     // Generate content (authenticated)
    Admin,        // AI provider config (admins only)
}
```

---

## Feature-by-Feature Scoping

### 1. Semantic Search (`/search/semantic`)

**How it works:**
1. User submits search query
2. System extracts user context from JWT
3. Filter page IDs by user permissions
4. Search only within accessible pages

**SQL-level filtering:**
```sql
-- Guest users
SELECT * FROM content_embeddings 
WHERE page_id IN (
    SELECT id FROM pages WHERE status = 'published'
)

-- Authenticated users  
SELECT * FROM content_embeddings
WHERE page_id IN (
    SELECT id FROM pages 
    WHERE status = 'published' OR author_id = :user_id
)

-- Admins
SELECT * FROM content_embeddings
-- No filtering (all pages)
```

**Example:**
```bash
# Guest search - only published
curl -X POST /search/semantic \
  -d '{"query": "rust cms"}'

# User search - published + own
curl -X POST /search/semantic \
  -H "Authorization: Bearer <user_token>" \
  -d '{"query": "my drafts"}'
```

---

### 2. AI Content Generation (`/ai/generate`)

**Access Control:**
- ✅ **Requires authentication** (no anonymous generation)
- ✅ Generated content is owned by requesting user
- ✅ Usage tracked per user for billing
- ❌ Cannot generate content for other users

**Scoping:**
```rust
// Extract user from JWT
let user = extract_user_context(&req)?;

// Verify user can write
check_scope(&user, AIScope::WriteOwn)?;

// Track usage per user
log_ai_usage(user.user_id, tokens_used, cost);
```

**Example:**
```bash
# User generates content (requires auth)
curl -X POST /ai/generate \
  -H "Authorization: Bearer <user_token>" \
  -d '{"prompt": "Write a blog post", "content_type": "blog_post"}'
```

---

### 3. Metadata Automation (`/ai/metadata/*`)

**Access Control:**
- ✅ **Requires authentication**
- ✅ Can only generate metadata for own content
- ✅ Cannot extract keywords from private content of others

**Scoping:**
```rust
// Verify user owns the page
if !verify_page_access(&user, page_id, pool).await? {
    return Err(CustomHttpError::Forbidden("Cannot access page"));
}

// Generate metadata only for accessible content
extract_keywords(user_accessible_content)
```

**Example:**
```bash
# Generate keywords for own page
curl -X POST /ai/metadata/keywords \
  -H "Authorization: Bearer <user_token>" \
  -d '{"content": "My blog post text", "max_items": 10}'
```

---

### 4. Recommendations (`/recommendations/related`)

**Access Control:**
- ✅ Only recommends content user can access
- ✅ Filters out private pages of other users
- ✅ Respects content visibility settings

**Scoping:**
```rust
// Get source page
verify_page_access(&user, source_page_id, pool).await?;

// Find similar pages (filtered by access)
let accessible_pages = get_accessible_page_ids(&user, pool).await?;

// Recommend only from accessible set
WHERE page_id IN (accessible_pages)
```

**Example:**
```bash
# Get recommendations (filtered by access)
curl -X POST /recommendations/related \
  -H "Authorization: Bearer <user_token>" \
  -d '{"page_id": 123, "limit": 5}'
```

---

### 5. AI Provider Management (`/admin/ai/providers/*`)

**Access Control:**
- ✅ **Admin-only** (strict enforcement)
- ❌ Regular users cannot view API keys
- ❌ Regular users cannot modify budgets

**Scoping:**
```rust
// Verify admin role
check_scope(&user, AIScope::Admin)?;

// Only admins can proceed
create_provider(provider_config)
```

**Example:**
```bash
# Admin creates provider (admin token required)
curl -X POST /admin/ai/providers \
  -H "Authorization: Bearer <admin_token>" \
  -d '{"provider_type": "openai", "api_key": "sk-..."}'
```

---

## Implementation Details

### JWT Token Structure

```json
{
  "sub": "user_id",
  "username": "john_doe",
  "role": "user",
  "permissions": ["read:public", "write:own"],
  "exp": 1703520000
}
```

### User Context Extraction

```rust
// In every AI endpoint
let user = extract_user_context(&req)?;

// User context contains:
pub struct UserContext {
    pub user_id: i32,       // 0 for guests
    pub username: String,   // "guest" for anonymous
    pub role: String,       // "guest", "user", "admin"
    pub permissions: Vec<String>,
}
```

### Authorization Checks

```rust
// 1. Extract user context
let user = extract_user_context(&req)?;

// 2. Check required scope
check_scope(&user, AIScope::WriteOwn)?;

// 3. Filter data by permissions
let pages = get_accessible_page_ids(&user, pool).await?;

// 4. Execute scoped operation
perform_ai_operation(pages)
```

---

## Security Guarantees

### ✅ What's Protected

1. **Private Content** - Users cannot search/analyze others' drafts
2. **API Keys** - Only admins see AI provider credentials
3. **Usage Data** - Users only see their own AI usage
4. **Generated Content** - Attributed to requesting user
5. **Embeddings** - Scoped to accessible content only

### ❌ What's Not Allowed

1. **Cross-user data access** - Cannot read others' content
2. **Anonymous generation** - Must be authenticated
3. **Privilege escalation** - Cannot bypass scopes
4. **Provider tampering** - Only admins manage providers

---

## Multi-Tenancy Support (Future)

```rust
pub struct UserContext {
    pub user_id: i32,
    pub org_id: Option<i32>,  // Organization scope
    pub team_id: Option<i32>, // Team scope
    pub role: String,
}

// Organization-level scoping
WHERE page_id IN (
    SELECT id FROM pages 
    WHERE org_id = :user_org_id
    AND (status = 'published' OR author_id = :user_id)
)
```

---

## Audit Trail

All AI operations are logged with user context:

```rust
log_ai_usage(
    user_id: user.user_id,
    operation: "semantic_search",
    query: query_text,
    results_count: 10,
    timestamp: now()
);
```

**Audit table:**
```sql
CREATE TABLE ai_usage_log (
    id BIGSERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    operation VARCHAR(50),
    -- User who performed the action
    created_at TIMESTAMP DEFAULT NOW()
);
```

---

## Testing

```bash
# Test guest access (should fail for generation)
curl -X POST /ai/generate -d '{"prompt": "test"}'
# Expected: 401 Unauthorized

# Test user access (should succeed for own content)
curl -X POST /ai/generate \
  -H "Authorization: Bearer <user_token>" \
  -d '{"prompt": "test"}'
# Expected: 201 Created

# Test admin access (should succeed)
curl -X POST /admin/ai/providers \
  -H "Authorization: Bearer <admin_token>" \
  -d '{"provider_type": "openai"}'
# Expected: 201 Created
```

---

## Best Practices

### For Developers

1. **Always extract user context first**
   ```rust
   let user = extract_user_context(&req)?;
   ```

2. **Check scope before operations**
   ```rust
   check_scope(&user, required_scope)?;
   ```

3. **Filter data by permissions**
   ```rust
   WHERE page_id IN (get_accessible_page_ids(&user))
   ```

4. **Log AI operations with user_id**
   ```rust
   log_usage(user.user_id, operation);
   ```

### For API Clients

1. **Include JWT in Authorization header**
   ```
   Authorization: Bearer <your_jwt_token>
   ```

2. **Handle 401/403 errors gracefully**
   - 401: Not authenticated (login required)
   - 403: Not authorized (insufficient permissions)

3. **Request only accessible content**
   - Don't assume access to all data
   - Respect permission boundaries

---

## Configuration

### Enable Strict Mode

```env
# In .env
AI_STRICT_AUTH=true          # Require auth for all AI features
AI_GUEST_SEARCH=false        # Disable guest search
AI_ADMIN_ONLY_PROVIDERS=true # Lock provider config to admins
```

### Rate Limiting (Per User)

```rust
// Per-user token budgets
daily_token_limit: 10000,      // Per user
monthly_budget_cents: 500,     // $5 per user/month
```

---

## Summary

**FreeRadical's AI scoping ensures:**

✅ **Privacy** - Users only access their data  
✅ **Security** - Admin features locked down  
✅ **Attribution** - All AI ops tracked per user  
✅ **Compliance** - Data isolation enforced  
✅ **Transparency** - Clear permission model

**Default stance: Deny access unless explicitly authorized.**
