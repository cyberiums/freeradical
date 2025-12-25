# HTTPS Setup Guide for FreeRadical CMS

## Overview

FreeRadical CMS currently runs on HTTP (port 8000) by default. This guide covers three approaches to enable HTTPS for production deployments.

## Current Status

**Default Configuration:**
- Protocol: HTTP only
- Port: 8000
- Binding: `0.0.0.0:8000`
- SSL/TLS: Not configured

## Option 1: Reverse Proxy with Caddy (✅ Recommended)

**Best for:** Production deployments, automatic SSL management

### Why Caddy?
- Automatic HTTPS with Let's Encrypt
- Zero configuration SSL certificates
- Automatic certificate renewal
- Simple configuration

### Setup

1. **Install Caddy**
```bash
# Ubuntu/Debian
sudo apt install -y debian-keyring debian-archive-keyring apt-transport-https
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/caddy-stable.list
sudo apt update
sudo apt install caddy
```

2. **Create Caddyfile**
```caddyfile
# /etc/caddy/Caddyfile
your-domain.com {
    reverse_proxy localhost:8000
    
    # Optional: Security headers
    header {
        Strict-Transport-Security "max-age=31536000;"
        X-Content-Type-Options "nosniff"
        X-Frame-Options "DENY"
        Referrer-Policy "no-referrer-when-downgrade"
    }
    
    # Optional: Compression
    encode gzip
    
    # Optional: Logging
    log {
        output file /var/log/caddy/access.log
    }
}
```

3. **Start Caddy**
```bash
sudo systemctl enable caddy
sudo systemctl start caddy
sudo systemctl status caddy
```

**That's it!** Caddy automatically obtains and renews SSL certificates.

---

## Option 2: Reverse Proxy with Nginx + Certbot

**Best for:** Advanced configurations, integration with existing Nginx setups

### Setup

1. **Install Nginx and Certbot**
```bash
sudo apt update
sudo apt install nginx certbot python3-certbot-nginx
```

2. **Configure Nginx**
```nginx
# /etc/nginx/sites-available/freeradical
server {
    listen 80;
    server_name your-domain.com;
    
    location / {
        proxy_pass http://localhost:8000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }
}
```

3. **Enable Site**
```bash
sudo ln -s /etc/nginx/sites-available/freeradical /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

4. **Obtain SSL Certificate**
```bash
sudo certbot --nginx -d your-domain.com
```

5. **Auto-Renewal** (already configured by certbot)
```bash
# Test renewal
sudo certbot renew --dry-run
```

---

## Option 3: Native Actix-Web HTTPS

**Best for:** Direct SSL without reverse proxy, testing environments

### Prerequisites

Add to `Cargo.toml`:
```toml
[dependencies]
openssl = "0.10"
actix-web = { version = "4", features = ["openssl"] }
```

### Implementation

Update `src/main.rs`:

```rust
use actix_web::HttpServer;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ... existing setup code ...
    
    // SSL configuration
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file("certs/key.pem", SslFiletype::PEM)?;
    builder.set_certificate_chain_file("certs/cert.pem")?;
    
    let http_server = HttpServer::new(move || {
        // ... existing app configuration ...
    })
    .bind_openssl(("0.0.0.0", 8443), builder)?  // HTTPS on port 8443
    .workers(2)
    .run();
    
    println!("🚀 Server running on https://0.0.0.0:8443 🚀");
    
    http_server.await
}
```

### Generate Self-Signed Certificate (Development)

```bash
mkdir -p certs
openssl req -x509 -newkey rsa:4096 -nodes \
  -keyout certs/key.pem \
  -out certs/cert.pem \
  -days 365 \
  -subj "/CN=localhost"
```

### Production Certificates

For production, use Let's Encrypt:
```bash
sudo certbot certonly --standalone -d your-domain.com
# Certificates will be in /etc/letsencrypt/live/your-domain.com/
```

Update paths in code:
```rust
builder.set_private_key_file("/etc/letsencrypt/live/your-domain.com/privkey.pem", SslFiletype::PEM)?;
builder.set_certificate_chain_file("/etc/letsencrypt/live/your-domain.com/fullchain.pem")?;
```

---

## Option 4: Docker with HTTPS

**Best for:** Containerized deployments

### Update `docker-compose.yml`

```yaml
version: '3.8'

services:
  cms:
    # ... existing cms configuration ...
  
  caddy:
    image: caddy:latest
    container_name: freeradical_caddy
    restart: unless-stopped
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./Caddyfile:/etc/caddy/Caddyfile
      - caddy_data:/data
      - caddy_config:/config
    depends_on:
      - cms
    networks:
      - default

volumes:
  caddy_data:
  caddy_config:
```

### Create `Caddyfile`

```caddyfile
{
    email admin@your-domain.com
}

