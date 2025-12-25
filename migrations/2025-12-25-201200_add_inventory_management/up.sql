-- Add advanced inventory management fields to products table

ALTER TABLE products
ADD COLUMN stock_quantity INT NOT NULL DEFAULT 0,
ADD COLUMN low_stock_threshold INT DEFAULT 10,
ADD COLUMN stock_status ENUM('in_stock', 'low_stock', 'out_of_stock') DEFAULT 'in_stock',
ADD COLUMN track_inventory BOOLEAN DEFAULT TRUE,
ADD COLUMN allow_backorder BOOLEAN DEFAULT FALSE,
ADD COLUMN backorder_limit INT DEFAULT NULL;

-- Update stock_status based on current stock (for existing products)
UPDATE products 
SET stock_status = CASE
    WHEN stock_quantity = 0 THEN 'out_of_stock'
    WHEN stock_quantity \u003c= low_stock_threshold THEN 'low_stock'
    ELSE 'in_stock'
END;

-- Create product variants table
CREATE TABLE product_variants (
    id INT AUTO_INCREMENT PRIMARY KEY,
    uuid VARCHAR(36) NOT NULL UNIQUE,
    product_id INT NOT NULL,
    sku VARCHAR(100) UNIQUE,
    variant_name VARCHAR(255) NOT NULL,
    price DECIMAL(10, 2),
    stock_quantity INT DEFAULT 0,
    weight DECIMAL(8, 2),
    attributes JSON,
    image_url VARCHAR(500),
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
    INDEX idx_product_variants_product (product_id),
    INDEX idx_product_variants_sku (sku),
    INDEX idx_product_variants_active (is_active)
);

-- Create inventory audit log table
CREATE TABLE inventory_audit_log (
    id INT AUTO_INCREMENT PRIMARY KEY,
    product_id INT,
    variant_id INT NULL,
    user_id INT NULL,
    order_id INT NULL,
    change_type ENUM('adjustment', 'sale', 'restock', 'return', 'damaged') NOT NULL,
    quantity_before INT NOT NULL,
    quantity_after INT NOT NULL,
    quantity_change INT NOT NULL,
    reason VARCHAR(500),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
    FOREIGN KEY (variant_id) REFERENCES product_variants(id) ON DELETE SET NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE SET NULL,
    INDEX idx_inventory_audit_product (product_id),
    INDEX idx_inventory_audit_created (created_at)
);
