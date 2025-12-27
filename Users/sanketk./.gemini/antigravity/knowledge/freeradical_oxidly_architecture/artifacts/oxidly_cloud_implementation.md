# Oxidly Cloud Platform Implementation
The Oxidly Cloud Platform serves as the SaaS entry point for the FreeRadical ecosystem, built with a Node.js/Express stack for high developer velocity and seamless integration with Handlebars templates.

## Technical Foundation
- **Entry Point**: `server.js` initializes the Express application.
- **Middleware**: Includes `cookie-parser`, `express.json()`, and `express.urlencoded()`.
- **View Engine**: Configured with `express-handlebars` using `.hbs` extensions and layouts/partials support.
- **Helpers**: Includes an `eq` helper for equality checks in templates (e.g., `{{#if (eq this.status 'published')}}`).

## Project Structure (`/oxidly`)
```text
oxidly/
├── Dockerfile          # Production-ready Node container
├── package.json        # Dependencies (Express, Handlebars, Axios)
├── server.js           # Main application logic & routing
├── public/             # Static assets (Tailwind CSS, JS)
├── services/           
│   └── api.js          # API Client factory (GET, POST, etc.)
├── controllers/        
│   ├── auth_controller.js # Signup, Login, Logout logic
│   ├── site_controller.js # Site CRUD and CNAME validation
│   └── content_controller.js # Pages and Posts CRUD
└── views/              # Handlebars templates
    ├── layouts/        # main.hbs, dashboard.hbs
    ├── partials/       # header.hbs, sidebar.hbs (with Lucide icons)
    ├── auth/           # login.hbs, signup.hbs, verify.hbs
    ├── sites/          # list.hbs, new.hbs, details.hbs
    ├── content/        # index.hbs, edit.hbs (CRUD UI)
    └── dashboard/      # index.hbs (Main site management UI)
```

## Key Templates
- **Main Layout**: Standard wrapper for marketing and auth pages with the public header.
- **Dashboard Layout**: Uses a persistent sidebar with navigation icons powered by Lucide.
- **Sidebar**: Features high-level navigation: Dashboard, Sites, Commerce, Customers, and Settings.

## API Integration Pattern
Oxidly uses a **Per-Request Client Factory** pattern in `services/api.js` to manage authentication context during Server-Side Rendering (SSR).

### `getClient(req)`
- **Logic**: Accepts the Express `req` object.
- **Auth**: Extracts the `auth_token` from `req.cookies`.
- **Headers**: Automatically injects `Authorization: Bearer <token>` into the Axios instance if the token exists.
- **Usage**: Controllers pass the current `req` to API service methods (e.g., `api.sites.list(req)`).

#### API Service Structure (`services/api.js`)
```javascript
const getClient = (req = null) => {
    const headers = { 'Content-Type': 'application/json' };
    if (req && req.cookies && req.cookies.auth_token) {
        headers['Authorization'] = `Bearer ${req.cookies.auth_token}`;
    }
    return axios.create({
        baseURL: process.env.API_URL || 'http://cms:8000/v1',
        headers: headers,
        timeout: 10000
    });
};

module.exports = {
    auth: {
        login: (credentials) => getClient().post('/user/login', credentials),
        signup: (userData) => getClient().post('/user', userData),
        checkLogin: (req) => getClient(req).get('/user/me'),
    },
    sites: {
        create: (siteData, req) => getClient(req).post('/sites', siteData),
        list: (req) => getClient(req).get('/sites'),
        get: (id, req) => getClient(req).get(`/sites/${id}`),
        validateCname: (domain) => getClient().post('/sites/validate-cname', { domain }),
        update: (id, data, req) => getClient(req).put(`/sites/${id}`, data),
        delete: (id, req) => getClient(req).delete(`/sites/${id}`),
    },
    pages: {
        list: (req) => getClient(req).get('/pages'),
        get: (id, req) => getClient(req).get(`/pages/${id}`),
        create: (data, req) => getClient(req).post('/pages', data),
        update: (id, data, req) => getClient(req).put(`/pages/${id}`, data),
        delete: (id, req) => getClient(req).delete(`/pages/${id}`),
        getModules: (id, req) => getClient(req).get(`/pages/${id}/modules`),
    },
    modules: {
        create: (data, req) => getClient(req).post('/modules', data),
        update: (id, data, req) => getClient(req).put(`/modules/${id}`, data),
        delete: (id, req) => getClient(req).delete(`/modules/${id}`),
    },
};
```

- **Validation**: Subsequent requests use the `getClient(req)` pattern to authenticate against the FreeRadical API.

## Content Management UI
The Content Management UI unified both static Pages and dynamic Posts (Articles) through a single resource interface.

### Controller Logic (`controllers/content_controller.js`)
- **Listing**: Fetches all pages via `api.pages.list(req)`.
- **Filtering**: UI provides client-side or simple query-based filtering by `article_type` (Page vs Article).
- **CRUD Operations**: Maps standard HTTP actions to the backend `/v1/pages` endpoint.
- **Relational Module Mapping**: 
    - **Create**: After creating a `Page`, the controller extracts the `uuid` and creates a companion "Main Content" `Module` with `field_type: "markdown"`.
    - **Edit**: Sequentially fetches the page metadata and the page's associated modules. It scans the modules for the "Main Content" block to populate the text editor.
    - **Update**: Performs an atomic-like update on both the `Page` (metadata) and the `Module` (content), creating the module if it was missing.
- **Handling Result Sets**: Controllers account for the backend returning either an object or a single-element array (common in Diesel-based filters).

### View Patterns (`views/content/`)
- **`index.hbs`**: Uses a responsive Tailwind table with Lucide icons. Displays Title, URL slug, Article Type (badge-coded), and Status.
- **`edit.hbs`**: A unified form for creating and editing content. Includes sections for basic info (Title, Slug), SEO metadata (Meta Title, Description), and publishing options (Status, Article Type).

## Content Architecture (Block-Based)
Unlike traditional CMS platforms that store a single opaque content blob in the `pages` table, FreeRadical uses a relational **Module** system.

### Data Model
- **Primary**: `pages` table stores metadata (title, slug, SEO).
- **Secondary**: `modules` table stores the actual content blocks.
- **Relationship**: Each `Module` is associated with a `page_uuid`.
- **Field Types**: Modules support `field_type` (e.g., `text`, `richtext`, `image`) and `content` (string/JSON).

### Integration Strategy
The editor interface (SimpleMDE) maps its output to a specific "Main Content" module. During a save operation, the controller must:
1. Update the `Page` metadata.
2. Fetch or create the associated `Module` for content.
3. Update the `Module` content.

## Development & Build
- **Docker Optimization**: 
    - **Local Exclusions**: A `.dockerignore` file in the `oxidly/` directory excludes `node_modules`, `.env`, and `.DS_Store`.
    - **Root Exclusions**: The root `.dockerignore` must exclude the `.git` directory and `target/` (Rust build artifacts). Excluding `.git` is critical for preventing massive context transfer times during development builds.
- **Service Orchestration**: Oxidly is defined as a service in the root `docker-compose.yml`, linked to the `cms` service via internal networking (`http://cms:8000`).
- **Environment Constraints**: `npm` and other build tools may not be available on the host machine. All dependency management and build steps must be executed within the Dockerized environment.

## Dependencies
- **Express**: Main framework.
- **Express-Handlebars**: Server-side rendering.
- **Axios**: API client for communication with the FreeRadical Rust backend.
- **Lucide-Icons**: SVG icons for the dashboard interface.
- **Tailwind CSS**: Utility-first CSS framework for responsive design.
