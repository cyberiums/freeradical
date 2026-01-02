-- Drop marketplace_submissions table
DROP TRIGGER IF EXISTS trigger_marketplace_submissions_updated_at ON marketplace_submissions;
DROP FUNCTION IF EXISTS update_marketplace_submissions_updated_at();
DROP TABLE IF EXISTS marketplace_submissions;
