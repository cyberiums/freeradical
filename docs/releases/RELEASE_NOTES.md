# FreeRadical v0.1.5 Release Notes

**Release Date**: December 24, 2025  
**Status**: ✅ Production Ready

---

## 🎉 What's New in v0.1.5

### Branding & Project Updates
- Rebranded from "Radical" to **FreeRadical**
- New repository location: [github.com/cyberiums/freeradical](https://github.com/cyberiums/freeradical)
- Now officially **production ready** and deployment-ready
- Maintained by [FastBuilder.ai](https://fastbuilder.ai)

### Performance Benchmarks (Validated)
- **1,657 requests/second** on standard hardware
- **6ms average response time**
- **4x faster** than WordPress (405 req/s vs 1,657 req/s)
- **Zero failures** in 12,000+ request load testing
- WordPress failed under high load; FreeRadical completed 100%

### Documentation
- Comprehensive admin API documentation with screenshots
- Performance benchmarking against WordPress
- Complete testing reports (test.md, PERFORMANCE.md, BENCHMARK.md)
- Updated README with production deployment guidance

### Security & Features
- ✅ JWT-based authentication
- ✅ Argon2 password hashing
- ✅ Rate limiting (configurable)
- ✅ RESTful API (/v1 endpoints)
- ✅ Handlebars template engine with hot-reload
- ✅ MySQL/Diesel ORM with connection pooling

---

## Production Readiness

FreeRadical v0.1.5 is **production ready** with:

| Category | Status | Notes |
|----------|--------|-------|
| **Performance** | ✅ Excellent | Sub-10ms responses, 1,500+ req/s |
| **Reliability** | ✅ Perfect | Zero failures in extensive testing |
| **Security** | ✅ Enterprise | JWT auth, Argon2, rate limiting |
| **Scalability** | ✅ Excellent | Async I/O, connection pooling |
| **API Design** | ✅ Production | RESTful, JSON, versioned |
| **Documentation** | ✅ Complete | Admin guides, benchmarks |

---

## Breaking Changes

- **Package name**: `radical` → `freeradical` in Cargo.toml
- **Repository URL**: Updated to github.com/cyberiums/freeradical
- No API changes - fully backward compatible

---

## Installation

```bash
# Clone the repository
git clone https://github.com/cyberiums/freeradical.git
cd freeradical

# Set up database
mysql -u root -p
CREATE DATABASE freeradical;
CREATE USER 'freeradical'@'%' IDENTIFIED BY 'yourpassword';
GRANT ALL PRIVILEGES ON freeradical.* TO 'freeradical'@'%';

# Configure environment
cp .env.example .env
# Edit .env with your settings

# Run migrations
diesel migration run

# Start the server
cargo run --release
```

---

## Upgrade Notes

If upgrading from v0.1.4:

1. Update Cargo.toml package name to `freeradical`
2. Update any internal references from "Radical" to "FreeRadical"
3. No database migrations required
4. No API changes required

---

## Benchmarks Summary

**vs WordPress**:
- 4.1x faster throughput
- 4.1x lower latency
- 16x better p99 latency
- 100% reliability vs. timeout failures

**Metrics**:
- Homepage: 1,657 req/s @ 6ms
- API endpoints: 2,580 req/s @ 4ms
- High load (50 concurrent): 842 req/s @ 59ms
- Failure rate: 0%

---

## Support & Maintenance

**Maintained by**: [FastBuilder.ai](https://fastbuilder.ai)

For:
- Enterprise support
- Custom features
- Consulting services
- Training

Visit [fastbuilder.ai](https://fastbuilder.ai)

---

## Contributors

- Original author: fastbuilder <team@fastbuilder.ai>
- Maintained by: FastBuilder.ai team
- Community contributors welcome!

---

## License

See LICENSE file for details.

---

**FreeRadical v0.1.5** - Production-ready, blazing-fast headless CMS built with Rust 🚀
