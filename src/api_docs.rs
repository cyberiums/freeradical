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
        // Endpoints will be added progressively
        // Phase 2-6: Add controller paths here
    ),
    components(schemas(
        // Schemas will be added progressively
        // Models with #[derive(ToSchema)] will appear here
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
                        .description("JWT token obtained from authentication endpoints. Format: `Bearer <token>`")
                        .build(),
                ),
            )
        }
    }
}
