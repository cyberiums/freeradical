# Enabling OAuth in FreeRadical

FreeRadical provides a built-in OAuth architecture that simplifies integrating third-party identity providers. This guide explains how to enable OAuth for a new provider (e.g., Google, GitHub, Microsoft) using the standard `OAuthService` and `oauth_providers` system.

## 1. Prerequisites

Ensure you have the following:
- Access to the `freeradical` codebase.
- A registered application with your OAuth provider (e.g., Google Cloud Console, GitHub Developer Settings).
- The `Client ID` and `Client Secret` from your provider.
- A configured Redirect URI (default: `http://localhost:8000/oauth/callback`).

## 2. Configuration (Environment Variables)

Add your provider's credentials to your environment configuration (e.g., `docker-compose.yml` or `.env`).

**Example (`docker-compose.yml`):**
```yaml
services:
  cms:
    environment:
      # ... other vars
      GOOGLE_CLIENT_ID: "your-client-id"
      GOOGLE_CLIENT_SECRET: "your-client-secret"
      GOOGLE_REDIRECT_URI: "http://localhost:8000/oauth/callback" # MUST match Google Console
      # For GitHub:
      # GITHUB_CLIENT_ID: "..."
      # GITHUB_CLIENT_SECRET: "..."
```

## 3. Database Seeding

The platform uses the `oauth_providers` table to manage active providers and resolve them during the callback. You must seed this table with your provider's details.

**Create a Migration:**
1. Create a new migration directory (e.g., `migrations_postgres/2025-12-30-030000_seed_oauth_providers`).
2. Create `up.sql`:

```sql
INSERT INTO oauth_providers (name, client_id, client_secret, enabled)
VALUES 
    ('google', 'your-client-id', 'your-client-secret', true)
ON CONFLICT DO NOTHING;
```

3. Create `down.sql`:

```sql
DELETE FROM oauth_providers WHERE name = 'google';
```

**Note:** While storing secrets in the database is supported by the schema, the `OAuthService` typically prioritizes Environment Variables for security. The database entry is primarily used to validate the provider's existence and map it to an ID for the `user_oauth_connections` table.

## 4. Backend Implementation

FreeRadical uses a centralized `OAuthService` (`src/services/oauth_service.rs`) and standardized controllers.

### A. Extend OAuthService (if adding a NEW provider)

If you are adding a provider not yet supported by `OAuthService` (currently supports 'google' and 'github'):

1. Modify `src/services/oauth_service.rs`.
2. Update `get_authorization_url` to handle the new provider name and return the correct auth URL.
3. Update `exchange_code_for_token` to handle the token exchange logic for the new provider.
4. Add a `fetch_{provider}_profile` method to get user details.

### B. Implement/Update Login Controller

Use `OAuthService::get_authorization_url` in your login handler.

**`src/controllers/oauth_controller.rs`:**
```rust
pub async fn google_login() -> impl Responder {
    match OAuthService::get_authorization_url(
        "google", 
        "http://localhost:8000/v1/auth/google/callback", 
        "state_token"
    ) {
        Ok(url) => HttpResponse::Found().append_header(("Location", url)).finish(),
        Err(e) => HttpResponse::InternalServerError().body(e),
    }
}
```

### C. Use Standard Callback Controller

The `src/controllers/oauth_callback_controller.rs` handles the code-to-token exchange and user creation/linking. ensure it is updated to use the correct Redirect URI matching your provider registration.

**`src/controllers/oauth_callback_controller.rs`:**
```rust
pub async fn google_callback(query: web::Query<OAuthCallbackQuery>, ...) {
    // ...
    let token = OAuthService::exchange_code_for_token(
        "google", 
        &query.code, 
        "http://localhost:8000/v1/auth/google/callback"
    ).await?;
    // ...
}
```

### D. Register Routes

Register the endpoints in `src/main.rs`:

```rust
.route("/auth/google", web::get().to(controllers::oauth_controller::google_login))
.route("/auth/google/callback", web::get().to(controllers::oauth_callback_controller::google_callback))
```

## 5. Frontend Integration (Oxidly)

Add a login button to your frontend view (e.g., `oxidly/views/auth/login.hbs`). The button should link directly to the backend login route.

```html
<a href="http://localhost:8000/v1/auth/google" class="btn-oauth">
    Sign in with Google
</a>
```

### Important Notes:
- **Proxying**: If you are running Oxidly (port 5005) and CMS (port 8000) separately, ensure the link points to the CMS backend.
- **Cookies**: The callback controller sets an HttpOnly cookie. Ensure your frontend can read/proxy this or that your domains match (localhost vs localhost).

## 6. Troubleshooting

- **Redirect URI Mismatch**: Ensure the URI in `oauth_callback_controller.rs` EXACTLY matches the one registered with the provider and the one in the JSON credentials.
- **Database Errors**: Ensure the `seed_oauth_providers` migration ran successfully so the callback controller can find the provider ID.
- **Diesel Errors**: If compilation fails on `diesel` macros, ensure `user_models.rs` and `schema.rs` are in sync.
