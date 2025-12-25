# FreeRadical CMS Deployment Guide

## Production Deployment Options

### Option 1: Docker Compose (Recommended)

**Requirements**:
- Docker Engine 20.10+
- Docker Compose 2.0+
- 2GB RAM minimum
-4GB recommended

**Steps**:

1. **Clone and configure**:
```bash
git clone https://github.com/yourusername/freeradical.git
cd freeradical
cp .env.example .env
# Edit .env with production values
```

2. **Deploy**:
```bash
chmod +x scripts/deploy.sh
./scripts/deploy.sh
```

3. **Verify**:
```bash
curl http://localhost:8000/api/health
```

### Option 2: Manual Deployment

**Requirements**:
- Rust 1.75+
- Node.js 18+
- MySQL 8.0+
- Redis 7+

**Steps**:

1. **Build CMS**:
```bash
cargo build --release
```

2. **Run migrations**:
```bash
diesel migration run
```

3. **Build CLI**:
```bash
cd ../cli
cargo build --release
```

4. **Build Admin**:
```bash
cd admin
npm install
npm run build
```

5. **Start services**:
```bash
# CMS
./target/release/freeradical

# Admin (with static server)
cd admin/dist
python3 -m http.server 3000
```

### Option 3: Kubernetes

See `k8s/` directory for Kubernetes manifests.

---

## Environment Configuration

### Required Variables

```bash
DATABASE_URL=mysql://user:pass@host/db
JWT_SECRET=random-64-char-string
APP_BASE_URL=https://your-domain.com
```

### Optional Variables

See `.env.production.example` for all options.

---

## SSL/TLS Setup

### With Nginx

```nginx
server {
    listen 443 ssl http2;
    server_name your-domain.com;

    ssl_certificate /path/to/cert.pem;
    ssl_certificate_key /path/to/key.pem;

    location / {
        proxy_pass http://localhost:8000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### With Caddy

```
your-domain.com {
    reverse_proxy localhost:8000
}
```

---

## Backup & Restore

### Database Backup
```bash
mysqldump -u user -p freeradical > backup.sql
```

### Media Backup
```bash
tar -czf uploads.tar.gz uploads/
```

### Restore
```bash
mysql -u user -p freeradical < backup.sql
tar -xzf uploads.tar.gz
```

---

## Monitoring

### Health Check
```bash
curl http://localhost:8000/api/health
```

### Logs
```bash
# Docker
docker-compose logs -f cms

# Manual
journalctl -u freeradical -f
```

### Metrics
- Prometheus: Port 9090
- Grafana dashboards available in `monitoring/`

---

## Security Checklist

- [ ] Change default JWT_SECRET
- [ ] Use strong database passwords
- [ ] Enable HTTPS
- [ ] Configure CORS properly
- [ ] Set up firewall rules
- [ ] Regular security updates
- [ ] Database backups scheduled
- [ ] Monitor access logs

---

## Troubleshooting

### Port already in use
```bash
lsof -i :8000
kill -9 <PID>
```

### Database connection failed
- Verify DATABASE_URL
- Check MySQL is running
- Ensure user has permissions

### Migration errors
```bash
diesel migration redo
```

---

## Performance Tuning

### Database
- Enable query caching
- Index optimization
- Connection pooling (default: 10)

### Redis
- Set appropriate memory limits
- Configure eviction policy

### Application
- Adjust worker threads
- Enable compression
- CDN for static assets

---

## Support

- Documentation: https://docs.freeradical.dev
- Issues: GitHub Issues
- Discord: discord.gg/freeradical
