# Google OAuth Authentication Setup

## Overview

FreeRadical CMS uses Google OAuth for headless authentication. Users authenticate via Google, and the CMS generates a JWT token that the tenant application (Oxidly) uses for API authentication.

---

## Architecture

```
User → Oxidly → CMS OAuth Endpoint → Google → CMS Callback → Oxidly (with JWT)
```

### Flow Details:

1. **User clicks "Login with Google"** on Oxidly (http://localhost:5005/login)
2. **Browser redirects** to CMS OAuth endpoint: `http://localhost:8000/v1/auth/google`
3. **CMS redirects** to Google OAuth consent screen
4. **User authenticates** with Google account
5. **Google redirects back** to CMS callback: `http://localhost:8000/v1/auth/google/callback?code=...`
6. **CMS processes OAuth**:
   - Exchanges authorization code for access token
   - Fetches user profile from Google
   - Creates/updates user in database
   - Generates JWT token (24-hour expiry)
7. **CMS redirects to Oxidly** with token: `http://localhost:5005? token=<JWT>`
8. **Oxidly JavaScript handler**:
   - Extracts token from URL
   - Stores in localStorage
   - Decodes user info
   - Redirects to dashboard

---

## Google Cloud Console Setup

### 1. Create OAuth 2.0 Credentials

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Select project: `fastbuilderfx`
3. Navigate to **APIs & Services** → **Credentials**
4. Click **Create Credentials** → **OAuth 2.0 Client ID**
5. Application type: **Web application**
6. Name: `FreeRadical Local Development`

### 2. Configure Authorized Redirect URIs

Add these URIs for local development:

```
http://localhost:8000/v1/auth/google/callback
http://127.0.0.1:8000/v1/auth/google/callback
```

For production, add your domain:

```
https://your-domain.com/v1/auth/google/callback
```

### 3. Get Credentials

After creating, you'll receive:
- **Client ID**: `YOUR_GOOGLE_CLIENT_ID_HERE`
- **Client Secret**: `YOUR_GOOGLE_CLIENT_SECRET_HERE`

---

## Environment Configuration

### CMS (.env)

```bash
# Google OAuth
GOOGLE_CLIENT_ID=YOUR_GOOGLE_CLIENT_ID_HERE
GOOGLE_CLIENT_SECRET=YOUR_GOOGLE_CLIENT_SECRET_HERE
GOOGLE_REDIRECT_URI=http://localhost:8000/v1/auth/google/callback

# OAuth Success Redirect (where to send users after auth)
OAUTH_SUCCESS_REDIRECT_URL=http://localhost:5005

# JWT Secret (for signing tokens)
JWT_SECRET=your-secret-key-change-in-production
```

### Oxidly (.env)

```bash
# API URL (used in templates for OAuth links)
API_URL=http://localhost:8000/v1
```

---

## Backend Implementation

### OAuth Callback Controller

File: `/src/controllers/oauth_callback_controller.rs`

Key features:
- Exchanges OAuth code for access token
- Fetches user profile from Google
- Creates user if doesn't exist (password = NULL for OAuth users)
- Links OAuth account to user
- **Generates JWT token** with user ID, email, and role
- **Redirects to tenant app** with token in URL

```rust
// Generate JWT token
let jwt_token = create_jwt_token(user_id, email, "user")?;

// Redirect with token
let redirect_url = env::var("OAUTH_SUCCESS_REDIRECT_URL")?;
let redirect_with_token = format!("{}?token={}", redirect_url, jwt_token);

HttpResponse::Found()
    .append_header(("Location", redirect_with_token))
    .finish()
```

### JWT Token Structure

```json
{
  "sub": "4",                          // User ID
  "email": "user@example.com",         // User email
  "role": "user",                      // User role
  "exp": 1704444444                    // Expiration (Unix timestamp)
}
```

---

## Frontend Implementation

### Login Button

File: `/oxidly/views/auth/login.hbs`

```html
<a href="{{apiUrl}}/auth/google"
   class="w-full flex justify-center items-center py-3 px-4 border rounded-2xl">
  <svg class="h-5 w-5 mr-2"><!-- Google icon --></svg>
  Google
</a>
```

### OAuth Handler Script

File: `/oxidly/public/js/oauth-handler.js`

Automatically runs on page load:

```javascript
// Check for token in URL
const urlParams = new URLSearchParams(window.location.search);
const token = urlParams.get('token');

if (token) {
    // Store token
    localStorage.setItem('auth_token', token);
    
    // Decode and store user info
    const payload = JSON.parse(atob(token.split('.')[1]));
    localStorage.setItem('user_email', payload.email);
    localStorage.setItem('user_id', payload.sub);
    
    // Clean URL and redirect
    window.history.replaceState({}, document.title, window.location.pathname);
    window.location.href = '/dashboard';
}
```

### Helper Functions

```javascript
// Check if user is logged in
function isLoggedIn() {
    const token = localStorage.getItem('auth_token');
    if (!token) return false;
    
    const payload = JSON.parse(atob(token.split('.')[1]));
    const now = Math.floor(Date.now() / 1000);
    return payload.exp > now;  // Check expiration
}

// Get auth token for API calls
function getAuthToken() {
    return localStorage.getItem('auth_token');
}

// Logout
function logout() {
    localStorage.removeItem('auth_token');
    localStorage.removeItem('user_email');
    localStorage.removeItem('user_id');
    window.location.href = '/';
}
```

---

## Making Authenticated API Calls

### Add Token to Axios

File: `/oxidly/services/api.js`

```javascript
const axios = require('axios');

// Get token from localStorage
const token = localStorage.getItem('auth_token');

// Set default Authorization header
if (token) {
    axios.defaults.headers.common['Authorization'] = `Bearer ${token}`;
}

// Make API calls
const response = await axios.get('/pages');
```

---

## Database Schema

### Users Table

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL,  -- Email for OAuth users
    password VARCHAR(255),                   -- NULL for OAuth users
    tenant_id INTEGER REFERENCES tenants(id),
    two_factor_enabled BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT NOW()
);
```

### OAuth Connections Table

```sql
CREATE TABLE user_oauth_connections (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    provider_id INTEGER REFERENCES oauth_providers(id),
    provider_user_id VARCHAR(255),  -- Google user ID
    access_token TEXT,
    refresh_token TEXT,
    expires_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

---

## Testing

### 1. Test OAuth Flow

```bash
# Start all services
docker-compose up -d

# Navigate to login
open http://localhost:5005/login

# Click "Google" button
# Should redirect to Google → back to Oxidly with token
```

### 2. Verify Token Storage

Open browser console (F12):

```javascript
// Check stored token
localStorage.getItem('auth_token')
// Should return: "eyJ0eXAiOiJKV1QiLCJhbGc..."

// Check user email
localStorage.getItem('user_email')
// Should return: "your-email@gmail.com"

// Check login status
isLoggedIn()
// Should return: true
```

### 3. Test API Authentication

```javascript
// Make authenticated API call
fetch('http://localhost:8000/v1/pages', {
    headers: {
        'Authorization': `Bearer ${localStorage.getItem('auth_token')}`
    }
})
.then(r => r.json())
.then(console.log);
```

---

## Troubleshooting

### Token not being stored

**Issue**: Redirects to Oxidly but token isn't saved

**Fix**: Check browser console for JavaScript errors. Ensure `oauth-handler.js` is loaded:

```html
<!-- In landing.hbs -->
<script src="/js/oauth-handler.js"></script>
```

### Redirect URI mismatch

**Issue**: `Error 400: redirect_uri_mismatch`

**Fix**: Add the exact URI to Google Cloud Console:
1. Go to Credentials
2. Edit OAuth 2.0 Client
3. Add: `http://localhost:8000/v1/auth/google/callback`

### Password constraint error

**Issue**: `null value in column "password" violates not-null constraint`

**Fix**: Make password column nullable:

```sql
ALTER TABLE users ALTER COLUMN password DROP NOT NULL;
```

### Token expired

**Issue**: `isLoggedIn()` returns false

**Fix**: Tokens expire after 24 hours. Login again to get new token.

---

## Security Considerations

### Production Checklist

- [ ] Use HTTPS for all OAuth redirects
- [ ] Store JWT_SECRET in secure environment (GCP Secret Manager)
- [ ] Rotate Google OAuth Client Secret periodically
- [ ] Implement OAuth state parameter for CSRF protection
- [ ] Add token refresh mechanism
- [ ] Implement proper logout (token blacklist)
- [ ] Use httpOnly cookies instead of localStorage (if same-domain)
- [ ] Add rate limiting to OAuth endpoints
- [ ] Log OAuth events for audit trail

---

## Database Credentials

### OAuth Users (Created by Google Auth)

| Email | Auth Type | User ID | Created |
|-------|-----------|---------|---------|
| prabhat.iit@gmail.com | Google OAuth | 4 | Auto |
| prabhatkr@gmail.com | Google OAuth | 5 | Auto |

### Password Users

| Username | Password | User ID |
|----------|----------|---------|
| admin@localhost.com | `password` | 2 |
| root | *(empty)* | 1 |

### Database Connection

```
Host: localhost:5432
Database: freeradical
Username: freeradical
Password: password
```

---

## Related Documentation

- [OAuth 2.0 Specification](https://oauth.net/2/)
- [Google OAuth Documentation](https://developers.google.com/identity/protocols/oauth2)
- [JWT.io](https://jwt.io/) - Decode and verify JWT tokens
- Backend: `src/controllers/oauth_callback_controller.rs`
- Frontend: `oxidly/public/js/oauth-handler.js`
