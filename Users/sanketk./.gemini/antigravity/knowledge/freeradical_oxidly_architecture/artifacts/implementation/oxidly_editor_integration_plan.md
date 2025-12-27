# Oxidly Editor Integration Implementation Plan

## Goal
Integrate a Markdown editor (SimpleMDE) into the Oxidly Content Management UI to allow users to write and save content for pages and posts, mapping the output to the FreeRadical Module system.

## Proposed Changes

### 1. Frontend Integration
**File**: `oxidly/views/content/edit.hbs`
- Load SimpleMDE via CDN (CSS and JS).
- Initialize the editor on the primary content textarea.
- Add a script to sync editor state with form submission.

### 2. API Service Expansion
**File**: `oxidly/services/api.js`
- **Method**: `pages.getModules(id, req)` - Fetch content modules for a page.
- **Method**: `modules.create(data, req)` - Create a new content block.
- **Method**: `modules.update(id, data, req)` - Update existing blocks.

### 3. Controller Logic
**File**: `oxidly/controllers/content_controller.js`
- **Create**:
    1. Create `Page` via API.
    2. On success, create a default "Main Content" `Module` associated with the new `page_uuid`.
- **Edit**:
    1. Fetch `Page`.
    2. Fetch Page's modules via `api.pages.getModules`.
    3. Identify the principal module and pass its content to the Handlebars view.
- **Update**:
    1. Update `Page` metadata.
    2. Update the corresponding `Module` content.

## Backend Context
The Rust backend defines a `Module` with:
- `uuid`: Identifier.
- `page_uuid`: Foreign key.
- `title`: Block name (e.g., "Main Content").
- `content`: The raw content string (Markdown/HTML).
- `field_type`: Set to `richtext` or `markdown` for editor compatibility.

## Verification
- Create a post with rich content.
- Verify the content appears in the list/preview.
- Edit the post and ensure the editor loads with existing content.
- Modify and verify the update persists in the database (via modules table).
