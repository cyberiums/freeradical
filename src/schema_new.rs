// @generated automatically by Diesel CLI.

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

diesel::joinable!(module_category -> pages (page_uuid));
diesel::joinable!(modules -> module_category (category_uuid));
diesel::joinable!(modules -> pages (page_uuid));

diesel::allow_tables_to_appear_in_same_query!(module_category, modules, pages, users,);
