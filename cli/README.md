# FreeRadical CLI

Command-line tool for FreeRadical CMS project management.

## Installation

```bash
cd freeradical-cli
cargo build --release
sudo cp target/release/freeradical /usr/local/bin/
```

## Commands

### Initialize Project
Create a new FreeRadical CMS project with full structure:

```bash
freeradical init my-blog
cd my-blog
```

Creates:
- `config.toml` - Configuration file
- `.env.example` - Environment template
- `.gitignore` - Git ignore rules
- `README.md` - Project documentation
- `uploads/` - Media storage directory
- `templates/` - Template directory
- `migrations/` - Database migrations

### Export Content
Export content to JSON files:

```bash
# Export pages
freeradical export --resource=pages --output=pages.json

# Export modules
freeradical export --resource=modules --output=modules.json

# Export media
freeradical export --resource=media --output=media.json
```

### Import Content
Import content from JSON files with batch processing:

```bash
freeradical import --file=pages.json
```

Features:
- Auto-detects resource type from JSON structure
- Batch processing with progress indicators
- Per-item error handling
- Continues on individual failures

### Database Migrations
Manage database migrations using diesel CLI:

```bash
# Run pending migrations
freeradical migrate run

# Rollback last migration
freeradical migrate rollback

# Show migration status
freeradical migrate status
```

**Note**: Requires diesel CLI: `cargo install diesel_cli`

### Development Server
Start the development server:

```bash
freeradical dev
```

Runs `cargo run` in the current directory.

### Production Build
Build for production:

```bash
freeradical build
```

Runs `cargo build --release`.

## Configuration

Projects use `config.toml` for configuration:

```toml
[server]
host = "127.0.0.1"
port = 8000

[database]
url = "mysql://user:password@localhost/freeradical"
pool_size = 10

[cache]
enabled = true
redis_url = "redis://127.0.0.1:6379"

[security]
jwt_secret = "change-in-production"
```

## Examples

### Complete Workflow
```bash
# 1. Create new project
freeradical init my-site
cd my-site

# 2. Configure environment
cp .env.example .env
# Edit .env with your database credentials

# 3. Run migrations
freeradical migrate run

# 4. Start development
freeradical dev
```

### Content Migration
```bash
# Export from old site
cd old-site
freeradical export --resource=pages --output=backup.json

# Import to new site  
cd ../new-site
freeradical import --file=../old-site/backup.json
```

## Troubleshooting

### Export/Import 504 Errors
Make sure the FreeRadical server is running:
```bash
cargo run
```

### Migration Command Not Found
Install diesel CLI:
```bash
cargo install diesel_cli --no-default-features --features mysql
```

## License

MIT
