# Oxidly Maintenance & Operations Guide

This guide covers the system maintenance features of the Oxidly platform, including the backup system and scheduled publishing services.

## 1. Automated Backup System
The Oxidly Backup System provides a secure way to archive the platform's database and state.

### Backend Architecture
- **Controllers**: `src/controllers/backup_controller.rs`
- **Services**: `src/services/backup_service.rs`
- **Implementation**:
    - Uses `mysqldump` to generate raw SQL dumps.
    - Compresses backups using `gzip` to save storage space.
    - Exposes endpoints for listing available backups and triggering new ones.

### Frontend Integration
- **Route**: `/settings/backups`
- **Controller**: `oxidly/controllers/backup_controller.js`
- **View**: `oxidly/views/settings/backups.hbs`
- **Features**: 
    - Real-time listing of backup archives from the server.
    - Manual "Create Backup Now" trigger for on-demand protection.

---

## 2. Scheduled Publishing Service
The platform includes a robust background task runner for automated content status transitions.

### Backend Implementation
- **Service**: `src/services/scheduler_service.rs`
- **Engine**: powered by `tokio_cron_scheduler`.
- **Logic**:
    - Runs every 1 minute.
    - **Auto-Publish**: Transitions pages from `Scheduled` to `Published` when the current time exceeds `publish_at`.
    - **Auto-Archive**: Transitions pages from `Published` to `Archived` when the current time exceeds `unpublish_at`.

### Configuration
The scheduler logic is centralized in the Rust backend to ensure consistency across all client interfaces. It utilizes the `PageStatus` enum (mapped to the database) for reliable state management.

---

## 3. Infrastructure & Deployment
The platform is designed for containerized reliability.

### Docker Configuration
- **Dockerfile**: Implements a multi-stage build.
    - **Builder Stage**: Uses `rustlang/rust:nightly` with Python dependencies for `pyo3` support.
    - **Runner Stage**: Uses `debian:sid-slim` for a minimal production footprint.
- **Docker Compose**: Orchestrates the multi-service environment:
    - `postgres`: Database (with `pgvector`).
    - `redis`: Caching layer.
    - `cms`: FreeRadical Rust backend.
    - `oxidly`: Node.js frontend wrapper. 
    - `admin`: Static administrative interface.
