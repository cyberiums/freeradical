-- Create marketplace_submissions table for tracking plugin/theme submissions
CREATE TABLE marketplace_submissions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    resource_type VARCHAR(50) NOT NULL CHECK (resource_type IN ('plugin', 'theme')),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    repository_url VARCHAR(500),
    version VARCHAR(50),
    status VARCHAR(50) DEFAULT 'pending' CHECK (status IN ('pending', 'approved', 'rejected', 'in_review')),
    reviewed_by INTEGER REFERENCES users(id),
    reviewed_at TIMESTAMP,
    review_notes TEXT,
    created_at TIMESTAMP DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP DEFAULT NOW() NOT NULL
);

-- Indexes for better query performance
CREATE INDEX idx_marketplace_submissions_user_id ON marketplace_submissions(user_id);
CREATE INDEX idx_marketplace_submissions_status ON marketplace_submissions(status);
CREATE INDEX idx_marketplace_submissions_resource_type ON marketplace_submissions(resource_type);
CREATE INDEX idx_marketplace_submissions_created_at ON marketplace_submissions(created_at DESC);

-- Update trigger for updated_at
CREATE OR REPLACE FUNCTION update_marketplace_submissions_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_marketplace_submissions_updated_at
    BEFORE UPDATE ON marketplace_submissions
    FOR EACH ROW
    EXECUTE FUNCTION update_marketplace_submissions_updated_at();
