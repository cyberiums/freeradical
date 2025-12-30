-- Create billing_plans table
CREATE TABLE billing_plans (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    code VARCHAR(50) NOT NULL UNIQUE, -- e.g. "starter", "pro"
    price_cents INTEGER NOT NULL,
    billing_interval VARCHAR(20) NOT NULL, -- "month", "year"
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    limits JSONB, -- { "sites": 1, "visits": 25000, "storage_gb": 10 }
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create billing_subscriptions table
CREATE TABLE billing_subscriptions (
    id SERIAL PRIMARY KEY,
    tenant_id INTEGER NOT NULL REFERENCES tenants(id),
    plan_id INTEGER NOT NULL REFERENCES billing_plans(id),
    status VARCHAR(50) NOT NULL, -- "active", "past_due", "canceled", "incomplete"
    current_period_start TIMESTAMP NOT NULL,
    current_period_end TIMESTAMP NOT NULL,
    cancel_at_period_end BOOLEAN DEFAULT FALSE,
    canceled_at TIMESTAMP,
    provider_subscription_id VARCHAR(255), -- Stripe Subscription ID
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create billing_invoices table
CREATE TABLE billing_invoices (
    id SERIAL PRIMARY KEY,
    subscription_id INTEGER NOT NULL REFERENCES billing_subscriptions(id),
    amount_cents INTEGER NOT NULL,
    status VARCHAR(50) NOT NULL, -- "draft", "open", "paid", "void", "uncollectible"
    due_date TIMESTAMP NOT NULL,
    paid_at TIMESTAMP,
    line_items JSONB, -- Snapshot of what was billed
    invoice_number VARCHAR(50),
    pdf_url TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Create billing_payments table
CREATE TABLE billing_payments (
    id SERIAL PRIMARY KEY,
    invoice_id INTEGER NOT NULL REFERENCES billing_invoices(id),
    amount_cents INTEGER NOT NULL,
    provider_transaction_id VARCHAR(255), -- Stripe Charge ID
    status VARCHAR(50) NOT NULL, -- "succeeded", "pending", "failed"
    payment_method VARCHAR(50), -- "card", "bank_transfer"
    payment_date TIMESTAMP DEFAULT NOW(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_billing_subscriptions_tenant_id ON billing_subscriptions(tenant_id);
CREATE INDEX idx_billing_subscriptions_status ON billing_subscriptions(status);
CREATE INDEX idx_billing_invoices_subscription_id ON billing_invoices(subscription_id);
CREATE INDEX idx_billing_invoices_status ON billing_invoices(status);
