# pgvector Setup for FreeRadical CMS

## Quick Start

### For Docker/Ubuntu Server

Run the automated installation script:

```bash
# Make executable (if not already)
chmod +x scripts/setup-pgvector.sh

# Run as root or with sudo
sudo ./scripts/setup-pgvector.sh
```

This will:
1. ✅ Detect PostgreSQL version
2. ✅ Install build dependencies
3. ✅ Download and compile pgvector v0.5.1
4. ✅ Install extension system-wide
5. ✅ Verify installation

### For Docker Compose

Add to your `docker-compose.yml`:

```yaml
services:
  postgres:
    image: postgres:14
    # ... your existing config ...
    volumes:
      - ./scripts/setup-pgvector.sh:/docker-entrypoint-initdb.d/setup-pgvector.sh:ro
    # OR run manually after container starts:
    # docker exec -it container_name bash /path/to/setup-pgvector.sh
```

### For Dockerfile

Add to your Dockerfile:

```dockerfile
FROM postgres:14

# Install pgvector
COPY scripts/setup-pgvector.sh /tmp/
RUN chmod +x /tmp/setup-pgvector.sh && \
    /tmp/setup-pgvector.sh && \
    rm /tmp/setup-pgvector.sh
```

## After Installation

1. **Restart PostgreSQL**:
   ```bash
   # Ubuntu/Debian
   sudo systemctl restart postgresql
   
   # Docker
   docker restart container_name
   ```

2. **Run Migration**:
   ```bash
   diesel migration run
   ```

3. **Verify**:
   ```bash
   psql -d freeradical_dev -c "SELECT extname, extversion FROM pg_extension WHERE extname='vector';"
   ```

## Troubleshooting

### Script fails with "postgresql-server-dev not found"

```bash
# Find available PostgreSQL versions
apt-cache search postgresql-server-dev

# Install specific version
sudo apt-get install postgresql-server-dev-14
```

### Permission denied

Run with sudo:
```bash
sudo ./scripts/setup-pgvector.sh
```

### Extension still not available after installation

Restart PostgreSQL and check extension path:
```bash
sudo systemctl restart postgresql
pg_config --sharedir
ls $(pg_config --sharedir)/extension/vector*
```

---

*Installation script: `/scripts/setup-pgvector.sh`*
