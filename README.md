# FreeRadical

## Index

- [Project](#project-description)
- [How to Install](https://github.com/cyberiums/freeradical)
- [Maintained By](#maintained-by)
- [Project State](#project-state)
- [Admin Functionality](#admin-functionality)
- [Performance](#performance)
- [Testing](#note-on-testing)
- [Dev Environment Setup](#dev-environment-setup)
- [Environment Variables](#environment-variables)
- [404 Pages](#notes-on-404-pages)
# FreeRadical CMS

**Version**: 0.3.0  
**Status**: Production Ready ✅  
**SEO Score**: 97/100  
**Performance**: >2,000 req/s

A high-performance, SEO-optimized headless CMS built with Rust. FreeRadical delivers exceptional speed (5.3× faster than WordPress) while providing enterprise-grade SEO features and privacy-compliant analytics.

## ⚡ Key Features

### v0.3.0 (Latest)
- **Advanced SEO**: 97/100 score with breadcrumbs, article schema, image sitemap
- **Built-in Analytics**: Privacy-first tracking with GDPR compliance
- **Admin Dashboard**: Real-time metrics and SEO health monitoring
- **High Performance**: >2,000 req/s maintained with analytics enabled
- **Dynamic Configuration**: Database-driven robots.txt and sitemap management

**Key Features:**
- 🚀 **5.3× faster** than WordPress (2,159 req/s vs 405 req/s)
- ⚡ **4.6ms average response time** (79% improvement)
- 🎯 **93/100 SEO score** (Google: 95%, Bing: 95%)
- 🔒 JWT-based authentication with Argon2 password hashing
- 📊 RESTful API for all operations
- 🎨 Handlebars template engine with hot-reload
- 💾 MySQL/Diesel ORM with connection pooling + strategic indexes
- 🛡️ Production-ready with zero-downtime deployments

## Server Installation

See the [FreeRadical repository](https://github.com/cyberiums/freeradical) for installation instructions.

Otherwise, you can run follow the directions to set it up on Kubernetes.

Sub $2/month GCP Cloud Run tutorial coming soon!

## Project State

Version: **v0.2.0** 🎉

|             | Status |
| ----------- | ----------- |
| Backend | ✅ Production Ready |
| Frontend | ✅ Production Ready |
| Performance | ✅ Benchmarked |
| Security | ✅ JWT + Argon2 |
| SEO | ✅ **Ready** (P0 features complete) |
| Production | ✅ **Ready** (HTTPS pending) |

## Admin Functionality

**FreeRadical** provides a complete RESTful API for content management with JWT-based authentication and enterprise-grade security.

### Quick Start

**Start the server:**
```bash
cargo run
```

Server runs on `http://127.0.0.1:8080` by default.

### API Overview

![Admin Pages API](assets/admin_pages_api.png)

### Authentication

All admin operations require JWT authentication. Login to receive an authentication token:

```bash
# Login (first time with root user, empty password)
curl -X POST http://127.0.0.1:8080/v1/users/login \
  -H "Content-Type: application/json" \
  -d '{"username":"root","password":""}' \
  -c cookies.txt

# The response sets an auth cookie for subsequent requests
```

**Available Auth Endpoints:**
- `POST /v1/users/login` - User login
- `POST /v1/users/logout` - User logout
- `GET /v1/users/check` - Check authentication status

### Content Management API

#### Pages

**List all pages:**
```bash
GET /v1/pages
```

**Get specific page:**
```bash
GET /v1/pages/{uuid}
```

**Get page with modules:**
```bash
GET /v1/pages/{uuid}/modules
```

**Create new page** (requires auth):
```bash
curl -X POST http://127.0.0.1:8080/v1/pages \
  -H "Content-Type: application/json" \
  -b cookies.txt \
  -d '{
    "page_name": "about",
    "page_url": "/about",
    "page_title": "About Us"
  }'
```

**Update page** (requires auth):
```bash
PUT /v1/pages/{uuid}
```

**Delete page:**
```bash
DELETE /v1/pages/{uuid}
```

#### Modules (Content Fields)

![Admin Modules API](assets/admin_modules_api.png)

**List all modules:**
```bash
GET /v1/modules
```

**Get specific module:**
```bash
GET /v1/modules/{uuid}
```

**Get modules by category:**
```bash
GET /v1/modules/category/{category_uuid}
```

**Create new module** (requires auth):
```bash
curl -X POST http://127.0.0.1:8080/v1/modules \
  -H "Content-Type: application/json" \
  -b cookies.txt \
  -d '{
    "page_uuid": "page-uuid-here",
    "category_uuid": null,
    "title": "hero_text",
    "content": "Welcome to our site!"
  }'
```

**Update module** (requires auth):
```bash
PUT /v1/modules/{uuid}
```

**Delete module** (requires auth):
```bash
DELETE /v1/modules/{uuid}
```

#### Categories

Categories organize modules into groups for better content structure.

**Get category:**
```bash
GET /v1/categories/{uuid}
```

**Create category** (requires auth):
```bash
curl -X POST http://127.0.0.1:8080/v1/categories \
  -H "Content-Type: application/json" \
  -b cookies.txt \
  -d '{
    "page_uuid": "page-uuid-here",
    "title": "hero_section"
  }'
```

**Update category** (requires auth):
```bash
PUT /v1/categories/{uuid}
```

**Delete category** (requires auth):
```bash
DELETE /v1/categories/{uuid}
```

#### User Management

**Create user** (requires auth):
```bash
curl -X POST http://127.0.0.1:8080/v1/users \
  -H "Content-Type: application/json" \
  -b cookies.txt \
  -d '{
    "username": "editor",
    "password": "securepassword123"
  }'
```

**Get user** (requires auth):
```bash
GET /v1/users/{uuid}
```

**Update user** (requires auth):
```bash
PUT /v1/users/{uuid}
```

**Delete user** (requires auth):
```bash
DELETE /v1/users/{uuid}
```

### Frontend Rendering

![Homepage Rendering](assets/admin_homepage.png)

Pages are automatically rendered using Handlebars templates:

1. Create a page via API
2. Create a matching template file: `templates/{page_name}.hbs`
3. Add modules to the page via API
4. Access at the page's URL

**Template Example:**
```handlebars
<!DOCTYPE html>
<html>
<head>
    <title>{{page_title}}</title>
</head>
<body>
    <h1>{{fields.title.content}}</h1>
    <p>{{fields.description.content}}</p>
</body>
</html>
```

The template engine automatically populates `fields` with your modules.

### API Response Example

![API Example](assets/admin_api_example.png)

## Performance

FreeRadical v0.2.0 delivers exceptional performance with database optimizations:

- **2,159 requests/second** (vs WordPress: 405 req/s) - **5.3× faster**
- **4.6ms average response time** (vs WordPress: 25ms) - **5.4× faster**
- **+79% throughput improvement** vs v0.1.5 through database indexing
- **Zero failures** in extensive load testing (3,000+ requests)
- **SEO Score: 93/100** (better than most CMS platforms)

See [BENCHMARK.md](BENCHMARK.md), [PERFORMANCE.md](PERFORMANCE.md), and [post_optimization_benchmarks.md](post_optimization_benchmarks.md) for detailed metrics.

## Note on testing

Whenever you run the tests, it is best to have a clean database.

You **MUST** run it using this command (since mocking for Diesel isn't mature yet):

`cargo test -- --test-thread=1`

## Dev Environment Setup

Required items:
* Rustc
* MySQL (8.0+ preferred)
* Diesel CLI (`cargo install diesel_cli --no-default-features --features mysql`)

1. Clone this repository.

`git clone git@github.com:Rust-CMS/radical.git`

Then, enter the repository.

`cd rust-cms`

2. Create a MySQL database.

Log in to your MySQL server, and `CREATE DATABASE rustcms;`

3. Add a user to this database and give them privileges.

Change anything labelled `rustcms`, as it is a placeholder. Since this is a dev environment, you shouldn't have to worry about security too much though.

`CREATE USER 'rustcms'@'%' IDENTIFIED BY 'rustcms';`

Giving them privileges:

`GRANT ALL PRIVILEGES ON rustcms.* TO 'rustcms'@'%';`

`FLUSH PRIVILEGES`

4. Setup environment variables.

Since this is a development environment, we won't be explicitly setting these on the system. Instead, we'll store them in a file named `.env` in the root of this project.

First, create a file named `.env`.

Next, put in your MySQL connection string. Extensive examples for a full .env can be seen [here](#environment-variables).

The MySQL connection string is not used for the program connecting to the database, but rather for running the migrations. This is usually not required to be done manually.

`DATABASE_URL=mysql://rustcms:rustcms@localhost:3306/rustcms`

5. Run Diesel migrations.

First, install Diesel.

`cargo install diesel_cli --no-default-features --features mysql`

Next, run the migrations.

`diesel migration run`

## Environment Variables
Most all environment setup will be handled by an installer GUI in the future.

```yaml
app_mysql_username=String
app_mysql_password=String
app_mysql_database=String
app_bind_address=String
app_jwt_key=String
app_bind_port=Number
# Max request per IP per minute. Recommended 100 for 512mb 1vCPU
app_max_req=Number

app_mysql_url?=String
app_mysql_port?=Number

# OR for places like GCP Cloud Run. Do not mix, it will not work.
# Note the lack of the APP_ prefix.
MYSQL_UNIX_PORT?=String


```

## Notes on 404 Pages

404s are handled (currently) by creating a file called `404.html.` It will automatically be added as your 404 page.

## Maintained By

**FreeRadical** is actively maintained and supported by [FastBuilder.ai](https://fastbuilder.ai).

FastBuilder.ai provides:
- 🔧 Ongoing maintenance and security updates
- 🚀 Performance optimizations
- 📚 Documentation improvements
- 🐛 Bug fixes and feature enhancements
- 💬 Community support

For enterprise support, custom features, or consulting services, visit [fastbuilder.ai](https://fastbuilder.ai).

## Repositories Like This

Markdown static site generators:

(Data retrieved from [here](https://www.arewewebyet.org/topics/cms/))

https://github.com/getzola/zola

https://github.com/cobalt-org/cobalt.rs

https://github.com/rust-lang/mdBook