your-domain.com {
    reverse_proxy cms:8000
    encode gzip
}
```

### Deploy

```bash
docker-compose up -d
```

---

## Comparison Matrix

| Feature | Caddy | Nginx | Native Actix | Docker |
|---------|-------|-------|--------------|--------|
| **Auto SSL** | ✅ Yes | ❌ Manual | ❌ Manual | ✅ Yes (Caddy) |
| **Setup Time** | ⚡ 5 min | ⏱️ 15 min | ⏱️ 20 min | ⚡ 10 min |
| **Complexity** | ⭐ Easy | ⭐⭐ Medium | ⭐⭐⭐ Advanced | ⭐⭐ Medium |
| **Performance** | 🚀 Excellent | 🚀 Excellent | 🚀 Excellent | 🚀 Excellent |
| **Best For** | Production | Advanced Config | Direct SSL | Containers |

---

## Security Recommendations

### 1. Security Headers

Add these headers via your reverse proxy:

```nginx
# Nginx
add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
add_header X-Frame-Options "SAMEORIGIN" always;
add_header X-Content-Type-Options "nosniff" always;
add_header X-XSS-Protection "1; mode=block" always;
```

```caddyfile
# Caddy (in your domain block)
header {
    Strict-Transport-Security "max-age=31536000; includeSubDomains"
    X-Frame-Options "SAMEORIGIN"
    X-Content-Type-Options "nosniff"
    X-XSS-Protection "1; mode=block"
}
```

### 2. HTTP to HTTPS Redirect

**Caddy:** Automatic

**Nginx:**
```nginx
server {
    listen 80;
    server_name your-domain.com;
    return 301 https://$host$request_uri;
}
```

### 3. SSL/TLS Best Practices

- Use TLS 1.2 or higher only
- Disable weak ciphers
- Enable Perfect Forward Secrecy (PFS)
- Use OCSP stapling
- Regular certificate renewal monitoring

### 4. Firewall Configuration

```bash
# Allow HTTPS
sudo ufw allow 443/tcp

# Allow HTTP (for Let's Encrypt challenges)
sudo ufw allow 80/tcp

# Deny direct access to CMS port (if using reverse proxy)
sudo ufw deny 8000/tcp
```

---

## Testing HTTPS Setup

### 1. SSL Certificate Check

```bash
# Check certificate details
openssl s_client -connect your-domain.com:443 -servername your-domain.com

# Check expiration
echo | openssl s_client -connect your-domain.com:443 2>/dev/null | openssl x509 -noout -dates
```

### 2. SSL Labs Test

Visit: https://www.ssllabs.com/ssltest/analyze.html?d=your-domain.com

Aim for **A+ rating**

### 3. Security Headers

```bash
curl -I https://your-domain.com
```

---

## Troubleshooting

### Certificate Not Trusted

**Issue:** Browser shows "Not Secure"
**Solution:** 
- Ensure using valid CA certificate (not self-signed in production)
- Check certificate chain includes intermediate certificates
- Verify domain matches certificate CN/SAN

### Port Already in Use

**Issue:** `Address already in use`
**Solution:**
```bash
# Find process using port 443
sudo lsof -i :443
# Kill if needed
sudo kill -9 <PID>
```

### Certbot Fails

**Issue:** Certificate issuance fails
**Solution:**
- Verify domain DNS points to your server
- Ensure port 80 is accessible (required for HTTP-01 challenge)
- Check firewall rules
- Try different challenge method: `certbot --nginx --preferred-challenges dns`

### Mixed Content Warnings

**Issue:** HTTPS page loads HTTP resources
**Solution:**
- Update all asset URLs to use HTTPS
- Use protocol-relative URLs: `//cdn.example.com/script.js`
- Add `Content-Security-Policy` header to enforce HTTPS

---

## Monitoring & Maintenance

### Certificate Expiration Monitoring

```bash
# Add to cron (daily check)
0 0 * * * certbot renew --quiet && systemctl reload nginx
```

### Health Check Script

```bash
#!/bin/bash
# check-ssl.sh

DOMAIN="your-domain.com"
EXPIRY=$(echo | openssl s_client -connect $DOMAIN:443 -servername $DOMAIN 2>/dev/null | \
         openssl x509 -noout -enddate | cut -d= -f2)

echo "SSL certificate for $DOMAIN expires: $EXPIRY"

# Alert if expiring in 30 days
EXPIRY_EPOCH=$(date -d "$EXPIRY" +%s)
NOW_EPOCH=$(date +%s)
DAYS_LEFT=$(( ($EXPIRY_EPOCH - $NOW_EPOCH) / 86400 ))

if [ $DAYS_LEFT -lt 30 ]; then
    echo "WARNING: Certificate expires in $DAYS_LEFT days!"
fi
```

---

## Performance Optimization

### HTTP/2 Support

**Caddy:** Enabled by default

**Nginx:** Add to server block
```nginx
listen 443 ssl http2;
```

### Connection Pooling

Increase worker connections for high traffic:

**Nginx:**
```nginx
events {
    worker_connections 4096;
}
```

### Caching Static Assets

```nginx
location ~* \.(jpg|jpeg|png|gif|ico|css|js)$ {
    expires 1y;
    add_header Cache-Control "public, immutable";
}
```

---

## Production Checklist

- [ ] Valid SSL certificate from trusted CA
- [ ] HTTP to HTTPS redirect configured
- [ ] Security headers implemented
- [ ] SSL Labs A+ rating achieved
- [ ] Firewall rules configured
- [ ] Auto-renewal setup and tested
- [ ] Monitoring alerts configured
- [ ] Backup certificates stored securely
- [ ] Documentation updated with domain/paths

---

## Additional Resources

- [Caddy Documentation](https://caddyserver.com/docs/)
- [Nginx SSL Configuration](https://nginx.org/en/docs/http/configuring_https_servers.html)
- [Let's Encrypt Documentation](https://letsencrypt.org/docs/)
- [Mozilla SSL Configuration Generator](https://ssl-config.mozilla.org/)
- [Actix-Web OpenSSL Guide](https://actix.rs/docs/server/)

---

**Need Help?** Check the [FreeRadical CMS Wiki](https://github.com/cyberiums/freeradical/wiki) or open an issue on GitHub.
