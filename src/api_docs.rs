use utoipa::OpenApi;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "FreeRadical CMS API",
        version = "1.0.4",
        description = "FreeRadical - The fastest Rust CMS. 3-8x faster than WordPress with AI-powered content generation, advanced e-commerce, and enterprise-grade security.",
        contact(
            name = "FreeRadical Support",
            url = "https://github.com/cyberiums/freeradical"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:8000", description = "Local development server"),
        (url = "http://localhost:8001", description = "PostgreSQL instance"),
        (url = "https://api.freeradical.dev", description = "Production API")
    ),
    paths(
        // AI Provider Management
        crate::services::ai_provider_service::list_providers,
        crate::services::ai_provider_service::get_provider,
        crate::services::ai_provider_service::create_provider,
        crate::services::ai_provider_service::update_provider,
        crate::services::ai_provider_service::delete_provider,
        crate::services::ai_provider_service::test_provider,
        
        // Inventory Management
        crate::services::inventory_service::get_product_variants,
        crate::services::inventory_service::create_variant,
        crate::services::inventory_service::update_variant_stock,
        crate::services::inventory_service::get_inventory_audit_log,
        crate::services::inventory_service::delete_variant,
    ),
    components(
        schemas(
            // AI Provider schemas
            crate::models::ai_provider_models::AIProviderConfigPublic,
            crate::services::ai_provider_service::CreateProviderRequest,
            crate::services::ai_provider_service::UpdateProviderRequest,
            crate::services::ai_provider_service::TestProviderRequest,
            crate::services::ai_provider_service::TestProviderResponse,
            
            // Inventory schemas
            crate::models::inventory_models::ProductVariant,
            crate::models::inventory_models::NewProductVariant,
            crate::models::inventory_models::InventoryAuditLog,
            crate::services::inventory_service::CreateVariantRequest,
            crate::services::inventory_service::UpdateStockRequest,
        )
    ),
    tags(
        (name = "AI Providers", description = "AI provider configuration and management endpoints"),
        (name = "Inventory", description = "Product variant and stock management endpoints"),
        (name = "Users", description = "User management and authentication"),
        (name = "Pages", description = "Content page management"),
        (name = "Products", description = "E-commerce product management"),
        (name = "Orders", description = "Order processing and management"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            )
        }
    }
}
