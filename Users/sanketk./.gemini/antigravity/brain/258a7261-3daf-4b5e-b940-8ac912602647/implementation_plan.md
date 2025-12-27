# Implementation Plan - Oxidly Initialization & Auth

## Goal
To configure the Docker environment for `oxidly` (Cloud Frontend) and implement functional Authentication (Signup, Login) by connecting the Node.js frontend to the FreeRadical Rust backend.

## Proposed Changes

### Configuration
#### [MODIFY] [docker-compose.yml](file:///Users/sanketk./freeradical/docker-compose.yml)
- Add `oxidly` service configuration.
- Map port 3000.
- Link to `cms` service (backend).

### Oxidly Codebase
#### [NEW] [oxidly/controllers/auth_controller.js](file:///Users/sanketk./freeradical/oxidly/controllers/auth_controller.js)
- Implement `signup` method:
    - Receive form data.
    - Call `api.auth.signup`.
    - Redirect to login or verify page on success.
    - Render signup page with error on failure.
- Implement `login` method:
    - Receive email/password.
    - Call `api.auth.login`.
    - Store JWT token in HTTP-only cookie.
    - Redirect to dashboard.
    - Render login page with error on failure.
- Implement `logout` method:
    - Clear cookie.
    - Redirect to home.

#### [MODIFY] [oxidly/server.js](file:///Users/sanketk./freeradical/oxidly/server.js)
- Import `auth_controller`.
- Update Routes:
    - `POST /signup` -> `authController.signup`
    - `POST /login` -> `authController.login`
    - `GET /logout` -> `authController.logout`

#### [MODIFY] [oxidly/services/api.js](file:///Users/sanketk./freeradical/oxidly/services/api.js)
- Implement interceptor to read token from cookies (using `cookie-parser` on server side, or passing request context).
- *Self-correction*: `api.js` is used in Node environment (SSR/Controller). It needs access to the request's cookies.
- I will modify `api.js` to potentially accept a context or token for authenticated requests, or handle it in the controller.
- **Refinement**: simpler approach is to have methods accept `token` or `req` as argument, or instantiate the client per request. Given `api.js` exports a singleton-like object, I might need to refactor it or just pass the token in headers manually in the controller for now. I'll see what fits best.

## Verification Plan

### Automated
- **Docker Build**: Verify `docker-compose up --build oxidly` succeeds.
- **Connectivity Check**:
    - Use `curl` inside the container (or from host) to check if `oxidly` can reach `cms` (e.g., `curl http://cms:8000/health`).

### Manual Verification
1. **Start Services**: `docker-compose up -d postgres redis cms oxidly`.
2. **Signup**:
    - Open `http://localhost:3000/signup`.
    - Fill details (name, email, password).
    - Submit.
    - Verify redirection to Login or Verify page.
    - Verify user exists in DB (optional, via backend logs).
3. **Login**:
    - Open `http://localhost:3000/login`.
    - Fill credentials.
    - Submit.
    - Verify redirection to `/dashboard`.
    - Check browser cookies for `auth_token`.
