-- Add advanced inventory management fields to products table

ALTER TABLE products
ADD COLUMN stock_quantity INT NOT NULL DEFAULT 0,
ADD COLUMN low_stock_threshold INT DEFAULT 10,
ADD COLUMN stock_status VARCHAR(20) DEFAULT 'in_stock' CHECK (stock_status IN ('in_stock', 'low_stock', 'out_of_stock')),
ADD COLUMN track_inventory BOOLEAN DEFAULT TRUE,
ADD COLUMN allow_backorder BOOLEAN DEFAULT FALSE,
ADD COLUMN backorder_limit INT DEFAULT NULL;

-- Update stock_status based on current stock (for existing products)
UPDATE products 
SET stock_status = CASE
    WHEN stock_quantity = 0 THEN 'out_of_stock'
    WHEN stock_quantity <= low_stock_threshold THEN 'low_stock'
    ELSE 'in_stock'
END;

-- Create product variants table
CREATE TABLE product_variants (
    id SERIAL PRIMARY KEY,
    uuid VARCHAR(36) NOT NULL UNIQUE,
    product_id INT NOT NULL,
    sku VARCHAR(100) UNIQUE,
    variant_name VARCHAR(255) NOT NULL,
    price DECIMAL(10, 2),
    stock_quantity INT DEFAULT 0,
    weight DECIMAL(8, 2),
    attributes JSONB,
    image_url VARCHAR(500),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE
);

CREATE INDEX idx_product_variants_product ON product_variants(product_id);
CREATE INDEX idx_product_variants_sku ON product_variants(sku);
CREATE INDEX idx_product_variants_active ON product_variants(is_active);

-- Create inventory audit log table
CREATE TABLE inventory_audit_log (
    id SERIAL PRIMARY KEY,
    product_id INT,
    variant_id INT NULL,
    user_id INT NULL,
    order_id INT NULL,
    change_type VARCHAR(20) NOT NULL CHECK (change_type IN ('adjustment', 'sale', 'restock', 'return', 'damaged')),
    quantity_before INT NOT NULL,
    quantity_after INT NOT NULL,
    quantity_change INT NOT NULL,
    reason VARCHAR(500),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
    FOREIGN KEY (variant_id) REFERENCES product_variants(id) ON DELETE SET NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE SET NULL
);

CREATE INDEX idx_inventory_audit_product ON inventory_audit_log(product_id);
CREATE INDEX idx_inventory_audit_created ON inventory_audit_log(created_at);
