# Oxidly Dashboard & Analytics Guide

This guide details the implementation of the main administrative dashboard and its integration with the FreeRadical analytics engine.

## 1. Overview
The Oxidly Dashboard provides a centralized view of site performance, SEO health, and top-performing content. It uses a **BFF (Backend For Frontend)** pattern to aggregate data from multiple backend services into a single UI view.

## 2. Backend Architecture
The analytics data is served by the Rust backend's `DashboardController`.

- **Controller**: `src/controllers/dashboard_controller.rs`
- **Service**: `src/services/analytics_service.rs`
- **Endpoints**:
    - `GET /admin/dashboard/summary`: High-level metrics (Total Pages, Views Today/Week/All Time).
    - `GET /admin/analytics/summary`: Detailed view/visitor data.
    - `GET /admin/seo/health`: Site-wide SEO audit (Score, missing meta descriptions, etc.).
    - `GET /admin/analytics/pages`: List of top-performing URLs.

## 3. Frontend Integration (BFF)
The Oxidly Node.js server proxies these requests and aggregates data for server-side rendering.

### API Client
The `oxidly/services/api.js` includes a `dashboard` resource:
```javascript
dashboard: {
    summary: (req) => getClient(req).get('/admin/dashboard/summary'),
    analytics: (req) => getClient(req).get('/admin/analytics/summary'),
    seoHealth: (req) => getClient(req).get('/admin/seo/health'),
    topPages: (req) => getClient(req).get('/admin/analytics/pages'),
}
```

### Dashboard Controller
Located at `oxidly/controllers/dashboard_controller.js`, it fetches data in parallel to minimize load times:
- Uses `Promise.all` to fetch Summary, SEO Health, and Top Pages.
- Handles error states with default empty values to ensure the dashboard remains accessible.

## 4. UI Components
The dashboard layout (`oxidly/views/dashboard/index.hbs`) is built with Tailwind CSS and Lucide icons.

### Key Metrics Row
Displays four primary cards:
1. **Views Today**: Real-time traffic.
2. **Views This Week**: Short-term growth.
3. **Total Pages**: Scale of the site.
4. **SEO Score**: Site-wide health check (0-100).

### Top Performing Content
A table listing the most viewed URLs on the site, allowing administrators to identify their best content.

## 5. On-Demand SEO Audit Tool
A dedicated utility for deep-diving into individual URL performance.

- **Backend Logic**:
    - **Controller**: `src/controllers/seo_controller.rs`.
    - **Service**: `src/services/seo_auditor.rs`.
    - **Method**: Fetches target URL content via `reqwest` and performs a 20+ point analysis.
- **Frontend Interaction**:
    - **View**: `oxidly/views/seo/audit.hbs`.
    - **AJAX Flow**: Users input a URL -> Client calls Node.js `/seo/audit` -> Proxies to Rust backend -> Renders categorized scorecards (Technical, On-Page, Content, UX) and prioritized issues list.

---
*Note: This dashboard is the primary entry point for site administrators and integrates with the SEO, AI, and Analytics backend services.*
