#!/bin/bash
# Quick migration copier with basic PostgreSQL syntax fixes

set -e

echo "Copying and converting MySQL migrations to PostgreSQL..."

for mysql_dir in migrations/*/; do
    dirname=$(basename "$mysql_dir")
    
    # Create postgres directory if it doesn't exist
    mkdir -p "migrations_postgres/$dirname"
    
    # Convert up.sql if it doesn't exist
    if [ -f "$mysql_dir/up.sql" ] && [ ! -f "migrations_postgres/$dirname/up.sql" ]; then
        echo "Converting $dirname/up.sql..."
        
        # Basic conversion
        sed -E \
            -e 's/INSERT IGNORE/INSERT/g' \
            -e 's/UUID\(\)/gen_random_uuid()::text/g' \
            -e 's/BIGINT UNSIGNED/BIGINT/g' \
            -e 's/AUTO_INCREMENT/SERIAL/g' \
            -e 's/INT([^E])/INTEGER/g' 2>/dev/null \
            -e 's/DATETIME/TIMESTAMP/g' \
            -e 's/ENGINE=[^ ;]*//' \
            -e 's/DEFAULT CHARSET=[^ ;]*//' \
            -e 's/COMMENT ['"'"'][^'"'"']*['"'"']//' \
            "$mysql_dir/up.sql" > "migrations_postgres/$dirname/up.sql" || \
            cp "$mysql_dir/up.sql" "migrations_postgres/$dirname/up.sql"
        
        # Add ON CONFLICT for simple INSERTs
        if grep -q "^INSERT INTO" "migrations_postgres/$dirname/up.sql"; then
            sed -i.bak 's/\(INSERT INTO [^;]*;\)$/\1 -- Add ON CONFLICT if needed/' "migrations_postgres/$dirname/up.sql" 2>/dev/null || true
            rm -f "migrations_postgres/$dirname/up.sql.bak"
        fi
    fi
    
    # Copy down.sql if it doesn't exist
    if [ -f "$mysql_dir/down.sql" ] && [ ! -f "migrations_postgres/$dirname/down.sql" ]; then
        echo "Copying $dirname/down.sql..."
        cp "$mysql_dir/down.sql" "migrations_postgres/$dirname/down.sql"
    fi
done

echo "✅ Migration copy complete"
ls -l migrations_postgres/*/up.sql | wc -l | xargs echo "Total migrations:"
