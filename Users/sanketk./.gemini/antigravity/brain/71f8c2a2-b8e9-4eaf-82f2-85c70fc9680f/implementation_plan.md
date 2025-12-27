# Oxidly Cloud Platform - Implementation Plan (Phase 1)

**Goal**: Build the foundation of Oxidly.com, the cloud frontend for FreeRadical CMS, using Node.js, Express, and Handlebars.

## User Review Required
> [!IMPORTANT]
> **Stack Selection**: I am proposing **Node.js + Express + Handlebars** for the Oxidly frontend, as it aligns with the "Handlebars templates" requirement and separates the cloud platform logic from the core Rust backend. The `admin` panel uses Vite/TS, so Node.js is already part of the ecosystem. Please confirm if you prefer a Rust-based frontend instead.

## Proposed Changes

### [oxidly]
#### [NEW] [package.json](file:///Users/sanketk./freeradical/oxidly/package.json)
- Initialize Node.js project.
- Dependencies: `express`, `express-handlebars`, `dotenv`, `axios` (for API calls), `cookie-parser`.
- Dev Dependencies: `nodemon`, `typescript` (optional, sticking to JS for speed unless requested), `tailwindcss` (as per rules).

#### [NEW] [server.js](file:///Users/sanketk./freeradical/oxidly/server.js)
- Entry point.
- Configure Express.
- Setup Handlebars engine.
- middleware for static files and body parsing.

#### [NEW] [views]
- `layouts/main.hbs`: Main HTML structure.
- `partials/header.hbs`: Navigation.
- `partials/sidebar.hbs`: Dashboard sidebar.
- `home.hbs`: Landing page.
- `auth/login.hbs`: Login form.
- `auth/signup.hbs`: Signup form.
- `dashboard/index.hbs`: Dashboard home.

#### [NEW] [public]
- `css/style.css`: Main styles (Tailwind output).
- `js/main.js`: Client-side logic.

## Verification Plan

### Automated Tests
- None for the initial setup.

### Manual Verification
1.  **Start Server**: Run `npm run dev` in `oxidly`.
2.  **Home Page**: Visit `http://localhost:3000` and verify the landing page loads.
3.  **Auth Pages**: Visit `/login` and `/signup` and verify fields exist.
4.  **Dashboard**: Visit `/dashboard` (mock auth for now) and verify layout/sidebar.

