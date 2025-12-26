-- Rollback CRM tables
DROP TRIGGER IF EXISTS update_crm_notes_updated_at ON crm_notes;
DROP TRIGGER IF EXISTS update_crm_tasks_updated_at ON crm_tasks;
DROP TRIGGER IF EXISTS update_crm_campaigns_updated_at ON crm_campaigns;
DROP TRIGGER IF EXISTS update_crm_segments_updated_at ON crm_segments;
DROP TRIGGER IF EXISTS update_crm_customers_updated_at ON crm_customers;

DROP FUNCTION IF EXISTS update_updated_at_column();

DROP TABLE IF EXISTS crm_notes;
DROP TABLE IF EXISTS crm_tasks;
DROP TABLE IF EXISTS crm_campaigns;
DROP TABLE IF EXISTS crm_segment_members;
DROP TABLE IF EXISTS crm_segments;
DROP TABLE IF EXISTS crm_interactions;
DROP TABLE IF EXISTS crm_customers;
