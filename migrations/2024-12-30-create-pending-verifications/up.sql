-- Migration: Create pending_verifications table
-- Description: Stores pending email verifications with configurable TTL

CREATE TABLE IF NOT EXISTS pending_verifications (
    id SERIAL PRIMARY KEY,
    uuid UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
    verification_type VARCHAR(50) NOT NULL,
    email VARCHAR(255) NOT NULL,
    verification_token VARCHAR(255) UNIQUE NOT NULL,
    payload JSONB NOT NULL,
    tenant_id INTEGER REFERENCES tenants(id) ON DELETE CASCADE,
    verified BOOLEAN DEFAULT FALSE,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    verified_at TIMESTAMP,
    
    -- Indexes for performance
    CONSTRAINT idx_verification_token UNIQUE (verification_token)
);

CREATE INDEX IF NOT EXISTS idx_pending_verifications_expires ON pending_verifications(expires_at);
CREATE INDEX IF NOT EXISTS idx_pending_verifications_type_email ON pending_verifications(verification_type, email);
CREATE INDEX IF NOT EXISTS idx_pending_verifications_verified ON pending_verifications(verified);

-- Add comment for documentation
COMMENT ON TABLE pending_verifications IS 'Stores pending email verifications for public APIs with auto-expiration';
COMMENT ON COLUMN pending_verifications.verification_type IS 'Type of verification: crm_customer, user_registration, form_submission, etc.';
COMMENT ON COLUMN pending_verifications.payload IS 'Original request body stored as JSON';
COMMENT ON COLUMN pending_verifications.expires_at IS 'When this verification expires (based on TTL settings)';
