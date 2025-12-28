# FreeRadical CMS & Oxidly Cloud Platform - Comprehensive Architecture

> **Version**: 2.0.0 (Release Candidate)
> **Backend**: FreeRadical v1.7.0 (Rust)
> **Frontend**: Oxidly v1.0.0 (Node.js)
> **Date**: December 28, 2025

## 1. Executive Summary

FreeRadical is a high-performance, **hybrid-architecture** platform designed to solve the "WordPress performance problem" while delivering modern AI capabilities. It consists of two distinct layers:
1.  **FreeRadical Backend (v1.7.0)**: A monolithic, ultra-high-performance headless CMS written in Rust. It powers the core logic, database interactions, AI processing, and API delivery.
2.  **Oxidly Cloud Platform (v1.0.0)**: A multi-tenant SaaS frontend written in Node.js/Express. It provides the administrative UI, site management, and customer-facing interfaces, consuming the FreeRadical backend via API.

---

## 2. High-Level Architecture

The system follows a **Headless CMS** pattern where the backend is strictly an API provider, and the frontend is a consumer.

```mermaid
graph TD
    User[End User / Admin] -->|HTTPS| Oxidly[Oxidly Frontend (Node.js)]
    Oxidly -->|REST API (JSON)| FreeRadical[FreeRadical Backend (Rust)]
    
    subgraph "FreeRadical Backend"
        API[Actix-Web API Layer]
        Core[CMS Logic Core]
        AI[AI/MCP Engine]
        SEO[SEO/AEO Engine]
        Commerce[Commerce Engine]
        
        API --> Core
        Core --> AI
        Core --> SEO
        Core --> Commerce
    end
    
    subgraph "Data Layer"
        Redis[(Redis Cache)]
        DB[(MySQL / PostgreSQL)]
    end
    
    FreeRadical -->|Read/Write| DB
    FreeRadical -->|Cache| Redis
```

---

## 3. Core Components

### 3.1. FreeRadical Backend (Rust)
**Stack**: Rust, Actix-Web, Diesel ORM, Tokio Async Runtime.

*   **Performance**: ~6ms response time (9x faster than WordPress), 30MB binary size.
*   **Modules**:
    *   **CMS Core**: Content management, Revision history, Media handling.
    *   **AI/MCP Engine**:
        *   **Provider Abstraction**: Pluggable support for OpenAI, Anthropic, Gemini.
        *   **Security Scoping**: Granular JWT-based access (`ReadPublic`, `ReadOwn`, `WriteOwn`). See [AI Scoping](./docs/AI_MCP_SCOPING.md).
        *   **Orchestrator**: Multi-step workflow execution (e.g., "Write a blog post" -> Outline -> Draft -> Image Gen -> SEO Optimize).
    *   **SEO/AEO Engine**:
        *   **Generative Engine Optimization (GEO)**: Optimizes content for AI search (Perplexity, SearchGPT).
        *   **Automated Metadata**: Generates JSON-LD Schema (Article, Product, FAQ) automatically.
    *   **Commerce Engine**:
        *   **Pluggable Payments**: Trait-based system (`PaymentHandler`) supporting Stripe, PayPal, Square.
        *   **Inventory**: Atomic tracking of stock levels.

### 3.2. Oxidly Frontend (Node.js)
**Stack**: Node.js, Express, Handlebars (SSR), TailwindCSS.

*   **Role**: The "Face" of the platform.
*   **Architecture**:
    *   **Server-Side Rendering (SSR)**: Uses Handlebars for fast First Contentful Paint (FCP) and SEO-friendly admin pages.
    *   **Service Layer**: `api.js` acts as a typed client for the FreeRadical Backend.
    *   **Controllers**: Isolated logic for Site Management, Auth, and Commerce.
*   **Key Features**:
    *   **Site Management**: Create/Manage subdomains (`*.oxidly.com`) and Custom Domains (CNAME).
    *   **Dashboard**: Analytics visualization.
    *   **Team Management**: RBAC (Owner, Admin, Editor) and Member Invites.
    *   **Commerce UI**: Product Catalog, Manual Order Entry, Inventory.
    *   **Settings**: SSO Configuration (Stub), Profile.

---

## 4. Data Architecture

The system is database-agnostic, supporting both MySQL and PostgreSQL via feature flags.

### 4.1. Database Schema (Simplified)
*   **Users**: Identity and Authentication (common across sites).
*   **Sites**: Multi-tenancy isolation.
*   **Pages/Posts**: Polymorphic content tables with `jsonb` attributes.
*   **AuditLogs**: Immutable record of all actions.
*   **PaymentIntents**: Tracks transaction lifecycle.

### 4.2. Migration Strategy
All schema changes are managed via `Diesel CLI`.
*   **MySQL**: `migrations/` - Native SQL.
*   **Postgres**: `migrations_postgres/` - Optimized data types (e.g., `JSONB` vs `TEXT`).
*   **Validation**: CI pipeline verifies schema compatibility with both engines.

---

## 5. Security Architecture

### 5.1. Authentication
*   **JWT (JSON Web Tokens)**: Stateless authentication.
*   **Context Scoping**:
    *   `guest`: Public read-only.
    *   `user`: Read/Write own data.
    *   `admin`: System-wide access.

### 5.2. AI Security & Privacy
*   **Data Isolation**: AI context is strictly bounded by user permissions. A user cannot generate embeddings or search content they do not own.
*   **Key Management**: AI Provider keys (OpenAI, etc.) are encrypted at rest and never exposed to the frontend.

---

## 6. Infrastructure & Deployment

### 6.1. Docker Stack
*   **Backend**: Multi-stage build (Alpine Linux base), resulting in a <50MB image.
*   **Frontend**: Node.js LTS image.
*   **Database**: Official MySQL 8.0 / Postgres 15 images.
*   **Cache**: Redis 7.0 for session and query caching.

### 6.2. Scalability
*   **Stateless Backend**: FreeRadical can scale horizontally behind a load balancer (Nginx/Traefik).
*   **Read Replicas**: Database configuration supports Read/Write splitting for high-traffic deployments.

---

## 7. Roadmap & Capabilities

| Feature Domain | Current Status (v1.7) | Next Steps (v2.0+) |
| :--- | :--- | :--- |
| **CMS Performance** | ✅ 9x faster than WP | Edge Caching integration |
| **AI Capabilities** | ✅ Full MCP Agent support | Voice-controlled Admin UI |
| **E-commerce** | ✅ Stripe/PayPal/Square | Subscription Billing |
| **Frontend/UI** | 🟡 Basic Dashboard | Full Drag-and-Drop Builder |
| **Multi-tenancy** | 🟡 Database Ready | Org/Team Management UI |

---

## 8. Reference Documentation
*   [Feature Gap Analysis](./docs/roadmaps/FEATURE-GAP-ANALYSIS.md)
*   [AI Scoping & Security](./docs/AI_MCP_SCOPING.md)
*   [Database Config](./docs/databases.md)
*   [SEO Readiness](./docs/SEO-readiness.md)
