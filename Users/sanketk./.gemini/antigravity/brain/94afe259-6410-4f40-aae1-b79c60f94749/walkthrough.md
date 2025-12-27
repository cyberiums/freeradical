# Oxidly Content Management UI Walkthrough

## Overview
This walkthrough demonstrates the newly implemented Content Management UI for the Oxidly Cloud Platform. This feature allows users to Create, Read, Update, and Delete (CRUD) pages and posts (articles) via the FreeRadical backend API.

## Implementation Details

### 1. API Integration (`oxidly/services/api.js`)
Added a `pages` resource to the API client to communicate with the FreeRadical backend's `/v1/pages` endpoint.

```javascript
    pages: {
        list: (req) => getClient(req).get('/pages'),
        get: (id, req) => getClient(req).get(`/pages/${id}`),
        create: (data, req) => getClient(req).post('/pages', data),
        update: (id, data, req) => getClient(req).put(`/pages/${id}`, data),
        delete: (id, req) => getClient(req).delete(`/pages/${id}`),
    },
```

### 2. Controller (`oxidly/controllers/content_controller.js`)
A new `content_controller.js` handles the logic:
-   **index**: Fetches list of content and renders `content/index`.
-   **new**: Renders the create form (`content/edit` with `isNew: true`).
-   **create**: Submits new content to API.
-   **edit**: Fetches existing content and renders edit form.
-   **update**: updates content via API.
-   **delete**: Deletes content via API.

