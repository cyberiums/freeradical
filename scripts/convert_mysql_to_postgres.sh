#!/bin/bash
# Automated SQL converter: MySQL → PostgreSQL
# Converts common MySQL syntax to PostgreSQL equivalents

set -e

convert_sql_file() {
    local input_file=$1
    local output_file=$2
    
    echo "Converting $input_file → $output_file"
    
    # Read and convert SQL
    sed -E \
        -e 's/INSERT IGNORE/INSERT/g' \
        -e 's/ON DUPLICATE KEY UPDATE/ON CONFLICT DO UPDATE SET/g' \
        -e 's/\(SELECT UUID\(\)\)/gen_random_uuid()::text/g' \
        -e 's/UUID\(\)/gen_random_uuid()::text/g' \
        -e 's/BIGINT UNSIGNED/BIGINT/g' \
        -e 's/AUTO_INCREMENT//g' \
        -e 's/INT\([^E]\)/INTEGER\1/g' \
        -e 's/DATETIME/TIMESTAMP/g' \
        -e "s/COMMENT '[^']*'//g" \
        -e 's/ENGINE=[A-Za-z]+//g' \
        -e 's/DEFAULT CHARSET=[A-Za-z0-9]+//g' \
        "$input_file" > "$output_file"
    
    # Add ON CONFLICT for INSERT statements that don't have it
    sed -i.bak 's/^INSERT INTO \(.*\) VALUES/INSERT INTO \1 VALUES/g; s/VALUES \(.*\);$/VALUES \1 ON CONFLICT DO NOTHING;/g' "$output_file" 2>/dev/null || true
    rm -f "${output_file}.bak" 2>/dev/null || true
}

# Copy and convert all migration files
for dir in migrations/*/; do
    dirname=$(basename "$dir")
    
    # Skip if already exists
    if [ -d "migrations_postgres/$dirname" ]; then
        if [ -f "$dir/up.sql" ] && [ ! -f "migrations_postgres/$dirname/up.sql" ]; then
            convert_sql_file "$dir/up.sql" "migrations_postgres/$dirname/up.sql"
        fi
        
        if [ -f "$dir/down.sql" ] && [ ! -f "migrations_postgres/$dirname/down.sql" ]; then
            convert_sql_file "$dir/down.sql" "migrations_postgres/$dirname/down.sql"
        fi
    fi
done

echo "✅ SQL conversion complete"
echo "⚠️  Manual review recommended for:"
echo "   - ENUM types (convert to VARCHAR with CHECK constraints)"
echo "   - INDEX creation syntax  "
echo "   - Complex JOIN operations"
