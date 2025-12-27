# Frontend Architecture

The FreeRadical and Oxidly ecosystem utilizes a hybrid frontend strategy to balance performance, SEO, and user interactivity.

## 1. Handlebars Template System (SSR/Static)
- **Role**: Primary rendering engine for marketing pages, blog posts, and public-facing site content.
- **Location**: `/templates` in the Rust backend.
- **Integration**: Leverages the `handlebars` Rust crate for server-side rendering.
- **Benefits**: Fast initial load times (9x faster than WordPress) and excellent SEO/GEO/AEO performance.

## 2. Admin Dashboard (SPA)
- **Role**: High-interactivity management interface for merchants and administrators within individual sites.
- **Stack**: 
    - **Build Tool**: Vite
    - **Framework**: React
    - **Language**: TypeScript
    - **Styling**: Tailwind CSS, PostCSS
- **Location**: `/admin`
- **Features**: 
    - Modern UI components for complex data management.
    - Consumes the FreeRadical Rust backend via REST/GraphQL APIs.

## 3. Oxidly Cloud Platform (SaaS Layer)
- **Role**: The main cloud-facing frontend for Oxidly.com, handling multi-tenant signup, pricing, and high-level site orchestration.
- **Stack**:
    - **Runtime**: Node.js
    - **Framework**: Express
    - **Templating**: Handlebars (via `express-handlebars`)
    - **Styling**: Tailwind CSS
- **Location**: `/oxidly`
- **Features**:
    - Centralized dashboard for managing multiple sites.
    - Authentication and Billing integrations.
    - Marketing pages and signup flow.

## 4. Implementation Strategy
- **Hybrid Delivery**: Public pages are served via Rust + Handlebars for maximum speed. Administrative tasks and complex site management are handled by the React SPA. The cloud platform (SaaS layer) uses Node.js/Express for rapid development of enterprise features.
- **Theming**: Liquid and Handlebars templates are used for end-user theme development, providing a flexible and familiar environment for developers.

## 5. API Interaction Pattern (SSR Data Fetching)
To handle authenticated API calls during Server-Side Rendering (SSR) without maintaining a global state that could leak between concurrent users, Oxidly implements a **Per-Request Client Factory**:
- **Mechanism**: A `getClient(req)` function in the service layer creates a transient Axios instance for each request.
- **Security**: The factory extracts the JWT from `req.cookies` (HTTP-only) and attaches it to the `Authorization` header.
- **Consistency**: This ensures that all data fetched for a specific user's dashboard view is scoped to their permissions and session.
