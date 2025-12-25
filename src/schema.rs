// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct PagesStatusEnum;
}

diesel::table! {
    analytics_summary (id) {
        id -> Integer,
        #[max_length = 500]
        page_url -> Varchar,
        date -> Date,
        view_count -> Nullable<Integer>,
        unique_visitors -> Nullable<Integer>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    media (id) {
        id -> Integer,
        #[max_length = 36]
        uuid -> Varchar,
        #[max_length = 255]
        filename -> Varchar,
        #[max_length = 255]
        original_filename -> Varchar,
        #[max_length = 100]
        mime_type -> Varchar,
        file_size -> Bigint,
        width -> Nullable<Integer>,
        height -> Nullable<Integer>,
        #[max_length = 255]
        folder -> Nullable<Varchar>,
        #[max_length = 500]
        storage_path -> Varchar,
        #[max_length = 500]
        cdn_url -> Nullable<Varchar>,
        upload_user_id -> Nullable<Integer>,
        #[max_length = 255]
        alt_text -> Nullable<Varchar>,
        caption -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    media_variants (id) {
        id -> Integer,
        media_id -> Integer,
        #[max_length = 50]
        variant_name -> Varchar,
        #[max_length = 500]
        file_path -> Varchar,
        width -> Nullable<Integer>,
        height -> Nullable<Integer>,
        file_size -> Nullable<Bigint>,
        created_at -> Nullable<Timestamp>,
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
    }
}

diesel::table! {
    page_revisions (id) {
        id -> Bigint,
        #[max_length = 36]
        page_uuid -> Varchar,
        revision_number -> Integer,
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
        changed_by_user_id -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    page_views (id) {
        id -> Bigint,
        #[max_length = 500]
        page_url -> Varchar,
        #[max_length = 36]
        page_uuid -> Nullable<Varchar>,
        #[max_length = 64]
        visitor_hash -> Varchar,
        #[max_length = 500]
        referrer -> Nullable<Varchar>,
        #[max_length = 500]
        user_agent -> Nullable<Varchar>,
        viewed_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PagesStatusEnum;

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
        #[max_length = 50]
        article_type -> Nullable<Varchar>,
        #[max_length = 500]
        featured_image -> Nullable<Varchar>,
        word_count -> Nullable<Integer>,
        reading_time -> Nullable<Integer>,
        current_revision -> Nullable<Integer>,
        last_modified_by -> Nullable<Integer>,
        #[max_length = 9]
        status -> Nullable<PagesStatusEnum>,
        publish_at -> Nullable<Timestamp>,
        unpublish_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    robots_rules (id) {
        id -> Integer,
        #[max_length = 100]
        user_agent -> Varchar,
        #[max_length = 20]
        directive -> Varchar,
        #[max_length = 500]
        path -> Varchar,
        crawl_delay -> Nullable<Integer>,
        #[max_length = 200]
        comment -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    content_relationships (id) {
        id -> Bigint,
        #[max_length = 255]
        source_type -> Varchar,
        #[max_length = 255]
        source_id -> Varchar,
        #[max_length = 255]
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
    roles (id) {
        id -> Integer,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
        permissions -> Nullable<Json>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    user_roles (user_id, role_id) {
        #[max_length = 255]
        user_id -> Varchar,
        role_id -> Integer,
        assigned_at -> Nullable<Timestamp>,
        #[max_length = 255]
        assigned_by -> Nullable<Varchar>,
    }
}

diesel::table! {
    webhooks (id) {
        id -> Integer,
        #[max_length = 500]
        url -> Varchar,
        events -> Json,
        #[max_length = 255]
        secret -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        last_triggered_at -> Nullable<Timestamp>,
        failure_count -> Nullable<Integer>,
    }
}

diesel::table! {
    webhook_logs (id) {
        id -> Bigint,
        webhook_id -> Integer,
        #[max_length = 100]
        event_type -> Varchar,
        payload -> Nullable<Json>,
        response_status -> Nullable<Integer>,
        response_body -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (uuid) {
        #[max_length = 255]
        uuid -> Varchar,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 511]
        token -> Nullable<Varchar>,
    }
}

diesel::joinable!(media_variants -> media (media_id));
diesel::joinable!(module_category -> pages (page_uuid));
diesel::joinable!(modules -> module_category (category_uuid));
diesel::joinable!(modules -> pages (page_uuid));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(webhook_logs -> webhooks (webhook_id));

diesel::allow_tables_to_appear_in_same_query!(
    analytics_summary,
    content_relationships,
    media,
    media_variants,
    module_category,
    modules,
    page_revisions,
    page_views,
    pages,
    robots_rules,
    roles,
    user_roles,
    users,
    webhooks,
    webhook_logs,
);
