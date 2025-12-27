# FreeRadical CMS & Oxidly Cloud Platform

## Overview
FreeRadical CMS is a high-performance content management system built with Rust, designed as a 9x faster alternative to WordPress. The current version (v1.7.0) features a complete backend with AI/MCP automation, advanced SEO/AEO/GEO tools, and an extensible e-commerce engine.

Oxidly.com is the forthcoming cloud/SaaS frontend platform (v2.0.0) that will expose these features through a multi-tenant Handlebars-based interface.

## System Components
- **Backend (Rust)**: High-performance core with modular services for AI, SEO, and Commerce.
- **Oxidly Cloud Platform (Node.js)**: SaaS management layer for signup, billing, and site orchestration.
- **Public Frontend (Handlebars)**: High-speed SSR/Static rendering for SEO-critical site content (Rust-driven).
- **Admin Dashboard (React/Vite)**: Modern SPA for merchants to manage individual sites (located in `/admin`).
- **Database**: Relational DB (PostgreSQL/MySQL) with Diesel ORM.
- **Cache**: Redis for high-speed performance monitoring and caching.

## Detailed Architecture
For a deep dive into the platform's multi-layered design, refer to the [Comprehensive Architecture](./architecture/comprehensive_architecture.md) document.

## Current Readiness
As of v1.7.0, all backend modules are 100% complete and production-ready. The project is currently transitioning to the "Oxidly Cloud" phase to build the enterprise-grade management UI.
