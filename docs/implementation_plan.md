# Implementation Plan - Oxidly Auth & Dashboard Foundation

## Goal Description
Implement the core authentication flows (Login, Signup) and the foundational Dashboard Shell for the Oxidly Cloud Platform. This enables user onboarding and access to the protected SaaS features.

## Proposed Changes

### `oxidly/views`
#### [NEW] `oxidly/views/auth/login.hbs`
- Login page with "Voltaic" design.
- Email/Password fields.
- "Forgot Password" link.

#### [NEW] `oxidly/views/auth/signup.hbs`
- Signup page with Plan selection context (optional).
- Name, Email, Password fields.
- "Start Free Trial" CTA.

#### [NEW] `oxidly/views/layouts/dashboard.hbs`
- Main layout for protected routes.
- Sidebar navigation (Sites, Store, Analyze, Team, Settings).
- Header (Search, Notifications, User Profile).

#### [NEW] `oxidly/views/dashboard/index.hbs`
- Dashboard home/overview.
- Quick stats (Sites active, Recent orders).

### `oxidly/controllers`
#### [NEW] `oxidly/controllers/auth_controller.js`
- Handle `GET /login`, `GET /signup`.
- Handle `POST /login`, `POST /signup` (proxy to Rust API).
- Session/Cookie management.

### `oxidly/routes` (in `server.js` or separate)
- Register auth routes.
- Register dashboard routes (protected).

## Verification Plan

### Manual Verification
1.  **Browser Check:**
    - Navigate to `/signup` -> Verify UI.
    - Submit Signup form -> Check backend proxy/response.
    - Navigate to `/login` -> Verify UI.
    - Submit Login form -> Check redirection to `/dashboard`.
    - Verify `/dashboard` loads with the Layout + Sidebar.
