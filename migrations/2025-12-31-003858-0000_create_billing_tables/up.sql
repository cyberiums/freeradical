-- Create billing_plans table
CREATE TABLE billing_plans (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    code VARCHAR(100) NOT NULL UNIQUE,
    price_cents INTEGER NOT NULL,
    billing_interval VARCHAR(50) NOT NULL,  -- 'month', 'year'
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    limits JSONB,  -- {"pages": 100, "users": 5, "storage_gb": 10}
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Create billing_subscriptions table
CREATE TABLE billing_subscriptions (
    id SERIAL PRIMARY KEY,
    tenant_id INTEGER NOT NULL REFERENCES tenants(id),
    plan_id INTEGER NOT NULL REFERENCES billing_plans(id),
    status VARCHAR(50) NOT NULL,  -- 'active', 'past_due', 'canceled'
    current_period_start TIMESTAMP NOT NULL,
    current_period_end TIMESTAMP NOT NULL,
    cancel_at_period_end BOOLEAN DEFAULT false,
    canceled_at TIMESTAMP,
    provider_subscription_id VARCHAR(255),  -- Stripe subscription ID
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Create billing_invoices table
CREATE TABLE billing_invoices (
    id SERIAL PRIMARY KEY,
    subscription_id INTEGER NOT NULL REFERENCES billing_subscriptions(id),
    amount_cents INTEGER NOT NULL,
    status VARCHAR(50) NOT NULL,  -- 'draft', 'open', 'paid', 'void'
    due_date TIMESTAMP NOT NULL,
    paid_at TIMESTAMP,
    line_items JSONB,
    invoice_number VARCHAR(100),
    pdf_url VARCHAR(500),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Create billing_payments table
CREATE TABLE billing_payments (
    id SERIAL PRIMARY KEY,
    invoice_id INTEGER NOT NULL REFERENCES billing_invoices(id),
    amount_cents INTEGER NOT NULL,
    provider_transaction_id VARCHAR(255),  -- Stripe payment intent ID
    status VARCHAR(50) NOT NULL,  -- 'succeeded', 'pending', 'failed'
    payment_method VARCHAR(100),
    payment_date TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_billing_subscriptions_tenant ON billing_subscriptions(tenant_id);
CREATE INDEX idx_billing_subscriptions_plan ON billing_subscriptions(plan_id);
CREATE INDEX idx_billing_invoices_subscription ON billing_invoices(subscription_id);
CREATE INDEX idx_billing_payments_invoice ON billing_payments(invoice_id);
