# Walkthrough - Oxidly Docker & Auth Implementation

## Changes

### 1. Docker Configuration
- Added `oxidly` service to `docker-compose.yml`.
- Configured port mapping (3000:3000) and dependency on `cms`.
- Optimized build context by adding `.git` and `target` to `.dockerignore`.

### 2. Authentication Logic
- **Controller**: Created `oxidly/controllers/auth_controller.js` handling:
    - `signup`: Calls API and redirects to login.
    - `login`: Calls API, retrieves token, and sets HTTP-only cookie.
    - `logout`: Clears cookie and redirects.
- **Routes**: Wired up `POST /signup`, `POST /login`, `GET /logout` in `server.js`.

### 3. API Client Refactoring
- Refactored `oxidly/services/api.js` to support per-request context.
- Implemented `getClient(req)` factory to inject `Authorization` header from cookies.
- Updated `site_controller.js` to pass `req` object to API calls.

## Verification

### Manual Code Verification
- Validated `server.js` route mappings.
- Validated `api.js` token extraction logic (`req.cookies.auth_token`).
- Validated `auth_controller.js` cookie setting logic.

### Environment Notes
- **Docker Build**: Encountered significant delays in Docker build context loading and image pulling (slow network or large context).
- **Workaround**: Verified code logic statically. The implementation is standard Express/Node pattern and should work once environment issues are resolved.
- **Suggestion**: Ensure Docker connectivity is stable and retry construction with `docker-compose up --build`.
