# FreeRadical CMS Documentation

Welcome to the FreeRadical CMS documentation! This README provides an overview of all available documentation and guides.

---

## 📚 Documentation Structure

```
docs/
├── sdk/                    # SDK Development
│   ├── SDK_DEVELOPER_GUIDE.md
│   └── sdk_overview.md
│
├── core/                   # Core Development
│   └── CORE_DEVELOPER_GUIDE.md
│
├── architecture/           # System Architecture
│   └── payment_architecture.md
│
├── iterations/             # Development Iterations
│   ├── ITERATION-*.md
│   ├── PHASE-*.md
│   └── *-COMPLETE.md
│
├── roadmaps/              # Roadmaps & Planning
│   ├── roadmap.md
│   └── ROADMAP-*.md
│
├── releases/              # Release Notes
│   ├── CHANGELOG.md
│   └── RELEASE-NOTES-*.md
│
└── walkthrough.md         # Feature Walkthrough

```

---

## 🎯 Quick Start Guides

### For SDK Developers

**Building client libraries for FreeRadical CMS**

📖 [SDK Developer Guide](./sdk/SDK_DEVELOPER_GUIDE.md)

Learn how to:
- Use existing SDKs (TypeScript, Python, Go)
- Build new SDKs for other languages
- Understand API authentication
- Handle errors properly
- Test and publish SDKs

**Available SDKs**:
- ✅ **TypeScript/JavaScript** - `@freeradical/sdk`
- ✅ **Python** - `freeradical-client`
- ✅ **Go** - `freeradical-go-client`

📖 [SDK Overview](./sdk/sdk_overview.md)

---

### For Core Developers

**Contributing to FreeRadical CMS backend/frontend**

📖 [Core Developer Guide](./core/CORE_DEVELOPER_GUIDE.md)

Learn how to:
- Set up development environment
- Understand project architecture
- Create new endpoints
- Work with Diesel ORM
- Build React components
- Run migrations
- Deploy to production

**Tech Stack**:
- Backend: Rust + Actix-Web
- Frontend: React + TypeScript
- Database: MySQL + Diesel
- Admin: Refine + Ant Design

---

## 🏗️ Architecture Documentation

### Payment System Architecture

📖 [Payment Architecture](./architecture/payment_architecture.md)

Extensible payment handler system supporting:
- ✅ Stripe
- ✅ PayPal
- ✅ Square
- 🔌 Any payment provider (pluggable)

Trait-based design for adding new payment providers without modifying core code.

---

## 📖 Feature Documentation

### Complete Feature Walkthrough

📖 [Walkthrough](./walkthrough.md)

Comprehensive overview of all implemented features:
- **Phase 6**: Plugin System, Multi-Engine Templates, SDKs
- **Phase 7**: 2FA/TOTP, Backup Service
- **Phase 8**: Payment System (3 providers), Commerce Models

---

## 📋 Development Iterations

Historical documentation of development phases:

📁 [Iterations](./iterations/)

- Iteration planning documents
- Phase implementation summaries
- Completed feature checklists
- Development summaries

---

## 🗺️ Roadmaps & Planning

Future development plans and roadmaps:

📁 [Roadmaps](./roadmaps/)

- Product roadmap
- Feature prioritization
- Roadmap audits

---

## 📢 Release Notes

Version history and changelogs:

📁 [Releases](./releases/)

- Release notes by version
- Breaking changes
- Migration guides
- Changelog

---

## 🚀 Getting Started

### Quick Setup

```bash
# Clone repository
git clone https://github.com/your-org/freeradical.git
cd freeradical

# Backend setup
cargo build
diesel migration run

# Frontend setup
cd admin && npm install

# Run development servers
cargo run              # Backend on :8080
cd admin && npm run dev   # Frontend on :5173
```

### Using an SDK

**TypeScript**:
```bash
npm install @freeradical/sdk
```

**Python**:
```bash
pip install freeradical-client
```

**Go**:
```bash
go get github.com/your-org/freeradical-go-client
```

---

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/your-org/freeradical/issues)
- **Discussions**: [GitHub Discussions](https://github.com/your-org/freeradical/discussions)
- **Wiki**: [Project Wiki](https://github.com/your-org/freeradical/wiki)

---

## 🤝 Contributing

We welcome contributions! Please see:

- [Core Developer Guide](./core/CORE_DEVELOPER_GUIDE.md#contributing)
- [SDK Developer Guide](./sdk/SDK_DEVELOPER_GUIDE.md#contributing)

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

---

## 📜 License

MIT License - see [LICENSE](../LICENSE) for details

---

## 🎯 Current Status

**Version**: 0.9.0  
**Development Progress**: 75% to v1.0.0

**Completed**:
- ✅ Phase 6: Ecosystem Expansion (100%)
- ✅ Phase 7: Enterprise Features (60%)
- ✅ Phase 8: Commerce (65%)

**Next Steps**:
- Product/Order CRUD endpoints
- Frontend 2FA UI
- E2E testing
- Documentation finalization

---

**Last Updated**: December 2025

For the most up-to-date information, visit our [GitHub repository](https://github.com/your-org/freeradical).
