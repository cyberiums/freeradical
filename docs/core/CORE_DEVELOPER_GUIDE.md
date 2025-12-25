# FreeRadical CMS - Core Developer Guide

## 📚 Table of Contents

1. [Introduction](#introduction)
2. [Development Setup](#development-setup)
3. [Project Architecture](#project-architecture)
4. [Development Workflow](#development-workflow)
5. [Backend Development](#backend-development)
6. [Frontend Development](#frontend-development)
7. [Database Management](#database-management)
8. [Testing](#testing)
9. [Deployment](#deployment)
10. [Contributing](#contributing)

---

## Introduction

This guide is for developers working on **FreeRadical CMS core**:
- Backend (Rust/Actix-Web)
- Frontend (React/TypeScript)
- Database (MySQL/Diesel)
- Infrastructure (Docker, CI/CD)

### Tech Stack

**Backend**:
- **Language**: Rust 2021 edition
- **Web Framework**: Actix-Web 3.x
- **ORM**: Diesel 2.2 (MySQL)
- **Authentication**: JWT + Argon2

**Frontend**:
- **Framework**: React 18 + TypeScript
- **Build Tool**: Vite
- **UI Library**: Refine + Ant Design
- **State Management**: TanStack Query

**Database**:
- **Primary**: MySQL 8.0+
- **Caching**: Redis (optional)

---

## Development Setup

### Prerequisites

```bash
# Rust (latest stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js 18+
nvm install 18
nvm use 18

# MySQL 8.0+
brew install mysql@8.0

# Diesel CLI
cargo install diesel_cli --no-default-features --features mysql
```

### Clone and Setup

```bash
# Clone repository
git clone https://github.com/your-org/freeradical.git
cd freeradical

# Install Rust dependencies
cargo build

# Install frontend dependencies
cd admin && npm install && cd ..

# Setup environment
cp .env.sample .env
# Edit .env with your MySQL credentials
```

### Database Setup

```bash
# Create database
mysql -u root -p -e "CREATE DATABASE freeradical;"

# Run migrations
diesel migration run

# Seed data (optional)
cargo run --bin seed
```

### Running Development Servers

**Backend**:
```bash
cargo run
# Runs on http://localhost:8080
```

**Frontend**:
```bash
cd admin
npm run dev
# Runs on http://localhost:5173
```

**Watch Mode** (Rust auto-reload):
```bash
cargo install cargo-watch
cargo watch -x run
```

---

## Project Architecture

### Directory Structure

```
freeradical/
├── src/                      # Rust backend
│   ├── controllers/          # HTTP request handlers
│   ├── models/               # Database models
│   ├── services/             # Business logic
│   │   ├── auth_service.rs   # JWT authentication
│   │   ├── totp_service.rs   # 2FA TOTP
│   │   ├── payment_service/  # Payment handlers
│   │   │   ├── stripe.rs
│   │   │   ├── paypal.rs
│   │   │   └── square.rs
│   │   └── plugin_service/   # Plugin system
│   ├── routers/              # Route definitions
│   ├── schema.rs             # Diesel schema
│   └── main.rs               # Application entry
│
├── migrations/               # Database migrations
│   ├── 2025-12-25-010000_add_2fa/
│   └── 2025-12-25-020000_add_commerce/
│
├── admin/                    # React admin dashboard
│   ├── src/
│   │   ├── components/       # React components
│   │   ├── pages/            # Page components
│   │   └── App.tsx           # Root component
│   └── package.json
│
├── sdk/                      # TypeScript SDK
├── sdks/                     # Python/Go SDKs
├── docs/                     # Documentation
├── scripts/                  # Automation scripts
└── Cargo.toml
```

---

## Development Workflow

### Feature Development

1. **Create Branch**:
   ```bash
   git checkout -b feature/add-categories
   ```

2. **Backend Changes**:
   - Add model in `src/models/`
   - Create migration with `diesel migration generate`
   - Add controller in `src/controllers/`
   - Register routes in `src/routers/`

3. **Frontend Changes**:
   - Add component in `admin/src/components/`
   - Create page in `admin/src/pages/`
   - Update routing

4. **Test**:
   ```bash
   cargo test
   cd admin && npm test
   ```

5. **Commit**:
   ```bash
   git add .
   git commit -m "feat: add categories feature"
   ```

6. **Pull Request**:
   - Push to GitHub
   - Create PR with description
   - Wait for CI/CD checks

---

## Backend Development

### Creating a New Endpoint

#### 1. Define Model

**`src/models/category_models.rs`**:
```rust
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[diesel(table_name = categories)]
pub struct NewCategory {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}
```

#### 2. Create Migration

```bash
diesel migration generate add_categories
```

**`migrations/xxx_add_categories/up.sql`**:
```sql
CREATE TABLE categories (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

**`migrations/xxx_add_categories/down.sql`**:
```sql
DROP TABLE categories;
```

Run migration:
```bash
diesel migration run
```

#### 3. Create Controller

**`src/controllers/category_controller.rs`**:
```rust
use actix_web::{web, HttpResponse};
use crate::models::category_models::{Category, NewCategory};
use crate::models::{pool_handler, MySQLPool};
use crate::services::errors_service::CustomHttpError;

pub async fn list_categories(
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mut conn = pool_handler(pool)?;
    
    let categories = categories::table
        .load::<Category>(&mut conn)?;
    
    Ok(HttpResponse::Ok().json(categories))
}

pub async fn create_category(
    category: web::Json<NewCategory>,
    pool: web::Data<MySQLPool>,
) -> Result<HttpResponse, CustomHttpError> {
    let mut conn = pool_handler(pool)?;
    
    diesel::insert_into(categories::table)
        .values(&category.into_inner())
        .execute(&mut conn)?;
    
    Ok(HttpResponse::Created().finish())
}
```

#### 4. Register Routes

**`src/routers/category_router.rs`**:
```rust
use actix_web::{web, Scope};
use crate::controllers::category_controller::*;

pub struct CategoryRouter;

impl CategoryRouter {
    pub fn new() -> Scope {
        web::scope("/categories")
            .route("", web::get().to(list_categories))
            .route("", web::post().to(create_category))
    }
}
```

**`src/main.rs`**:
```rust
.service(
    web::scope("/v1")
        .service(CategoryRouter::new())
)
```

---

### Working with Diesel ORM

#### Queries

```rust
use crate::schema::pages::dsl::*;

// Select all
let all_pages = pages.load::<Page>(&mut conn)?;

// Filter
let active_pages = pages
    .filter(is_published.eq(true))
    .load::<Page>(&mut conn)?;

// Find by ID
let page = pages
    .find(page_uuid)
    .first::<Page>(&mut conn)?;

// Joins
let results = pages
    .inner_join(categories::table)
    .select((pages::all_columns, categories::name))
    .load::<(Page, String)>(&mut conn)?;
```

#### Inserts

```rust
diesel::insert_into(pages::table)
    .values(&new_page)
    .execute(&mut conn)?;
```

#### Updates

```rust
diesel::update(pages::table.find(uuid))
    .set(&updated_page)
    .execute(&mut conn)?;
```

#### Deletes

```rust
diesel::delete(pages::table.find(uuid))
    .execute(&mut conn)?;
```

---

### Authentication & Authorization

#### JWT Authentication

**Generate Token**:
```rust
use crate::services::auth_service::{Claims, encrypt};

let claims = Claims {
    sub: username.clone(),
    exp: (OffsetDateTime::now_utc() + Duration::days(7)).unix_timestamp() as usize,
};

let token = encrypt(claims)?;
```

**Verify Token** (Middleware):
```rust
use crate::services::auth_service::authenticate;

pub async fn protected_route(
    claim: Claims, // Extracted by middleware
) -> Result<HttpResponse, CustomHttpError> {
    // claim.sub contains username
    Ok(HttpResponse::Ok().json(format!("Hello, {}", claim.sub)))
}
```

#### 2FA/TOTP

```rust
use crate::services::totp_service::TotpService;

// Generate secret + QR code
let (secret, qr_base64) = TotpService::generate_secret(&username)?;

// Verify code
let is_valid = TotpService::verify(&secret, &user_code)?;
```

---

## Frontend Development

### Admin Dashboard Structure

**`admin/src/App.tsx`**:
```typescript
import { Refine } from "@refinedev/core";
import { dataProvider } from "./dataProvider";
import { authProvider } from "./authProvider";

function App() {
  return (
    <Refine
      dataProvider={dataProvider("http://localhost:8080/v1")}
      authProvider={authProvider}
      resources={[
        { name: "pages", list: PageList, create: PageCreate },
        { name: "modules", list: ModuleList },
      ]}
    />
  );
}
```

### Creating a Resource Page

**`admin/src/pages/categories/list.tsx`**:
```typescript
import { List, useTable } from "@refinedev/antd";
import { Table, Space } from "antd";

export const CategoryList = () => {
  const { tableProps } = useTable<Category>();

  return (
    <List>
      <Table {...tableProps} rowKey="id">
        <Table.Column dataIndex="name" title="Name" />
        <Table.Column dataIndex="slug" title="Slug" />
        <Table.Column
          title="Actions"
          render={(_, record) => (
            <Space>
              <EditButton size="small" recordItemId={record.id} />
              <DeleteButton size="small" recordItemId={record.id} />
            </Space>
          )}
        />
      </Table>
    </List>
  );
};
```

---

## Database Management

### Creating Migrations

```bash
# Generate migration
diesel migration generate add_feature_name

# Edit up.sql and down.sql

# Run migration
diesel migration run

# Rollback (if needed)
diesel migration revert
```

### Migration Best Practices

1. **Always additive**: Never drop columns in production
2. **Default values**: Add defaults for new columns
3. **Indexes**: Create indexes for frequently queried columns
4. **Foreign keys**: Use ON DELETE CASCADE/RESTRICT appropriately

### Schema Updates

After migration:
```bash
# Regenerate schema.rs
diesel print-schema > src/schema.rs
```

---

## Testing

### Unit Tests

**`src/services/totp_service.rs`**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_totp_generation() {
        let (secret, qr) = TotpService::generate_secret("testuser").unwrap();
        assert!(!secret.is_empty());
        assert!(qr.starts_with("data:image/png"));
    }
}
```

Run tests:
```bash
cargo test
```

### Integration Tests

**`tests/integration_test.rs`**:
```rust
use actix_web::{test, App};

#[actix_rt::test]
async fn test_create_page() {
    let app = test::init_service(App::new().configure(configure_routes)).await;
    
    let req = test::TestRequest::post()
        .uri("/v1/page")
        .set_json(&json!({
            "page_title": "Test",
            "page_url": "/test"
        }))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
```

---

## Deployment

### Docker

**Build**:
```bash
docker build -t freeradical:latest .
```

**Run**:
```bash
docker-compose up -d
```

### Production Checklist

- [ ] Set `RUST_LOG=info` (not `debug`)
- [ ] Use production database credentials
- [ ] Enable HTTPS (reverse proxy)
- [ ] Configure CORS properly
- [ ] Set up database backups
- [ ] Monitor logs (Sentry, Datadog, etc.)
- [ ] Set up health checks
- [ ] Configure rate limiting

---

## Contributing

### Code Review Guidelines

- **Rust**: Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- **TypeScript**: Use ESLint + Prettier
- **Commits**: Use [Conventional Commits](https://www.conventionalcommits.org/)
- **Tests**: Minimum 80% coverage
- **Documentation**: Update relevant docs

### Pull Request Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests pass
- [ ] Manual testing completed

## Checklist
- [ ] Code follows project conventions
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No new warnings
```

---

## Additional Resources

- [SDK Developer Guide](../sdk/SDK_DEVELOPER_GUIDE.md)
- [Architecture Documentation](../architecture/)
- [API Reference](../sdk/SDK_DEVELOPER_GUIDE.md#api-reference)
- [Payment Integration](../architecture/payment_architecture.md)
- [Plugin System](../architecture/plugin_system.md)

---

**Happy Coding!** 🦀
