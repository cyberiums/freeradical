-- Revert Seeding of Billing Plans
DELETE FROM billing_plans WHERE code IN (
    'site_starter_monthly', 'site_pro_monthly', 'site_growth_monthly', 'site_scale_monthly', 'site_enterprise_monthly',
    'store_startup_monthly', 'store_pro_monthly', 'store_growth_monthly', 'store_scale_monthly', 'store_enterprise_monthly'
);
