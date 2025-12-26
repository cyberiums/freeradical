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
        word_count -> Nullable<Int4>,
        reading_time -> Nullable<Int4>,
        current_revision -> Nullable<Int4>,
        last_modified_by -> Nullable<Int4>,
        #[max_length = 9]
        status -> Nullable<Varchar>,
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

diesel::joinable!(module_category -> pages (page_uuid));
diesel::joinable!(modules -> module_category (category_uuid));
diesel::joinable!(modules -> pages (page_uuid));
diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(order_items -> products (product_id));
diesel::joinable!(product_variants -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    module_category,
    modules,
    order_items,
    orders,
    page_revisions,
    pages,
    product_variants,
    products,
    roles,
    user_roles,
    users,
);
