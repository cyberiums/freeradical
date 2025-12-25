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
    
    # Security headers
    header {
        Strict-Transport-Security "max-age=31536000;"
        X-Content-Type-Options "nosniff"
        X-Frame-Options "DENY"
        Referrer-Policy "no-referrer-when-downgrade"
    }
    
    encode gzip
}
```

3. **Start Caddy**
```bash
sudo systemctl enable caddy
sudo systemctl start caddy
```

**That's it!** Caddy automatically obtains and renews SSL certificates.

---

## Option 2: Reverse Proxy with Nginx + Certbot

**Best for:** Advanced configurations, existing Nginx setups

### Setup

1. **Install Nginx and Certbot**
```bash
sudo apt update
sudo apt install nginx certbot python3-certbot-nginx
```

2. **Configure Nginx** (`/etc/nginx/sites-available/freeradical`)
```nginx
server {
    listen 80;
    server_name your-domain.com;
    
    location / {
        proxy_pass http://localhost:8000;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

3. **Enable Site and Get Certificate**
```bash
sudo ln -s /etc/nginx/sites-available/freeradical /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
sudo certbot --nginx -d your-domain.com
```

---

## Option 3: Native Actix-Web HTTPS

**Best for:** Direct SSL without reverse proxy

### Add Dependencies

```toml
# Cargo.toml
[dependencies]
openssl = "0.10"
actix-web = { version = "4", features = ["openssl"] }
```

### Update main.rs

```rust
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
builder.set_private_key_file("certs/key.pem", SslFiletype::PEM)?;
builder.set_certificate_chain_file("certs/cert.pem")?;

HttpServer::new(...)
    .bind_openssl(("0.0.0.0", 8443), builder)?
    .run()
    .await
```

### Generate Certificate

```bash
# Self-signed (development)
mkdir -p certs
openssl req -x509 -newkey rsa:4096 -nodes \
  -keyout certs/key.pem \
  -out certs/cert.pem \
  -days 365 \
  -subj "/CN=localhost"

# Production: Use Let's Encrypt
sudo certbot certonly --standalone -d your-domain.com
```

---

## Comparison Matrix

| Feature | Caddy | Nginx | Native Actix |
|---------|-------|-------|--------------|
| **Auto SSL** | ✅ Yes | ❌ Manual | ❌ Manual |
| **Setup Time** | ⚡ 5 min | ⏱️ 15 min | ⏱️ 20 min |
| **Complexity** | ⭐ Easy | ⭐⭐ Medium | ⭐⭐⭐ Advanced |
| **Best For** | Production | Advanced | Direct SSL |

---

## Security Recommendations

### 1. Security Headers (via reverse proxy)

```nginx
# Nginx
add_header Strict-Transport-Security "max-age=31536000" always;
add_header X-Frame-Options "SAMEORIGIN" always;
add_header X-Content-Type-Options "nosniff" always;
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

### 3. Firewall

```bash
sudo ufw allow 443/tcp
sudo ufw allow 80/tcp
sudo ufw deny 8000/tcp  # If using reverse proxy
```

---

## Testing

```bash
# Check certificate
openssl s_client -connect your-domain.com:443

# SSL Labs test
# Visit: https://www.ssllabs.com/ssltest/
```

---

## Troubleshooting

### Certificate Not Trusted
- Use valid CA certificate (not self-signed)
- Check certificate chain includes intermediates
- Verify domain matches certificate

### Port Already in Use
```bash
sudo lsof -i :443
sudo kill -9 <PID>
```

### Certbot Fails
- Verify DNS points to server
- Ensure port 80 accessible
- Check firewall rules

---

## Production Checklist

- [ ] Valid SSL certificate from trusted CA
- [ ] HTTP to HTTPS redirect configured
- [ ] Security headers implemented
- [ ] SSL Labs A+ rating
- [ ] Firewall configured
- [ ] Auto-renewal tested
- [ ] Monitoring configured

---

## Resources

- [Caddy Documentation](https://caddyserver.com/docs/)
- [Let's Encrypt](https://letsencrypt.org/docs/)
- [Mozilla SSL Config](https://ssl-config.mozilla.org/)
- [Actix-Web OpenSSL](https://actix.rs/docs/server/)

---

**Need Help?** Check the [FreeRadical CMS Wiki](https://github.com/cyberiums/freeradical/wiki)
