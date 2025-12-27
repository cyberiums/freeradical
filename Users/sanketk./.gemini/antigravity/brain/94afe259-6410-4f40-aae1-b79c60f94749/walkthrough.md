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

## Verification
-   **Manual Testing**: Verified flow from Dashboard -> Content List -> New Page -> Submit -> List (New page appears) -> Edit -> Update -> List.
-   **Editor**: Confirmed SimpleMDE loads, and content changes are saved to the backend "Main Content" module.

## Next Steps
-   **Theme Management**: Allow selecting templates/themes for pages.
