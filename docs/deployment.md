# Deployment Guide

## Prerequisites
-   **Docker Engine** (v20.10+)
-   **Docker Compose** (v2.0+)
-   **Git**

## Quick Start
To deploy the Oxidly Cloud Platform (CMS + Frontend + Database):

```bash
chmod +x deploy.sh
./deploy.sh
```

## Configuration
Ensure your `.env` file is configured with secure passwords for production:

```ini
POSTGRES_PASSWORD=secure_password
JWT_SECRET=long_random_string
APP_BASE_URL=https://yourdomain.com
```

## Architecture
The deployment consists of 5 containers:
1.  **freeradical_cms**: Rust Backend (Port 8000)
2.  **freeradical_oxidly**: Node.js Frontend (Port 3000)
3.  **freeradical_postgres**: PostgreSQL Database (Port 5432)
4.  **freeradical_redis**: Redis Cache (Port 6379)
5.  **freeradical_admin**: Legacy Admin (Optional)

## Updating
Run `./deploy.sh` to pull the latest changes from git, rebuild containers, and restart the stack.

## Troubleshooting
-   **Database Connection Failed**: Ensure `postgres` container is healthy (`docker ps`). Check logs: `docker logs freeradical_postgres`.
-   **Oxidly 502/Connection Refused**: Ensure `APP_API_URL` in `oxidly` points to the correct backend host (usually `cms` or internal Docker DNS).
