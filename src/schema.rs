// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "pages_status"))]
    pub struct PagesStatus;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tsvector", schema = "pg_catalog"))]
    pub struct Tsvector;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "vector"))]
    pub struct Vector;
}

diesel::table! {
    ai_generated_content (id) {
        id -> Int8,
        #[max_length = 255]
        page_uuid -> Nullable<Varchar>,
        #[max_length = 50]
        content_type -> Varchar,
        prompt_used -> Nullable<Text>,
        generated_text -> Text,
        #[max_length = 100]
        model_name -> Nullable<Varchar>,
        #[max_length = 50]
        provider_type -> Nullable<Varchar>,
        tokens_used -> Nullable<Int4>,
        quality_score -> Nullable<Numeric>,
        was_accepted -> Nullable<Bool>,
        generated_by -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    ai_key_rotation_history (id) {
        id -> Int4,
        provider_key_id -> Int4,
        rotated_at -> Timestamp,
        #[max_length = 255]
        reason -> Nullable<Varchar>,
        rotated_by -> Nullable<Int4>,
        #[max_length = 64]
        old_key_hash -> Nullable<Varchar>,
    }
}

diesel::table! {
    ai_provider_configs (id) {
        id -> Int4,
        #[max_length = 50]
        provider_type -> Varchar,
        api_key_encrypted -> Text,
        #[max_length = 100]
        model_name -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        daily_token_limit -> Nullable<Int4>,
        monthly_budget_cents -> Nullable<Int4>,
        created_by -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    ai_provider_keys (id) {
        id -> Int4,
        #[max_length = 50]
        provider_name -> Varchar,
        #[max_length = 100]
        key_name -> Varchar,
        encrypted_key -> Text,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        rotated_at -> Nullable<Timestamp>,
        last_used_at -> Nullable<Timestamp>,
        request_count -> Int4,
        token_count -> Int8,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    ai_usage_log (id) {
        id -> Int8,
        user_id -> Nullable<Int4>,
        #[max_length = 50]
        operation -> Varchar,
        #[max_length = 50]
        provider_type -> Nullable<Varchar>,
        tokens_used -> Nullable<Int4>,
        cost_cents -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    analytics_events (id) {
        id -> Int8,
        #[max_length = 50]
        event_type -> Varchar,
        #[max_length = 255]
        page_uuid -> Nullable<Varchar>,
        user_id -> Nullable<Int4>,
        #[max_length = 255]
        session_id -> Nullable<Varchar>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        referer -> Nullable<Text>,
        metadata -> Nullable<Json>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    analytics_summary (id) {
        id -> Int4,
        #[max_length = 500]
        page_url -> Varchar,
        date -> Date,
        view_count -> Nullable<Int4>,
        unique_visitors -> Nullable<Int4>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    backups (id) {
        id -> Int4,
        #[max_length = 255]
        uuid -> Varchar,
        #[sql_name = "type"]
        #[max_length = 50]
        type_ -> Varchar,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        #[max_length = 500]
        file_path -> Nullable<Varchar>,
        file_size -> Nullable<Int8>,
        #[max_length = 100]
        storage_location -> Nullable<Varchar>,
        metadata -> Nullable<Json>,
        created_at -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Vector;

    content_embeddings (id) {
        id -> Int8,
        page_id -> Nullable<Int4>,
        embedding_vector -> Nullable<Vector>,
        #[max_length = 100]
        model_name -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    content_relationships (id) {
        id -> Int8,
        #[max_length = 20]
        source_type -> Varchar,
        #[max_length = 255]
        source_id -> Varchar,
        #[max_length = 20]
        target_type -> Varchar,
        #[max_length = 255]
        target_id -> Varchar,
        #[max_length = 50]
        relationship_type -> Nullable<Varchar>,
        metadata -> Nullable<Json>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    crm_campaigns (id) {
        id -> Int4,
        #[max_length = 200]
        name -> Varchar,
        #[max_length = 50]
        campaign_type -> Varchar,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        segment_id -> Nullable<Int4>,
        target_customer_count -> Nullable<Int4>,
        #[max_length = 255]
        subject -> Nullable<Varchar>,
        content -> Nullable<Text>,
        template_id -> Nullable<Int4>,
        scheduled_at -> Nullable<Timestamp>,
        started_at -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
        sent_count -> Nullable<Int4>,
        delivered_count -> Nullable<Int4>,
        opened_count -> Nullable<Int4>,
        clicked_count -> Nullable<Int4>,
        converted_count -> Nullable<Int4>,
        revenue_generated -> Nullable<Numeric>,
        created_by -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    crm_customers (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 50]
        lifecycle_stage -> Varchar,
        customer_since -> Nullable<Timestamp>,
        last_purchase_date -> Nullable<Timestamp>,
        rfm_recency_score -> Nullable<Int4>,
        rfm_frequency_score -> Nullable<Int4>,
        rfm_monetary_score -> Nullable<Int4>,
        rfm_total_score -> Nullable<Int4>,
        total_orders -> Nullable<Int4>,
        total_revenue -> Nullable<Numeric>,
        average_order_value -> Nullable<Numeric>,
        customer_lifetime_value -> Nullable<Numeric>,
        last_interaction_date -> Nullable<Timestamp>,
        interaction_count -> Nullable<Int4>,
        email_open_rate -> Nullable<Numeric>,
        email_click_rate -> Nullable<Numeric>,
        health_score -> Nullable<Int4>,
        #[max_length = 20]
        churn_risk -> Nullable<Varchar>,
        primary_segment_id -> Nullable<Int4>,
        tags -> Nullable<Array<Nullable<Text>>>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Tsvector;

    crm_interactions (id) {
        id -> Int4,
        customer_id -> Int4,
        #[max_length = 50]
        interaction_type -> Varchar,
        #[max_length = 50]
        interaction_channel -> Nullable<Varchar>,
        #[max_length = 255]
        subject -> Nullable<Varchar>,
        description -> Nullable<Text>,
        #[max_length = 100]
        outcome -> Nullable<Varchar>,
        order_id -> Nullable<Int4>,
        #[max_length = 50]
        related_entity_type -> Nullable<Varchar>,
        related_entity_id -> Nullable<Int4>,
        created_by -> Nullable<Int4>,
        created_at -> Timestamp,
        search_vector -> Nullable<Tsvector>,
    }
}

diesel::table! {
    crm_notes (id) {
        id -> Int4,
        customer_id -> Int4,
        note_text -> Text,
        is_pinned -> Nullable<Bool>,
        created_by -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    crm_segment_members (segment_id, customer_id) {
        segment_id -> Int4,
        customer_id -> Int4,
        added_at -> Timestamp,
    }
}

diesel::table! {
    crm_segments (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        criteria -> Jsonb,
        is_dynamic -> Nullable<Bool>,
        customer_count -> Nullable<Int4>,
        last_calculated_at -> Nullable<Timestamp>,
        created_by -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    crm_tasks (id) {
        id -> Int4,
        customer_id -> Nullable<Int4>,
        #[max_length = 200]
        title -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 50]
        task_type -> Nullable<Varchar>,
        #[max_length = 20]
        priority -> Nullable<Varchar>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        due_date -> Nullable<Timestamp>,
        completed_at -> Nullable<Timestamp>,
        assigned_to -> Nullable<Int4>,
        created_by -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    inventory_audit_log (id) {
        id -> Int4,
        product_id -> Nullable<Int4>,
        variant_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        order_id -> Nullable<Int4>,
        #[max_length = 30]
        change_type -> Varchar,
        quantity_before -> Nullable<Int4>,
        quantity_after -> Nullable<Int4>,
        quantity_change -> Nullable<Int4>,
        #[max_length = 500]
        reason -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    languages (id) {
        id -> Int4,
        #[max_length = 10]
        code -> Varchar,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        native_name -> Nullable<Varchar>,
        is_default -> Nullable<Bool>,
        is_rtl -> Nullable<Bool>,
        enabled -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    media (id) {
        id -> Int4,
        #[max_length = 36]
        uuid -> Varchar,
        #[max_length = 255]
        filename -> Varchar,
        #[max_length = 255]
        original_filename -> Varchar,
        file_path -> Text,
        #[max_length = 127]
        mime_type -> Varchar,
        file_size -> Int8,
        width -> Nullable<Int4>,
        height -> Nullable<Int4>,
        alt_text -> Nullable<Text>,
        tenant_id -> Nullable<Int4>,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        description -> Nullable<Text>,
        tags -> Nullable<Array<Nullable<Text>>>,
        uploaded_by -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    module_category (uuid) {
        #[max_length = 255]
        uuid -> Varchar,
        #[max_length = 255]
        page_uuid -> Varchar,
        #[max_length = 255]
        title -> Varchar,
    }
}

diesel::table! {
    module_translations (id) {
        id -> Int4,
        module_id -> Int4,
        language_id -> Int4,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        content -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    modules (uuid) {
        #[max_length = 255]
        uuid -> Varchar,
        #[max_length = 255]
        page_uuid -> Varchar,
        #[max_length = 255]
        category_uuid -> Nullable<Varchar>,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        #[max_length = 30]
        field_type -> Nullable<Varchar>,
        field_config -> Nullable<Text>,
        validation_rules -> Nullable<Text>,
    }
}

diesel::table! {
    oauth_providers (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 255]
        client_id -> Varchar,
        #[max_length = 255]
        client_secret -> Varchar,
        enabled -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    order_items (id) {
        id -> Int8,
        order_id -> Int8,
        product_id -> Int8,
        quantity -> Int4,
        price_cents -> Int8,
    }
}

diesel::table! {
    orders (id) {
        id -> Int8,
        #[max_length = 36]
        uuid -> Varchar,
        #[max_length = 36]
        user_uuid -> Varchar,
        #[max_length = 50]
        status -> Varchar,
        total_amount_cents -> Int8,
        tenant_id -> Nullable<Int4>,
        #[max_length = 50]
        payment_status -> Nullable<Varchar>,
        #[max_length = 50]
        payment_provider -> Nullable<Varchar>,
        #[max_length = 255]
        payment_intent_id -> Nullable<Varchar>,
        metadata -> Nullable<Jsonb>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    page_revisions (id) {
        id -> Int8,
        #[max_length = 36]
        page_uuid -> Varchar,
        revision_number -> Int4,
        #[max_length = 255]
        page_title -> Varchar,
        #[max_length = 500]
        page_url -> Varchar,
        page_content -> Nullable<Text>,
        #[max_length = 70]
        meta_title -> Nullable<Varchar>,
        #[max_length = 160]
        meta_description -> Nullable<Varchar>,
        #[max_length = 255]
        meta_keywords -> Nullable<Varchar>,
        #[max_length = 500]
        canonical_url -> Nullable<Varchar>,
        full_snapshot -> Text,
        #[max_length = 500]
        change_summary -> Nullable<Varchar>,
        changed_by_user_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    page_translations (id) {
        id -> Int4,
        page_id -> Int4,
        language_id -> Int4,
        #[max_length = 255]
        page_title -> Nullable<Varchar>,
        page_content -> Nullable<Text>,
        #[max_length = 255]
        page_url -> Nullable<Varchar>,
        #[max_length = 255]
        meta_title -> Nullable<Varchar>,
        meta_description -> Nullable<Text>,
        #[max_length = 255]
        og_title -> Nullable<Varchar>,
        og_description -> Nullable<Text>,
        #[max_length = 255]
        twitter_title -> Nullable<Varchar>,
        twitter_description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    page_views (id) {
        id -> Int4,
        page_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        user_agent -> Nullable<Text>,
        referer -> Nullable<Text>,
        #[max_length = 2]
        country_code -> Nullable<Varchar>,
        #[max_length = 100]
        city -> Nullable<Varchar>,
        viewed_at -> Timestamp,
        #[max_length = 255]
        session_id -> Nullable<Varchar>,
        duration_seconds -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PagesStatus;

    pages (uuid) {
        #[max_length = 255]
        uuid -> Varchar,
        #[max_length = 500]
        page_name -> Varchar,
        #[max_length = 255]
        page_url -> Varchar,
        #[max_length = 500]
        page_title -> Varchar,
        time_created -> Timestamp,
        #[max_length = 70]
        meta_title -> Nullable<Varchar>,
        #[max_length = 160]
        meta_description -> Nullable<Varchar>,
        #[max_length = 255]
        meta_keywords -> Nullable<Varchar>,
        #[max_length = 500]
        canonical_url -> Nullable<Varchar>,
        #[max_length = 70]
        og_title -> Nullable<Varchar>,
        #[max_length = 200]
        og_description -> Nullable<Varchar>,
        #[max_length = 500]
        og_image -> Nullable<Varchar>,
        #[max_length = 20]
        twitter_card -> Nullable<Varchar>,
        #[max_length = 70]
        twitter_title -> Nullable<Varchar>,
        #[max_length = 200]
        twitter_description -> Nullable<Varchar>,
        #[max_length = 100]
        author -> Nullable<Varchar>,
        tenant_id -> Nullable<Int4>,
        #[max_length = 50]
        article_type -> Nullable<Varchar>,
        #[max_length = 500]
        featured_image -> Nullable<Varchar>,
        word_count -> Nullable<Int4>,
        reading_time -> Nullable<Int4>,
        current_revision -> Nullable<Int4>,
        last_modified_by -> Nullable<Int4>,
        status -> Nullable<PagesStatus>,
        publish_at -> Nullable<Timestamp>,
        unpublish_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    product_variants (id) {
        id -> Int4,
        #[max_length = 36]
        uuid -> Varchar,
        product_id -> Int4,
        #[max_length = 100]
        sku -> Nullable<Varchar>,
        #[max_length = 255]
        variant_name -> Varchar,
        price -> Nullable<Numeric>,
        stock_quantity -> Nullable<Int4>,
        weight -> Nullable<Numeric>,
        attributes -> Nullable<Jsonb>,
        #[max_length = 500]
        image_url -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    products (id) {
        id -> Int8,
        #[max_length = 36]
        uuid -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        price_cents -> Int8,
        #[max_length = 100]
        sku -> Nullable<Varchar>,
        tenant_id -> Nullable<Int4>,
        inventory_count -> Nullable<Int4>,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        stock_quantity -> Int4,
        low_stock_threshold -> Nullable<Int4>,
        #[max_length = 20]
        stock_status -> Nullable<Varchar>,
        track_inventory -> Nullable<Bool>,
        allow_backorder -> Nullable<Bool>,
        backorder_limit -> Nullable<Int4>,
    }
}

diesel::table! {
    robots_rules (id) {
        id -> Int4,
        #[max_length = 100]
        user_agent -> Varchar,
        #[max_length = 20]
        directive -> Varchar,
        #[max_length = 500]
        path -> Varchar,
        crawl_delay -> Nullable<Int4>,
        #[max_length = 200]
        comment -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
        permissions -> Nullable<Json>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    search_history (id) {
        id -> Int8,
        user_id -> Nullable<Int4>,
        query_text -> Text,
        #[max_length = 20]
        search_type -> Nullable<Varchar>,
        results_count -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    tenants (id) {
        id -> Int4,
        #[max_length = 255]
        uuid -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 100]
        subdomain -> Varchar,
        #[max_length = 255]
        custom_domain -> Nullable<Varchar>,
        #[max_length = 50]
        plan -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        settings -> Nullable<Json>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tenant_members (id) {
        id -> Int4,
        tenant_id -> Int4,
        user_id -> Int4,
        #[max_length = 50]
        role -> Varchar,
        #[max_length = 20]
        status -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_oauth_connections (id) {
        id -> Int4,
        user_id -> Int4,
        provider_id -> Int4,
        #[max_length = 255]
        provider_user_id -> Varchar,
        access_token -> Nullable<Text>,
        refresh_token -> Nullable<Text>,
        expires_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_roles (user_id, role_id) {
        #[max_length = 255]
        user_id -> Varchar,
        role_id -> Int4,
        assigned_at -> Nullable<Timestamp>,
        #[max_length = 255]
        assigned_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    users (uuid) {
        id -> Int4,
        #[max_length = 255]
        uuid -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 511]
        token -> Nullable<Varchar>,
        #[max_length = 255]
        two_factor_secret -> Nullable<Varchar>,
        two_factor_enabled -> Bool,
    }
}

diesel::table! {
    webhook_logs (id) {
        id -> Int8,
        webhook_id -> Int4,
        #[max_length = 100]
        event_type -> Varchar,
        payload -> Nullable<Json>,
        response_status -> Nullable<Int4>,
        response_body -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    webhooks (id) {
        id -> Int4,
        #[max_length = 500]
        url -> Varchar,
        events -> Json,
        #[max_length = 255]
        secret -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        last_triggered_at -> Nullable<Timestamp>,
        failure_count -> Nullable<Int4>,
    }
}

diesel::joinable!(ai_key_rotation_history -> ai_provider_keys (provider_key_id));
diesel::joinable!(crm_campaigns -> crm_segments (segment_id));
diesel::joinable!(crm_interactions -> crm_customers (customer_id));
// TEMPORARILY DISABLED - optional FK joinable issue
// diesel::joinable!(crm_interactions -> orders (order_id));
diesel::joinable!(crm_notes -> crm_customers (customer_id));
diesel::joinable!(crm_segment_members -> crm_customers (customer_id));
diesel::joinable!(crm_segment_members -> crm_segments (segment_id));
diesel::joinable!(crm_tasks -> crm_customers (customer_id));
// TEMPORARILY DISABLED - optional FK joinable issue
// diesel::joinable!(inventory_audit_log -> orders (order_id));
diesel::joinable!(inventory_audit_log -> product_variants (variant_id));
// TEMPORARILY DISABLED - optional FK joinable issue
// diesel::joinable!(inventory_audit_log -> products (product_id));
diesel::joinable!(module_category -> pages (page_uuid));
diesel::joinable!(module_translations -> languages (language_id));
diesel::joinable!(modules -> module_category (category_uuid));
diesel::joinable!(modules -> pages (page_uuid));
diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(order_items -> products (product_id));
diesel::joinable!(page_translations -> languages (language_id));
// TEMPORARILY DISABLED - optional FK joinable issue
// diesel::joinable!(product_variants -> products (product_id));
diesel::joinable!(user_oauth_connections -> oauth_providers (provider_id));
diesel::joinable!(webhook_logs -> webhooks (webhook_id));

diesel::allow_tables_to_appear_in_same_query!(
    ai_generated_content,
    ai_key_rotation_history,
    ai_provider_configs,
    ai_provider_keys,
    ai_usage_log,
    analytics_events,
    analytics_summary,
    backups,
    content_embeddings,
    content_relationships,
    crm_campaigns,
    crm_customers,
    crm_interactions,
    crm_notes,
    crm_segment_members,
    crm_segments,
    crm_tasks,
    inventory_audit_log,
    languages,
    media,
    module_category,
    module_translations,
    modules,
    oauth_providers,
    order_items,
    orders,
    page_revisions,
    page_translations,
    page_views,
    pages,
    product_variants,
    products,
    robots_rules,
    roles,
    search_history,
    tenant_members,
    tenants,
    user_oauth_connections,
    user_roles,
    users,
    webhook_logs,
    webhooks,
);
