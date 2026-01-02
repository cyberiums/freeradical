-- Drop billing tables in reverse order (child tables first)
DROP TABLE IF EXISTS billing_payments CASCADE;
DROP TABLE IF EXISTS billing_invoices CASCADE;
DROP TABLE IF EXISTS billing_subscriptions CASCADE;
DROP TABLE IF EXISTS billing_plans CASCADE;
