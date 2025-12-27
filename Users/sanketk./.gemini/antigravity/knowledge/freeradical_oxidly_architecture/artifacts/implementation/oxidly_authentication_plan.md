# Oxidly Authentication & Docker Integration - Implementation Record

**Status**: ✅ COMPLETED

## Objective
Enable user authentication (Signup/Login) in the Oxidly frontend by connecting it to the FreeRadical Rust backend and integrating the service into the primary Docker stack.

## Architecture & Implementation Details

### 1. Docker Integration
- **File**: `docker-compose.yml` (Root)
- **Action**: Add `oxidly` service.
- **Config**:
  - Build context: `./oxidly`
  - Ports: `3000:3000`
  - Depends on: `cms` (backend)
  - Environment: `API_URL=http://cms:8000/v1`

### 2. Authentication Controller
- **File**: `oxidly/controllers/auth_controller.js`
- **Methods**:
  - `signup(req, res)`: Form submission handler. Calls backend `/user` endpoint.
  - `login(req, res)`: Authenticates user via backend `/user/login`.
  - `logout(req, res)`: Clears the HTTP-only cookie.
- **Session Management**: JWT tokens returned from the backend will be stored in an HTTP-only cookie named `auth_token`.

### 3. Server Routing
- **File**: `oxidly/server.js`
- **Updates**:
  - `app.post('/signup', authController.signup)`
  - `app.post('/login', authController.login)`
  - `app.get('/logout', authController.logout)`

### 4. API Client Interceptor
- **File**: `oxidly/services/api.js`
- **Update**: Implement a request interceptor to attach the `Authorization: Bearer <token>` header to requests if a token is present in the session/cookies.

## Verification
- **Automated**: `docker-compose up --build` and verify service health.
- **Manual**: Test the signup/login flow in the browser and verify the `auth_token` cookie presence.
