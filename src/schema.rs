// @generated automatically by Diesel CLI.

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

diesel::allow_tables_to_appear_in_same_query!(module_category, modules, pages, users,);
