// @generated automatically by Diesel CLI.

diesel::table! {
    analytics_summary (id) {
        id -> Integer,
        page_url -> Varchar,
        date -> Date,
        view_count -> Nullable<Integer>,
        unique_visitors -> Nullable<Integer>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    module_category (uuid) {
        uuid -> Varchar,
        page_uuid -> Varchar,
        title -> Varchar,
    }
}

diesel::table! {
    modules (uuid) {
        uuid -> Varchar,
        page_uuid -> Varchar,
        category_uuid -> Nullable<Varchar>,
        title -> Varchar,
        content -> Text,
    }
}

diesel::table! {
    page_views (id) {
        id -> Bigint,
        page_url -> Varchar,
        page_uuid -> Nullable<Varchar>,
        visitor_hash -> Varchar,
        referrer -> Nullable<Varchar>,
        user_agent -> Nullable<Varchar>,
        viewed_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    pages (uuid) {
        uuid -> Varchar,
        page_name -> Varchar,
        page_url -> Varchar,
        page_title -> Varchar,
        time_created -> Timestamp,
        meta_title -> Nullable<Varchar>,
        meta_description -> Nullable<Varchar>,
        meta_keywords -> Nullable<Varchar>,
        canonical_url -> Nullable<Varchar>,
        og_title -> Nullable<Varchar>,
        og_description -> Nullable<Varchar>,
        og_image -> Nullable<Varchar>,
        twitter_card -> Nullable<Varchar>,
        twitter_title -> Nullable<Varchar>,
        twitter_description -> Nullable<Varchar>,
        author -> Nullable<Varchar>,
        article_type -> Nullable<Varchar>,
        featured_image -> Nullable<Varchar>,
        word_count -> Nullable<Integer>,
        reading_time -> Nullable<Integer>,
    }
}

diesel::table! {
    robots_rules (id) {
        id -> Integer,
        user_agent -> Varchar,
        directive -> Varchar,
        path -> Varchar,
        crawl_delay -> Nullable<Integer>,
        comment -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (uuid) {
        uuid -> Varchar,
        username -> Varchar,
        password -> Varchar,
        token -> Nullable<Varchar>,
    }
}

diesel::joinable!(module_category -> pages (page_uuid));
diesel::joinable!(modules -> module_category (category_uuid));
diesel::joinable!(modules -> pages (page_uuid));

diesel::allow_tables_to_appear_in_same_query!(
    analytics_summary,
    module_category,
    modules,
    page_views,
    pages,
    robots_rules,
    users,
);
