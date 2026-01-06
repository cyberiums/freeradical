# Production Deployment - Quick Start

## 🎯 Architecture

**Local Development:**
- Uses `docker-compose.yml`
- Postgres runs in Docker container

**Production:**
- Uses `docker-compose.production.yml`
- Postgres via GCP Cloud SQL (managed)
- Auto-updates via Watchtower


---

## 🚀 Option 1: GCP Cloud SQL (Recommended for Production)

### Prerequisites
1. GCP account with Cloud SQL enabled
2. Cloud SQL instance created (see detailed guide below)

### Quick Deploy

```bash
# 1. Set up Cloud SQL (one-time)
# See: oxidly/docs/GCP_CLOUD_SQL_SETUP.md for detailed instructions

# 2. Configure environment
cp .env.production.template .env.production
# Edit .env.production with your Cloud SQL details

# 3. Deploy
docker-compose -f docker-compose.production.yml up -d
```

**📚 Complete Setup Guide:** See `oxidly/docs/GCP_CLOUD_SQL_SETUP.md`

---

## 🚀 Option 2: Self-Hosted Postgres (Alternative)

If you want to use a self-hosted Postgres server instead of Cloud SQL:

```bash
# Use the local docker-compose with external DB
docker-compose up -d

# Or modify DATABASE_URL to point to your Postgres server
DATABASE_URL=postgres://user:pass@your-postgres-server:5432/db
```

---

## 🔧 What Gets Auto-Updated

Watchtower monitors and auto-updates:
- ✅ FreeRadical CMS (`ghcr.io/cyberiums/freeradical/cms:latest`)
- ✅ Oxidly (`ghcr.io/cyberiums/freeradical/oxidly:latest`)
- ❌ Database (Cloud SQL - managed by GCP)
- ❌ Redis (stable image, rarely needs updates)

## 📊 Monitor Auto-Updates

```bash
# Watch Watchtower logs
docker logs -f freeradical_watchtower

# Check container status
docker-compose -f docker-compose.production.yml ps

# View update history
docker logs freeradical_watchtower | grep "Updated"
```

## 🔔 Get Notifications (Optional)

Uncomment these lines in `docker-compose.production.yml`:
```yaml
- WATCHTOWER_NOTIFICATIONS=slack
- WATCHTOWER_NOTIFICATION_SLACK_HOOK_URL=${SLACK_WEBHOOK_URL}
```

Then add to your `.env`:
```bash
SLACK_WEBHOOK_URL=https://hooks.slack.com/services/YOUR/WEBHOOK/URL
```

## 📚 More Options

See **[AUTO_UPDATE_GUIDE.md](oxidly/docs/AUTO_UPDATE_GUIDE.md)** for:
- Webhook-based deployment
- SSH deployment from GitHub Actions
- Cloud provider native solutions
- Kubernetes auto-updates
