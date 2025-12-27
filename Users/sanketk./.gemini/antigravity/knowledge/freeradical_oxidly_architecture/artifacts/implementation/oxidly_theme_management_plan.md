# Theme Management UI Implementation Plan

## Goal
Implement the Theme Management User Interface for Oxidly, allowing users to view available themes, upload new ones (UI flow), and activate a theme.

## Context & Constraints
The FreeRadical Rust backend currently lacks a dedicated `ThemeController` or explicit theme-management API endpoints (determined by scanning `src/controllers`). Themes are currently loaded statically from `./templates`.

Implementation will focus on the UI/UX flow in Oxidly, with API calls prepared for a future backend implementation.

## Proposed Changes

### Oxidly Service Layer
#### [MODIFY] [services/api.js]
- Add `themes` namespace.
- `list(req)`: Get available themes.
- `upload(formData, req)`: Upload a theme zip.
- `activate(id, req)`: Activate a theme.

### Oxidly Controllers
#### [NEW] [controllers/theme_controller.js]
- `index(req, res)`: List themes (Mock data if backend API fails/missing).
- `new(req, res)`: Render upload form.
- `create(req, res)`: Handle upload.
- `activate(req, res)`: Handle activation.

### Oxidly Views
#### [NEW] [views/themes/index.hbs]
- Grid of theme cards (Screenshot placeholder, Title, Version).
- "Active" badge on current theme.
- "Activate" button on others.
- "Upload New" button.

#### [NEW] [views/themes/upload.hbs]
- File input (accept .zip).
- Submit button.

### Oxidly Routing
#### [MODIFY] [server.js]
- Register theme routes: `/themes`, `/themes/new`, `/themes/:id/activate`, etc.

## Verification
1. Visit `/themes` and verify the UI rendering.
2. Test the navigation to "Upload Theme".
3. Verify form submission (even if backend returns 404/not implemented).
