# AI Feature Authorization & Access Control

## Overview

All AI features now implement user-based access control to ensure data privacy and security.

## Security Model

### User Scopes

1. **ReadPublic** - Read published content (everyone)
2. **ReadOwn** - Read own content (authenticated users)
3. **WriteOwn** - Generate/create content (authenticated users)
4. **Admin** - Full AI provider management (administrators only)

### Access Rules

**Guest Users (unauthenticated):**
- ✅ Can search published content
- ✅ Can see public recommendations
- ❌ Cannot generate content
- ❌ Cannot access AI admin features

**Authenticated Users:**
- ✅ Can search published + own content
- ✅ Can generate content
- ✅ Can see recommendations for accessible content
- ❌ Cannot access AI admin features

**Administrators:**
- ✅ Full access to all features
- ✅ Can manage AI providers
- ✅ Can view all content
- ✅ Can configure budgets

## Implementation

### User Context Extraction

```rust
use crate::services::ai_authorization_service::{extract_user_context, check_scope};

pub async fn my_handler(req: HttpRequest) -> Result<HttpResponse, CustomHttpError> {
    // Extract user from JWT/session
    let user = extract_user_context(&req)?;
    
    // Check authorization
    check_scope(&user, AIScope::WriteOwn)?;
    
    // Proceed with authorized operation
    // ...
}
```

### Search Authorization

Semantic search results are filtered by user permissions:

```rust
// Get accessible page IDs
let accessible_pages = get_accessible_page_ids(&user, pool).await?;

// Filter search results
WHERE page_id IN (accessible_pages)
```

### Recommendation Authorization

Recommendations only include content the user can access:

```rust
// Verify page access before showing recommendations
if !verify_page_access(&user, page_id, pool).await? {
    return Err(CustomHttpError::Forbidden("Access denied".to_string()));
}
```

### Provider Management

AI provider configuration requires admin role:

```rust
check_scope(&user, AIScope::Admin)?;
// Only admins can create/update/delete providers
```

## Data Scoping

### Published vs Private Content

- **Published**: Visible to everyone
- **Draft/Private**: Only visible to author and admins

### Multi-Tenancy Support

Future enhancement: Organization-based scoping

```rust
pub struct UserContext {
    pub user_id: i32,
    pub org_id: Option<i32>,  // Organization scope
    pub role: String,
}
```

## Security Features

1. **JWT Validation** - Extract user from Bearer token
2. **Role-Based Access** - Admin, User, Guest roles
3. **Content Filtering** - SQL-level permission filtering
4. **Audit Logging** - Track AI operations per user
5. **Rate Limiting** - Per-user token budgets

## API Changes

All AI endpoints now require authentication (except public search):

```bash
# Public access (no auth required)
POST /search/semantic?published_only=true

# Authenticated access
POST /ai/generate
Authorization: Bearer <jwt_token>

# Admin access
POST /admin/ai/providers
Authorization: Bearer <admin_jwt_token>
```

## Testing

Run authorization tests:

```bash
cargo test ai_authorization_service
```

## Future Enhancements

1. ��� Organization-based access control
2. 🔄 Team-level permissions
3. 🔄 Fine-grained content permissions
4. 🔄 API key-based access for integrations
5. 🔄 OAuth2 support
