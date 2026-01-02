use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "FreeRadical CMS API",
        version = "2.6.0",
        description = "Industrial-grade headless CMS with AI capabilities, multi-provider support, and comprehensive commerce features. Organized into Customer, Content, Commerce, Marketplace, and Internal categories.",
        contact(
            name = "FreeRadical Support",
            url = "https://freeradical.dev",
            email = "support@freeradical.dev"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:8000", description = "Local development server"),
        (url = "https://api.freeradical.dev", description = "Production API server")
    ),
    paths(
        // Authentication
        crate::controllers::user_controllers::login,
        crate::controllers::user_controllers::logout,
        crate::controllers::user_controllers::check_login,
        crate::controllers::user_controllers::setup_2fa,
        crate::controllers::user_controllers::enable_2fa,
        
        // Internal - Users
        crate::controllers::user_controllers::create_user,
        crate::controllers::user_controllers::get_user,
        crate::controllers::user_controllers::update_user,
        crate::controllers::user_controllers::delete_user,
        
        // CRM - Customers
        crate::controllers::crm_controller::list_customers,
        crate::controllers::crm_controller::get_customer_profile,
        crate::controllers::crm_controller::create_customer,
        crate::controllers::crm_controller::update_customer,
        crate::controllers::crm_controller::delete_customer,
        
        // CRM - Interactions
        crate::controllers::crm_controller::get_customer_timeline,
        crate::controllers::crm_controller::create_interaction,
        crate::controllers::crm_controller::get_interaction,
        
        // CRM - Segments
        crate::controllers::crm_controller::list_segments,
        crate::controllers::crm_controller::create_segment,
        crate::controllers::crm_controller::get_segment_members,
        
        // CRM - Campaigns
        crate::controllers::crm_controller::create_campaign,
        crate::controllers::crm_controller::list_campaigns,
        
        // CRM - Tasks  
        crate::controllers::crm_controller::create_task,
        crate::controllers::crm_controller::list_tasks,
        crate::controllers::crm_controller::update_task,
        
        // CRM - Notes
        crate::controllers::crm_controller::add_customer_note,
        crate::controllers::crm_controller::get_customer_notes,
        crate::controllers::crm_controller_public::create_customer_public,
        crate::controllers::crm_controller_public::verify_customer_email,
        
        // Content - Pages
        crate::controllers::page_controllers::create_page,
        crate::controllers::page_controllers::get_pages,
        crate::controllers::page_controllers::get_page,
        crate::controllers::page_controllers::get_page_join_modules,
        crate::controllers::page_controllers::update_page,
        crate::controllers::page_controllers::delete_page,
        
        // Commerce - Products
        crate::controllers::product_controller::list_products,
        crate::controllers::product_controller::get_product,
        crate::controllers::product_controller::create_product,
        crate::controllers::product_controller::update_product,
        crate::controllers::product_controller::delete_product,
        
        // Commerce - Orders
        crate::controllers::order_controller::list_orders,
        crate::controllers::order_controller::get_order,
        crate::controllers::order_controller::create_order,
        crate::controllers::order_controller::update_order_status,
        crate::controllers::order_controller::link_payment_to_order,
        
        // Content - Modules
        crate::controllers::module_controllers::create_module,
        crate::controllers::module_controllers::get_modules,
        crate::controllers::module_controllers::get_module,
        crate::controllers::module_controllers::update_module,
        crate::controllers::module_controllers::delete_module,
        crate::controllers::module_controllers::get_module_category,
        crate::controllers::category_controllers::create_category,
        crate::controllers::category_controllers::update_category,
        crate::controllers::category_controllers::get_category,
        crate::controllers::category_controllers::delete_category,
        
        // Marketplace - Themes
        crate::controllers::theme_controller::list_themes,
        crate::controllers::theme_controller::upload_theme,
        crate::controllers::theme_controller::activate_theme,
        
        // Marketplace - Plugins
        crate::controllers::marketplace_plugin_controller::list_plugins,
        crate::controllers::marketplace_plugin_controller::submit_plugin,
        crate::controllers::marketplace_plugin_controller::install_plugin,
        
        // System - Tenants
        crate::controllers::tenant_controller::create_tenant,
        crate::controllers::tenant_controller::list_my_tenants,
        crate::controllers::tenant_controller::invite_member,
        crate::controllers::tenant_controller::get_tenant_details,
        crate::controllers::tenant_controller::list_audit_logs,
        crate::controllers::tenant_controller::update_settings,
        
        // System - Webhooks
        crate::controllers::webhook_controller::list_webhooks,
        crate::controllers::webhook_controller::create_webhook,
        crate::controllers::webhook_controller::delete_webhook,
        
        // Commerce - Payments
        crate::controllers::payment_controller::create_payment_intent,
        crate::controllers::payment_controller::get_payment_intent,
        crate::controllers::payment_controller::list_payment_handlers,
        
        // Customer - OAuth
        crate::controllers::oauth_controller::google_login,
        crate::controllers::oauth_callback_controller::google_callback,
        crate::controllers::oauth_callback_controller::github_callback,
        crate::controllers::oauth_callback_controller::disconnect_provider,
        
        // System - Site Settings
        crate::controllers::site_controller::list_sites,
        crate::controllers::site_controller::create_site,
        crate::controllers::site_controller::validate_cname,
        crate::controllers::site_controller::get_site,
        
        // System - Surveys
        crate::controllers::survey_controller::create_survey,
        crate::controllers::survey_controller::list_surveys,
        crate::controllers::survey_controller::get_survey,
        crate::controllers::survey_controller::submit_response,
        crate::controllers::survey_controller::get_results,
        crate::controllers::survey_controller::add_question,
        
        // System - SSO
        crate::controllers::tenant_sso_controller::update_config,
        crate::controllers::tenant_sso_controller::login,
        crate::controllers::tenant_sso_controller::oidc_callback,
        crate::controllers::tenant_sso_controller::saml_acs,
        crate::controllers::tenant_sso_controller::metadata,
        
        // System - Verification Settings
        crate::controllers::verification_settings_controller::get_verification_settings,
        crate::controllers::verification_settings_controller::update_verification_settings,
        crate::controllers::verification_settings_controller::delete_verification_settings,
        
        // Commerce - Billing
        crate::controllers::billing_controller::get_all_plans,
        crate::controllers::billing_controller::subscribe,
        crate::controllers::billing_controller::cancel_subscription,
        crate::controllers::billing_controller::get_my_subscription,
        crate::controllers::billing_controller::get_billing_view,
        crate::controllers::billing_controller::get_plans_view,
        
        // Content - Media
        crate::controllers::media_controller::upload_media,
        crate::controllers::media_controller::list_media,
        crate::controllers::media_controller::get_media,
        crate::controllers::media_controller::delete_media,
        
        // Internal - Dashboard
        crate::controllers::dashboard_controller::dashboard_summary,
        crate::controllers::dashboard_controller::analytics_summary,
        crate::controllers::dashboard_controller::seo_health,
        crate::controllers::dashboard_controller::top_pages,
        
        // Internal - Metrics
        crate::controllers::metrics_controller::get_metrics,
        crate::controllers::metrics_controller::health_check,
        
        // Internal - Backup
        crate::controllers::backup_controller::list_backups,
        crate::controllers::backup_controller::create_backup,
        
        // Internal - SEO
        crate::controllers::seo_controller::audit_url,
        
        // Content - SEO
        crate::controllers::sitemap_controller::sitemap,
        crate::controllers::image_sitemap_controller::image_sitemap,
        crate::controllers::robots_controller::robots,
        
        // Content - Search
        crate::controllers::search_controller::search_content,
        
        // Content - i18n
        crate::controllers::i18n_controller::list_languages,
        crate::controllers::i18n_controller::create_language,
        crate::controllers::i18n_controller::get_translation,
        
        // Content - Relationships
        crate::controllers::relationship_controller::create_relationship,
        crate::controllers::relationship_controller::get_related,
        crate::controllers::relationship_controller::delete_relationship,
        
        // Content - Revisions
        crate::controllers::revision_controller::list_revisions,
        crate::controllers::revision_controller::get_revision,
        crate::controllers::revision_controller::rollback_revision,
        
        // Customer - CRM (AI)
        crate::services::recommendation_service::get_related_content,
        crate::services::recommendation_service::get_trending,
        crate::services::ai_content_service::analyze_sentiment,
    ),
    components(schemas(
        // Auth models
        crate::models::user_models::LoginRequest,
        crate::models::user_models::User,
        
        // CRM models
        crate::models::crm_models::CrmCustomer,
        crate::models::crm_models::CrmInteraction,
        crate::models::crm_models::CrmSegment,
        crate::models::crm_models::CrmCampaign,
        crate::models::crm_models::CrmTask,
        crate::models::crm_models::CrmNote,
        
        // CRM request DTOs
        crate::controllers::crm_controller::CustomerFilters,
        crate::controllers::crm_controller::CreateCustomerRequest,
        crate::controllers::crm_controller::UpdateCustomerRequest,
        crate::controllers::crm_controller::CreateInteractionRequest,
        crate::controllers::crm_controller::CreateSegmentRequest,
        crate::controllers::crm_controller::CreateCampaignRequest,
        crate::controllers::crm_controller::CreateTaskRequest,
        crate::controllers::crm_controller::CreateNoteRequest,
        crate::controllers::crm_controller::TaskFilters,
        
        // CRM response DTOs
        crate::controllers::crm_controller::CustomerDetailResponse,
        
        // Content - Page models
        crate::models::page_models::Page,
        crate::models::page_models::MutPage,
        crate::models::page_models::PageDTO,
        crate::models::page_models::PageModuleDTO,
        
        // Commerce - Product/Order models
        crate::models::commerce_models::Product,
        crate::models::commerce_models::NewProduct,
        crate::models::commerce_models::Order,
        crate::models::commerce_models::NewOrder,
        crate::models::commerce_models::OrderItem,
        crate::models::commerce_models::NewOrderItem,
        
        // Content - Module models
        crate::models::module_models::Module,
        crate::models::module_models::MutModule,
        crate::models::module_models::ModuleCategory,
        crate::models::module_models::MutCategory,
        crate::models::module_models::CategoryDTO,
        crate::models::module_models::FieldsDTO,
        
        // Marketplace - Theme models
        crate::models::theme_models::Theme,
        crate::models::theme_models::NewTheme,
        
        // Marketplace - Plugin models
        crate::models::marketplace_plugin_models::MarketplacePlugin,
        crate::models::marketplace_plugin_models::NewMarketplacePlugin,
        crate::models::marketplace_plugin_models::TenantPlugin,
        crate::models::marketplace_plugin_models::NewTenantPlugin,
        
        // System - Tenant models
        crate::models::tenant_models::Tenant,
        crate::models::tenant_models::NewTenant,
        crate::models::tenant_models::TenantMember,
        crate::models::tenant_models::NewTenantMember,
        
        // System - Webhook models
        crate::models::webhook_models::TenantWebhook,
        crate::models::webhook_models::NewTenantWebhook,
        
        // Commerce - Payment request DTOs
        crate::controllers::payment_controller::CreatePaymentRequest,
        crate::controllers::payment_controller::GetPaymentRequest,
        
        // Customer - OAuth DTOs
        crate::controllers::oauth_callback_controller::OAuthCallbackQuery,
        crate::controllers::oauth_callback_controller::UserProfile,
    )),
    tags(
        // ==================== CUSTOMER CATEGORY ====================
        (
            name = "Customer - Authentication", 
            description = "🔐 User authentication, OAuth (Google), and Single Sign-On (SAML/OIDC)",
            external_docs(
                url = "https://freeradical.dev/docs/authentication",
                description = "Authentication Guide"
            )
        ),
        (
            name = "Customer - CRM", 
            description = "👥 Customer relationship management: profiles, interactions, segments, campaigns, and tasks"
        ),
        (
            name = "Customer - CRM (AI)", 
            description = "🤖 AI-powered customer insights: sentiment analysis, behavioral predictions, and personalized recommendations"
        ),
        
        // ==================== CONTENT CATEGORY ====================
        (
            name = "Content - Pages", 
            description = "📄 Page management: CRUD operations, revisions, and publishing workflows"
        ),
        (
            name = "Content - Modules", 
            description = "🧩 Content modules and categories: reusable components and content organization"
        ),
        (
            name = "Content - Media", 
            description = "🖼️ Media library: file uploads, image processing, and asset management"
        ),
        (
            name = "Content - SEO", 
            description = "🔍 SEO optimization tools: sitemaps, robots.txt, meta tags, and audit reports"
        ),
        (
            name = "Content - AI", 
            description = "✨ AI-powered content tools: generation, metadata automation, keyword extraction, and semantic search"
        ),
        
        // ==================== COMMERCE CATEGORY ====================
        (
            name = "Commerce - Products", 
            description = "📦 Product catalog: catalog management, variants, pricing, and inventory tracking"
        ),
        (
            name = "Commerce - Orders", 
            description = "🛒 Order processing: order lifecycle, fulfillment, and status tracking"
        ),
        (
            name = "Commerce - Payments", 
            description = "💳 Payment processing: multi-provider support (Stripe, PayPal, Square), payment intents, and webhooks"
        ),
        (
            name = "Commerce - Inventory", 
            description = "📊 Stock management: inventory levels, variants, audit logs, and low-stock alerts"
        ),
        (
            name = "Commerce - AI", 
            description = "🎯 AI commerce tools: fraud detection, demand forecasting, and dynamic pricing"
        ),
        
        // ==================== MARKETPLACE CATEGORY ====================
        (
            name = "Marketplace - Plugins", 
            description = "🔌 Plugin marketplace: discovery, installation, and management of plugins"
        ),
        (
            name = "Marketplace - Themes", 
            description = "🎨 Theme marketplace: theme installation, activation, and customization"
        ),
        (
            name = "Marketplace - Submissions", 
            description = "📤 Developer submissions: plugin and theme submission workflow for developers"
        ),
        
        // ==================== INTERNAL CATEGORY ====================
        (
            name = "Internal - Tenants", 
            description = "🏢 Multi-tenancy: tenant management, member invitations, SSO configuration, and settings"
        ),
        (
            name = "Internal - Billing", 
            description = "💰 Subscriptions and invoicing: billing plans, subscriptions, invoices, and payment tracking"
        ),
        (
            name = "Internal - Admin", 
            description = "⚙️ Admin dashboard: analytics, reports, and system overview"
        ),
        (
            name = "Internal - System", 
            description = "🔧 System management: backups, webhooks, surveys, and site configuration"
        ),
        (
            name = "Internal - AI Providers", 
            description = "🤖 AI provider configuration (Admin only): manage AI service providers, API keys, and usage limits"
        )
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

/// Security scheme configuration for JWT Bearer authentication
pub struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some("JWT token obtained from authentication endpoints. Format: `Bearer <token>`".to_string()))
                        .build(),
                ),
            )
        }
    }
}
