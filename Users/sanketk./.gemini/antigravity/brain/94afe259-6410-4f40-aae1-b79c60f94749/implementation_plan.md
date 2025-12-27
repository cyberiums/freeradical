# Theme Management UI Implementation Plan

## Goal
Implement the Theme Management User Interface for Oxidly, allowing users to view available themes, upload new ones (UI flow), and activate a theme.

## User Review Required
None.

## Proposed Changes

### Oxidly Service Layer
#### [MODIFY] [services/api.js](file:///Users/sanketk./freeradical/oxidly/services/api.js)
-   Add `themes` namespace.
-   `list(req)`: Get available themes.
-   `upload(formData, req)`: Upload a theme zip.
-   `activate(id, req)`: Activate a theme.

### Oxidly Controllers
#### [NEW] [controllers/theme_controller.js](file:///Users/sanketk./freeradical/oxidly/controllers/theme_controller.js)
-   `index(req, res)`: List themes (Mock data if backend API fails/missing, or handling empty list).
-   `new(req, res)`: Render upload form.
-   `create(req, res)`: Handle upload.
-   `activate(req, res)`: Handle activation.

### Oxidly Views
#### [NEW] [views/themes/index.hbs](file:///Users/sanketk./freeradical/oxidly/views/themes/index.hbs)
-   Grid of theme cards (Screenshot placeholder, Title, Version).
-   "Active" badge on current theme.
-   "Activate" button on others.
-   "Upload New" button.

#### [NEW] [views/themes/upload.hbs](file:///Users/sanketk./freeradical/oxidly/views/themes/upload.hbs)
-   File input (accept .zip).
-   Submit button.

### Oxidly Routing
#### [MODIFY] [server.js](file:///Users/sanketk./freeradical/oxidly/server.js)
-   Register theme routes.

## Verification
-   Visit `/themes`.
-   Verify mock/real themes are listed.
-   Click "Upload".
-   Submit form.
-   Verify redirect.
