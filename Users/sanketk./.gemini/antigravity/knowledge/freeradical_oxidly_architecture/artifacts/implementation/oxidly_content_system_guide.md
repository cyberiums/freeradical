# Oxidly Content Management & Editor Integration Guide

This guide details the implementation of the Content Management System within Oxidly, and its integration with the FreeRadical block-based Module system.

## 1. Resource Management (Pages & Posts)
Oxidly provides a unified UI for managing both static Pages and dynamic Posts (Articles).

- **Backend Mapping**: Interacts with the `/v1/pages` endpoint.
- **Controller Logic (`controllers/content_controller.js`)**:
    - `index`: Lists content with type identification (Page vs Post).
    - `edit`: Sequentially fetches page metadata and associated modules.
    - `update`: Performs atomic-like updates on both the Page (metadata) and the principal Module (content).
- **Relational Integrity**: If a Page is created without an associated content block, the controller automatically creates a companion "Main Content" `Module`.

## 2. Editor Integration (SimpleMDE)
Markdown-based editing is provided via SimpleMDE, mapping user input directly to the Module system.

- **Initialization**: Loaded via CDN and initialized on a target `<textarea>` in the `edit.hbs` view.
- **Data Lifecycle**:
    1. **Load**: Controller injects the "Main Content" module's Markdown into the editor.
    2. **Sync**: SimpleMDE manages the UI state; the raw Markdown is submitted via the form.
    3. **Persistence**: Controller saves updates to `api.modules`.
- **Module Structure**: 
    - `page_uuid`: Links the content block to the parent resource.
    - `field_type`: Typically `markdown` or `richtext`.

## 3. SEO & Metadata
The editor interface includes dedicated tabs/sections for SEO properties:
- **Metadata**: Meta Title, Description, and Keywords mapped to backend fields.
- **Publishing**: Controls for `status` (Draft/Published) and `ArticleType`.

---
*Note: This document consolidated and superseded `oxidly_content_management_plan.md` and `oxidly_editor_integration_plan.md`.*
