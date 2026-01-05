-- Create email_templates table for multi-tenant email customization
CREATE TABLE IF NOT EXISTS email_templates (
    id SERIAL PRIMARY KEY,
    tenant_id INTEGER,
    template_key VARCHAR(100) NOT NULL,
    subject VARCHAR(255) NOT NULL,
    body_template TEXT NOT NULL,
    template_type VARCHAR(50) NOT NULL DEFAULT 'handlebars',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id, template_key),
    FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE
);

-- Create index for faster lookups
CREATE INDEX idx_email_templates_tenant_key ON email_templates(tenant_id, template_key);
CREATE INDEX idx_email_templates_active ON email_templates(is_active);

-- Insert all 19 default email templates (tenant_id NULL = global defaults)
-- These will be automatically available in production and can be overridden per tenant

-- 1. SIGNUP & ONBOARDING (3 templates)
INSERT INTO email_templates (tenant_id, template_key, subject, body_template, template_type) VALUES
(NULL, 'welcome', 'Welcome to {{app_name}}! Your account is ready 🎉', 'auth/welcome', 'handlebars'),
(NULL, 'onboarding_day3', '3 quick tips to get the most out of {{app_name}}', 'onboarding/day3_tips', 'handlebars'),
(NULL, 'onboarding_week1', 'Here''s what you can do next with {{app_name}}', 'onboarding/week1_next_steps', 'handlebars')
ON CONFLICT (tenant_id, template_key) DO NOTHING;

-- 2. SUBSCRIPTION & LIMITS (3 templates)
INSERT INTO email_templates (tenant_id, template_key, subject, body_template, template_type) VALUES
(NULL, 'limit_warning_80', 'You''re approaching your {{resource_type}} limit ({{usage_percentage}}%)', 'billing/limit_warning_80', 'handlebars'),
(NULL, 'limit_reached', '⚠️ You''ve reached your {{resource_type}} limit', 'billing/limit_reached', 'handlebars'),
(NULL, 'upgrade_success', 'Welcome to {{new_plan_name}}! Here''s what''s new', 'billing/upgrade_success', 'handlebars')
ON CONFLICT (tenant_id, template_key) DO NOTHING;

-- 3. BILLING & PAYMENTS (4 templates)
INSERT INTO email_templates (tenant_id, template_key, subject, body_template, template_type) VALUES
(NULL, 'payment_success', 'Payment received - Thank you! 💳', 'billing/payment_success', 'handlebars'),
(NULL, 'payment_failed', 'Action required: Payment unsuccessful', 'billing/payment_failed', 'handlebars'),
(NULL, 'invoice_ready', 'Your invoice is ready', 'billing/invoice_ready', 'handlebars'),
(NULL, 'trial_ending', 'Your trial ends in {{days_remaining}} days', 'billing/trial_ending', 'handlebars')
ON CONFLICT (tenant_id, template_key) DO NOTHING;

-- 4. ACTIVITY & ENGAGEMENT (3 templates)
INSERT INTO email_templates (tenant_id, template_key, subject, body_template, template_type) VALUES
(NULL, 'weekly_summary', 'Your weekly {{app_name}} summary 📊', 'engagement/weekly_summary', 'handlebars'),
(NULL, 'inactive_reminder', 'We miss you! Here''s what''s new', 'engagement/inactive_reminder', 'handlebars'),
(NULL, 'milestone_achieved', '🎉 Milestone reached: {{milestone_name}}', 'engagement/milestone', 'handlebars')
ON CONFLICT (tenant_id, template_key) DO NOTHING;

-- 5. FEATURE ADOPTION (3 templates)
INSERT INTO email_templates (tenant_id, template_key, subject, body_template, template_type) VALUES
(NULL, 'new_feature', 'New feature: {{feature_name}} is now available!', 'features/new_feature', 'handlebars'),
(NULL, 'feature_tip', 'Pro tip: Get more from {{feature_name}}', 'features/feature_tip', 'handlebars'),
(NULL, 'update_available', 'System update: What''s new in {{version}}', 'features/update_available', 'handlebars')
ON CONFLICT (tenant_id, template_key) DO NOTHING;

-- 6. SECURITY & ACCOUNT (3 templates)
INSERT INTO email_templates (tenant_id, template_key, subject, body_template, template_type) VALUES
(NULL, 'password_reset', 'Reset your password', 'auth/password_reset', 'handlebars'),
(NULL, 'email_verification', 'Verify your email address', 'auth/email_verification', 'handlebars'),
(NULL, 'security_alert', '🔒 Security alert: New login detected', 'auth/security_alert', 'handlebars')
ON CONFLICT (tenant_id, template_key) DO NOTHING;

COMMENT ON TABLE email_templates IS 'Stores customizable email templates per tenant';
COMMENT ON COLUMN email_templates.tenant_id IS 'NULL for global defaults, specific ID for tenant overrides';
COMMENT ON COLUMN email_templates.template_key IS 'Unique identifier for template type (e.g., welcome, password_reset)';
COMMENT ON COLUMN email_templates.body_template IS 'Path to handlebars template file or raw HTML';
