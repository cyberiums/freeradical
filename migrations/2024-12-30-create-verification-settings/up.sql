-- Migration: Create verification_settings table
-- Description: Configurable TTL and settings for email verification

CREATE TABLE IF NOT EXISTS verification_settings (
    id SERIAL PRIMARY KEY,
    tenant_id INTEGER REFERENCES tenants(id) ON DELETE CASCADE,
    verification_type VARCHAR(50) NOT NULL,
    ttl_hours INTEGER DEFAULT 12 CHECK (ttl_hours > 0 AND ttl_hours <= 168),
    enabled BOOLEAN DEFAULT TRUE,
    email_template TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    
    -- One config per tenant per type
    UNIQUE(tenant_id, verification_type)
);

-- Insert default settings (NULL tenant_id = global default)
INSERT INTO verification_settings (tenant_id, verification_type, ttl_hours, enabled)
VALUES 
    (NULL, 'crm_customer', 12, TRUE),
    (NULL, 'user_registration', 24, TRUE),
    (NULL, 'form_submission', 12, TRUE)
ON CONFLICT DO NOTHING;

-- Add comments
COMMENT ON TABLE verification_settings IS 'Configurable settings for email verification by type and tenant';
COMMENT ON COLUMN verification_settings.tenant_id IS 'NULL = global default, otherwise tenant-specific override';
COMMENT ON COLUMN verification_settings.ttl_hours IS 'Time to live in hours (1-168, max 7 days)';
