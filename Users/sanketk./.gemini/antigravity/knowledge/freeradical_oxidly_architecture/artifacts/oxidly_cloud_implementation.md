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
│   ├── content_controller.js # Pages and Posts CRUD
│   ├── theme_controller.js   # Theme activation and upload
│   ├── user_controller.js    # Profile management and identity logic
│   ├── store_controller.js   # Commerce setup and management
│   ├── product_controller.js # Catalog management
│   └── order_controller.js   # Order fulfillment and tracking
└── views/              # Handlebars templates
    ├── layouts/        # main.hbs, dashboard.hbs
    ├── partials/       # header.hbs, sidebar.hbs (with Lucide icons)
    ├── auth/           # login.hbs, signup.hbs, verify.hbs
    ├── sites/          # list.hbs, new.hbs, details.hbs
    ├── content/        # index.hbs, edit.hbs (CRUD UI)
    ├── themes/         # index.hbs, upload.hbs (Theme management)
    ├── users/          # profile.hbs (User profile management)
    ├── store/          # setup.hbs (Commerce setup wizard)
    ├── products/       # index.hbs, form.hbs (Catalog management)
    └── orders/         # index.hbs, details.hbs (Order fulfillment)
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
    themes: {
        list: (req) => getClient(req).get('/themes'),
        upload: (formData, req) => {
            const client = getClient(req);
            client.defaults.headers['Content-Type'] = 'multipart/form-data';
            return client.post('/themes/upload', formData);
        },
        activate: (id, req) => getClient(req).post(`/themes/${id}/activate`),
    },
    users: {
        get: (username, req) => getClient(req).get(`/user/${username}`),
        update: (username, data, req) => getClient(req).put(`/user/${username}`, data),
    },
    products: {
        list: (req) => getClient(req).get('/products'),
        get: (id, req) => getClient(req).get(`/products/${id}`),
        create: (data, req) => getClient(req).post('/products', data),
        update: (id, data, req) => getClient(req).put(`/products/${id}`, data),
        delete: (id, req) => getClient(req).delete(`/products/${id}`),
    },
    orders: {
        list: (req) => getClient(req).get('/orders'),
        get: (id, req) => getClient(req).get(`/orders/${id}`),
        updateStatus: (id, status, req) => getClient(req).put(`/orders/${id}/status`, { status }),
    },
    payments: {
        providers: (req) => getClient(req).get('/payments/providers'),
    },
};
```

- **Validation**: Subsequent requests use the `getClient(req)` pattern to authenticate against the FreeRadical API.

## Routing Table
Oxidly's routes are defined in `server.js` and mapped to their respective controllers:

| Path | Method | Controller Action | Description |
|------|--------|-------------------|-------------|
| `/` | GET | `(inline)` | Home page |
| `/login` | GET/POST | `auth.login` | Authentication |
| `/signup` | GET/POST | `auth.signup` | User Registration |
| `/logout` | GET | `auth.logout` | Session Termination |
| `/dashboard` | GET | `(inline)` | Main management portal |
| `/sites` | GET | `sites.index` | List managed sites |
| `/sites/new` | GET | `sites.new` | Site creation form |
| `/sites` | POST | `sites.create` | Create site logic |
| `/sites/:id` | GET | `sites.show` | Site details/metrics |
| `/content` | GET | `content.index` | List pages and posts |
| `/content/new` | GET | `content.new` | Content creation form |
| `/content` | POST | `content.create` | Create page/post logic |
| `/content/:id/edit`| GET | `content.edit` | Modify page/post |
| `/content/:id` | POST | `content.update` | Update logic |
| `/content/:id/delete`| POST | `content.delete` | Removal logic |
| `/themes` | GET | `themes.index` | List/Manage themes |
| `/themes/new` | GET | `themes.new` | Upload theme form |
| `/themes` | POST | `themes.create` | Process upload |
| `/themes/:id/activate`| POST | `themes.activate` | Set active theme |
| `/profile` | GET | `user.profile` | Identity/Profile view |
| `/profile` | POST | `user.update` | Update profile data |
| `/products` | GET | `products.index` | List products |
| `/products/new` | GET | `products.new` | New product form |
| `/products` | POST | `products.create` | Create product logic |
| `/products/:id/edit`| GET | `products.edit` | Edit product form |
| `/products/:id` | POST | `products.update` | Update product logic |
| `/products/:id/delete`| POST | `products.delete` | Delete product logic |
| `/store/setup` | GET | `store.wizard` | Commerce setup wizard |
| `/store/setup` | POST | `store.save` | Save commerce config |
| `/orders` | GET | `orders.index` | List merchant orders |
| `/orders/:id` | GET | `orders.show` | Order detail view |
| `/orders/:id/status`| POST | `orders.updateStatus`| Update order fulfillment |
| `/store/settings` | GET | `store.settings` | Store configuration UI |
| `/store/settings` | POST | `store.updateSettings`| Update preferences |


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

## Editor Integration (SimpleMDE)
Oxidly integrates **SimpleMDE** to provide a user-friendly Markdown editing experience for pages and posts.

### Assets & Initialization
- **CDN**: CSS and JS for SimpleMDE are loaded from `cdn.jsdelivr.net`.
- **Target**: A `<textarea id="content">` element in `edit.hbs`.
- **Initialization Script**:
  ```javascript
  var simplemde = new SimpleMDE({ element: document.getElementById("content") });
  ```

### Form Submission Lifecycle
1. **Load**: `ContentController.edit` fetches the "Main Content" module and injects the raw Markdown into the textarea.
2. **Edit**: SimpleMDE intercepts the textarea and provides the rich editor UI.
3. **Submit**: The form posts the textarea's current value (synced by SimpleMDE) along with a `main_module_id` hidden field.
4. **Save**: `ContentController.update` performs the multi-step save to `api.pages` and `api.modules`.

## Theme Management
Theme management in Oxidly allows users to customize the visual presentation of their sites.

### Design Principles
- **Separation of Concerns**: Themes are managed as discrete units containing templates and assets.
- **Architectural Gap**: The FreeRadical Rust backend currently lacks a native theme-management controller (verified by scanning `controllers/`). Themes are currently loaded from a static `./templates` directory.
- **Implementation Strategy**: Oxidly implements the UI for listing and uploading themes (zip format), while the API client (`services/api.js`) is prepared for future backend endpoints (`/themes`).
- **Mock Fallback**: Due to the backend gap, `ThemeController.index` includes a mock data fallback to ensure the UI can be demonstrated. Themes are displayed as responsive cards with screenshots and activation status.
- **View Logic**: 
    - `index.hbs`: Iterates over themes, highlighting the active one with a border and badge. Provides "Activate" buttons for inactive themes.
    - `upload.hbs`: Multi-part form for `.zip` file uploads.

### Theme Controller (`controllers/theme_controller.js`)
Handles the orchestration of theme-related data:
- `index`: Attempts to fetch themes from `api.themes.list`. If the backend returns an error (due to non-existence), it provides a set of high-quality mock themes.
- `create`: Processes theme uploads (using `multer` or similar in the router).
- `activate`: Triggers the backend activation endpoint.

## User Identity & Profile Management
The User Profile Management implementation bridges a gap where the backend provides session validation but lacks a dedicated "fetch me" endpoint that returns full user data.

### Identity Resolution Strategy (JWT Decoding)
To solve the challenge of identifying the currently logged-in user for profile fetching, Oxidly implements a **Frontend Identity Resolution** pattern in `controllers/user_controller.js`:
1. **Extraction**: The `auth_token` cookie is retrieved from the incoming request.
2. **Manual Decoding**: Since the `jsonwebtoken` dependency is not available, the controller manually parses the JWT's base64-encoded payload.
3. **Payload Mapping**: The `sub` field (which contains the `username` in the FreeRadical backend implementation) is extracted.
4. **Backend Fetch**: The extracted username is used to call the specific backend detail endpoint: `GET /v1/user/{username}`.

### Implementation Details
- **Controller (`user_controller.js`)**: Orchestrates the decode -> fetch -> render flow.
- **JWT Decoding Logic**: Uses `Buffer.from(payload, 'base64')` for Node.js compatibility after replacing URL-safe characters (`-` and `_`). This manual approach is used to avoid adding new heavy dependencies.
- **API Service**: Extended with a `users` namespace to handle `get` and `update` (mapping to `PUT /v1/user/{username}`).
- **Security**: Relies on the backend's `authenticate` middleware to verify the token during the specific detail call, ensuring the manual frontend decode is only used for identification, not authorization.

### Profile View (`views/users/profile.hbs`)
- **Visuals**: Uses the standard responsive pattern with Tailwind CSS.
- **Functionality**:
    - Displays `UUID` as read-only.
    - `Username/Email` is read-only (Primary Key).
    - `New Password` field for updates (left blank for no change).
    - Displays `2FA` status (Enabled/Disabled) using green/gray badges.
- **Feedback**: Includes standardized Handlebars blocks for `error` and `success` messages.

## Store Setup Wizard
The Store Setup Wizard provides a guided experience for initial commerce configuration, essential for transitioning a site into an e-commerce platform.

### Architectural Gap & Strategy
- **Gap Analysis**: Verification of `src/models/commerce_models.rs` revealed that while the backend supports `Products` and `Orders`, it lacks a centralized `Store` or `Settings` model for general commerce configuration (Industry, Currency, Business Address).
- **Frontend-First Approach**: Adopting an ECRSS (Eliminate, Combine, Reduce, Simplify, Standardize) strategy, Oxidly implements the wizard logic and captured data structure first.
- **Data Capture Flow**:
    1. **Store Details**: Name, Industry, Support Email.
    2. **Location**: Physical address for shipping and tax calculations.
    3. **Preferences**: Currency (default USD), Weight Units, Dimensions.

### Implementation Details
- **Controller (`store_controller.js`)**: Manages the multi-step state and handles final form submission.
- **Simulated Persistence**: In alignment with the ECRSS "Simplify" strategy, the `save` method currently logs the captured configuration and redirects with a success flag, establishing the UI flow before a dedicated backend settings endpoint is deployed.
- **View (`store/setup.hbs`)**: Uses a single-page multi-step interface with JavaScript-driven transitions. Step indicators dynamically update using icons (e.g., Lucide checkmarks) as the user progresses through Details, Address, and Preferences.

## Product Management UI
The Product Management UI allows merchants to manage their catalog through standard CRUD operations, interfacing with the FreeRadical Rust backend's commerce modules.

### Integration Details
- **Controller (`product_controller.js`)**:
    - **ID Handling**: Maps the URL ID to the backend integer `i64` type.
    - **UUID Generation**: Backend `NewProduct` struct requires a `uuid` string. The frontend generates this using `crypto.randomUUID()` during creation. For updates, the original UUID is preserved via a hidden field.
    - **Currency Logic**: Converts decimal inputs (dollars) to integers (cents) for the backend and vice versa for the display.
- **Views (`views/products/`)**:
    - `index.hbs`: Shows a responsive table of catalog items with price/stock metrics.
    - `form.hbs`: A unified interface for adding and updating products, including stock levels and SKUs.

## Order Management UI
The Order Management system provides oversight of the commerce lifecycle, moving from transaction processing to fulfillment.

### Data Architecture
- **Non-Relational Fetches**: Due to the backend's current decoupled schema (joinables commented out), the `get_order` handler performs sequential queries to resolve product names for `OrderItemWithProduct` objects.
- **State Machine**: Orders follow a rigid status flow: `pending` -> `processing` -> `completed`/`cancelled`. 
- **Payment Linkage**: Payment processing is handled by linking provider IDs (e.g., Stripe Payment Intents) directly to the specific order `i64` identifier.

### Implementation Strategy
- **Controller (`order_controller.js`)**: Focuses on presenting receipt-style views and facilitating status transitions for merchants.
- **Dynamic Formatting**: Similar to the Product UI, the controller maps `total_amount_cents` and line-item prices to human-readable currency strings using `(cents / 100).toFixed(2)`.
- **Date Handling**: Converts ISO backend timestamps to localized strings (e.g., `toLocaleDateString()`) for merchant readability.
- **Views (`views/orders/`)**:
    - `index.hbs`: A dashboard table summarizing all merchant orders with status badges.
    - `details.hbs`: A full order breakdown including a line-item table, customer UUID, and a form to trigger status updates (`Update` via `POST /orders/:id/status`).

## Inventory Tracking
Inventory tracking in Oxidly is currently managed at the product level, providing merchants with a clear view of stock levels alongside their product catalog.

### Implementation Pattern
- **Direct Management**: Merchants update `inventory_count` directly through the Product Edit form.
- **Visibility**: Current stock levels are displayed as a primary metric in the product list view (`index.hbs`), using icons to represent package counts.
- **Data Model Integration**: Maps to the `stock_quantity` field in the backend `ProductVariant` or `Product` structs, though primary CRUD currently operates on the `Product` model for simplicity.

## Payment & Shipping Configuration
This module provides the central interface for configuring store-wide commerce parameters and managing payment gateway status.

### Implementation Details
- **Payment Provider Discovery**: Interacts with the backend's `PaymentHandlerRegistry` via the `/payments/providers` endpoint to dynamically list supported gateways (Stripe, PayPal, etc.).
- **Shipping Simulation**: Currently implements a frontend-only shipping configuration form (Flat Rate and Free Shipping Threshold) to establish the UI lifecycle while backend shipping calculation modules are in development.
- **Controller Logic**: `StoreController.settings` orchestrates the fetching of backend provider data and merges it with local simulated settings for display.
- **View (`views/store/settings.hbs`)**: Highlights active providers with status badges and provides a structured form for logistical preferences.




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