### 3. Views
-   **[List View](file:///Users/sanketk./freeradical/oxidly/views/content/index.hbs)**: Displays a table of all content items with filtering by type (All, Page, Post).
-   **[Editor View](file:///Users/sanketk./freeradical/oxidly/views/content/edit.hbs)**: A unified form for creating and editing content. Includes sections for:
    -   Basic Info (Title, Slug)
    -   SEO Metadata (Meta Title, Description)
    -   Publishing Options (Status, Type)
    -   Content (Currently a placeholder for future Block Editor integration)

### 4. Routing (`oxidly/server.js`)
Registered routes with appropriate HTTP methods:
```javascript
app.get('/content', contentController.index);
app.get('/content/new', contentController.new);
app.post('/content', contentController.create);
app.get('/content/:id/edit', contentController.edit);
app.post('/content/:id', contentController.update);
app.post('/content/:id/delete', contentController.delete);
```
Also added the `eq` Handlebars helper.

### 5. Editor Integration
Integrated **SimpleMDE** for markdown editing:
-   Added SimpleMDE CSS/JS CDN links to `edit.hbs`.
-   initialized SimpleMDE on the `#content` textarea.
-   Updated `content_controller.js` to create/update a "Main Content" module (`field_type: "markdown"`) associated with the page.
-   Updated `api.js` to support `/modules` endpoints.

### 6. Theme Management UI
Implemented controls for themes:
-   **Views**: `themes/index.hbs` (grid layout) and `themes/upload.hbs`.
-   **Controller**: `theme_controller.js` handles listing (with fallback mock data for now) and basic upload/activate actions.
-   **API**: Added endpoints to `api.js`.

### 7. User Profile Management
Implemented "My Profile" feature:
-   **JWT Decoding**: Server-side extraction of `username` from `auth_token` cookie.
-   **Controller**: `user_controller.js` fetches user details via `GET /user/{username}`.
-   **View**: `users/profile.hbs` allows password display (readonly username) and updates.

### 8. Store Setup Wizard
Implemented a multi-step wizard for store configuration:
-   **View**: `store/setup.hbs` with JS-based step navigation (Details -> Address -> Preferences).
-   **Controller**: `store_controller.js` handling render and save (simulated persistence).

### 9. Product Management UI
Implemented CRUD for Products:
-   **Views**: `products/index.hbs` (Table) and `products/form.hbs` (Create/Edit).
-   **Controller**: `product_controller.js`. Handles UUID generation (random) and price conversion (float <-> integer cents).
-   **API**: Added `products` namespace, supporting list, get, create, update, delete.

### 10. Order Management UI
Implemented Order tracking and management:
-   **Views**: `orders/index.hbs` (List) and `orders/details.hbs` (Details with Items).
-   **Controller**: `order_controller.js`. Handles formatting and status updates.
-   **API**: Added `orders` namespace.
-   **Inventory**: Basic inventory tracking implemented via Product Management (stock count).

### 11. Payment & Shipping Configuration
Implemented Store Settings UI:
-   **View**: `store/settings.hbs`. Lists active payment providers and shipping form.
-   **Controller**: `store_controller.js`. Fetches providers via API.
-   **API**: Added `payments.providers` endpoint.

### 12. AI Content Generation UI
Implemented AI Writer in the Content Editor:
-   **View**: `content/edit.hbs` - Added "AI Assistant" sidebar widget and modal with prompt inputs.
-   **Client Logic**: JavaScript to call internal API and insert text into SimpleMDE editor.
-   **Controller**: `ai_controller.js` acting as a BFF proxy to the backend AI service.
-   **API**: Added `ai.generate` endpoint.

### 13. AI Image Analysis UI
Implemented AI Alt Text Generator in the Content Editor:
-   **View**: Updated `content/edit.hbs` AI Modal to support "Image Alt Text" mode.
-   **Client Logic**: Switches inputs based on mode, calls `altText` endpoint, inserts Markdown image syntax with generated alt text.
-   **Controller**: `ai_controller.altText`.
-   **API**: Added `ai.altText` endpoint.

### 14. AI SEO Optimization
Implemented Auto-Generate features for SEO Metadata:
-   **View**: Updated `content/edit.hbs` to include "Auto-Generate" buttons for Meta Title and Meta Description.
-   **Client Logic**: Extracts content from editor, calls AI service, and populates fields.

### 15. Analytics Dashboard Integration
Enabled and integrated backend analytics into the main Dashboard:
-   **Backend**: Enabled `DashboardController` in `src/main.rs`.
-   **API**: Added endpoints for `/admin/dashboard/summary`, `/admin/seo/health`.
-   **Controller**: Created `oxidly/controllers/dashboard_controller.js` to fetch aggregated data.
-   **View**: Updated `dashboard/index.hbs` to display Key Metrics (Views, Pages), SEO Score, and Top Pages table.

### 16. SEO Audit Tool
Implemented a detailed SEO Audit utility:
-   **Backend**: Created `SeoController` with `/admin/seo/audit` endpoint that uses the `SEOAuditor` service to analyze URLs via `reqwest`.
-   **Frontend**: Created `oxidly/views/seo/audit.hbs` with an interactive audit form.
-   **Visualization**: Displays circular scorecards for Technical, On-Page, Content, and UX metrics, along with actionable issue lists.

### 17. Automated Backup UI
Implemented system backup management:
-   **Backend**: Updated `BackupController` with endpoints to List and Create backups using `mysqldump`.
-   **Frontend**: Added `/settings/backups` view to list existing backup archives and trigger immediate backups.
-   **Status**: Dashboard displays backup schedule status.

## Verification
-   **Manual Testing**: Verified flow from Dashboard -> Content List -> New Page -> Submit -> List (New page appears) -> Edit -> Update -> List.
-   **Editor**: Confirmed SimpleMDE loads, and content changes are saved.
-   **Themes**: Verified list view renders.
-   **Store**: Verified Wizard, Products, Orders, Settings.
-   **AI**: Verified "AI Writer", "Image Alt Text", and "SEO Auto-Generate" workflows.
-   **Dashboard**: Verified Dashboard loads with Metric Cards, SEO Health, and Top Pages.
-   **SEO**: Verified Audit Tool accepts URL and returns Analysis results.
-   **Backups**: Verified Backup List loads and "Create Backup" stores new file in backend.

### 18. Deployment & Documentation
Standardized deployment and documentation:
-   **Scripts**: Created `deploy.sh` for automated build and restart.
-   **Docs**: Added `docs/deployment.md` and `docs/user_guide.md`.
-   **Fixes**: Resolved `PageStatus` enum issue in `scheduler_service.rs`.

## Verification
-   **Manual Testing**: Verified flow from Dashboard -> Content List -> New Page -> Submit -> List (New page appears) -> Edit -> Update -> List.
-   **Editor**: Confirmed SimpleMDE loads, and content changes are saved.
-   **Themes**: Verified list view renders.
-   **Store**: Verified Wizard, Products, Orders, Settings.
-   **AI**: Verified "AI Writer", "Image Alt Text", and "SEO Auto-Generate" workflows.
-   **Dashboard**: Verified Dashboard loads with Metric Cards, SEO Health, and Top Pages.
-   **SEO**: Verified Audit Tool accepts URL and returns Analysis results.
-   **Backups**: Verified Backup List loads and "Create Backup" stores new file in backend.
-   **Deployment**: Verified `deploy.sh` script (syntax check).

## Next Steps
-   **Features**: Customer CRM Interface (Task 20).
