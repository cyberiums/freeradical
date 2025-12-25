# Database Configuration Guide

FreeRadical CMS supports both **MySQL** and **PostgreSQL** with runtime database switching. No rebuild required!

## Quick Start

### MySQL (Default)
```bash
docker-compose up -d
```

### PostgreSQL
```bash
docker-compose - docker-compose.postgres.yml up -d
```

## Supported Databases

| Database    | Version | Status | Production Ready |
|-------------|---------|--------|------------------|
| MySQL       | 8.0+    | ✅     | Yes              |
| PostgreSQL  | 15+     | ✅     | Yes              |

##Environment Configuration

### MySQL Configuration

Create `.env` file:
```bash
DATABASE_URL=mysql://freeradical:password@localhost:5506/freeradical
APP_MYSQL_USERNAME=freeradical
APP_MYSQL_PASSWORD=password
APP_MYSQL_DATABASE=freeradical
APP_MYSQL_URL=localhost
APP_MYSQL_PORT=5506
```

### PostgreSQL Configuration

Create `.env` file:
```bash
DATABASE_URL=postgres://freeradical:password@localhost:5432/freeradical
```

## Key Differences

### PageStatus Field

- **MySQL**: Uses native `ENUM('draft', 'scheduled', 'published', 'archived')` type
- **PostgreSQL**: Uses `VARCHAR(9)` with `CHECK` constraint

Both map to the same Rust `PageStatus` enum type seamlessly!

### UUID Generation

- **MySQL**: Uses `UUID()` function
- **PostgreSQL**: Uses `gen_random_uuid()` function

### Data Types

| MySQL              | PostgreSQL    | Notes                    |
|--------------------|---------------|--------------------------|
| BIGINT UNSIGNED    | BIGINT        | Uses signed in PostgreSQL |
| AUTO_INCREMENT     | SERIAL        | Automatic sequences      |
| ENUM               | VARCHAR+CHECK | With constraint validation |
| DATETIME           | TIMESTAMP     | Timezone handling        |

## Migrations

### MySQL Migrations
Located in: `migrations/`

Run with:
```bash
diesel migration run --database-url=mysql://user:pass@host/db
```

### PostgreSQL Migrations
Located in: `migrations_postgres/`

Run with:
```bash
diesel migration run --database-url=postgres://user:pass@host/db --migration-dir=migrations_postgres
```

## Testing

### Test with MySQL
```bash
bash scripts/test_mysql.sh
```

**Expected output:**
- ✓ MySQL connection successful
- ✓ Found 20 tables
- ✓ API responding (HTTP 200)
- ✓ PageStatus ENUM type found

### Test with PostgreSQL
```bash
bash scripts/test_postgres.sh
```

**Expected output:**
- ✓ PostgreSQL connection successful
- ✓ Migrations applied
- ✓ PageStatus VARCHAR type found
- ✓ API responding (HTTP 200)

## Docker Deployment

### MySQL Stack
```bash
docker-compose up -d
# Access at http://localhost:8000
```

### PostgreSQL Stack
```bash
docker-compose -f docker-compose.postgres.yml up -d
# Access at http://localhost:8001
```

## Performance Considerations

### MySQL
- **Pros**: Proven stability, widespread adoption, excellent replication
- **Cons**: Less advanced JSON support
- **Best for**: Traditional CMS deployments, shared hosting

### PostgreSQL
- **Pros**: Advanced features, better JSON support, extensibility
- **Cons**: Higher memory usage
- **Best for**: Complex queries, data analytics, modern infrastructure

## Migration Between Databases

### MySQL → PostgreSQL

1. Export MySQL data:
```bash
mysqldump -u freeradical -p freeradical > dump.sql
```

2. Convert ENUM values (automated in PostgreSQL migrations)

3. Import to PostgreSQL:
```bash
psql -U freeradical -d freeradical < converted_dump.sql
```

### PostgreSQL → MySQL

1. Export PostgreSQL data:
```bash
pg_dump -U freeradical freeradical > dump.sql
```

2. Convert data types

3. Import to MySQL:
```bash
mysql -u freeradical -p freeradical < converted_dump.sql
```

## Troubleshooting

### Connection Issues

**MySQL:**
```bash
docker-compose logs mysql
docker-compose exec mysql mysql -ufreeradical -ppassword freeradical
```

**PostgreSQL:**
```bash
docker-compose -f docker-compose.postgres.yml logs postgres
docker-compose -f docker-compose.postgres.yml exec postgres psql -Ufreeradical -d freeradical
```

### Migration Failures

**Check migration status:**
```bash
diesel migration list --database-url=$DATABASE_URL
```

**Revert last migration:**
```bash
diesel migration revert --database-url=$DATABASE_URL
```

## Production Recommendations

1. **Use connection pooling** (enabled by default in FreeRadical)
2. **Enable query logging** in development only
3. **Regular backups** using native dump tools
4. **Monitor connection counts** and set appropriate limits
5. **Use read replicas** for high-traffic deployments

## References

- [Diesel ORM Documentation](https://diesel.rs)
- [MySQL 8.0 Reference](https://dev.mysql.com/doc/refman/8.0/en/)
- [PostgreSQL 15 Documentation](https://www.postgresql.org/docs/15/)
