
diesel::table! {
    product_variants (id) {
        id -> Integer,
        #[max_length = 36]
        uuid -> Varchar,
        product_id -> Integer,
        #[max_length = 100]
        sku -> Nullable<Varchar>,
        #[max_length = 255]
        variant_name -> Varchar,
        price -> Nullable<Decimal>,
        stock_quantity -> Nullable<Integer>,
        weight -> Nullable<Decimal>,
        attributes -> Nullable<Jsonb>,
        #[max_length = 500]
        image_url -> Nullable<Varchar>,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    inventory_audit_log (id) {
        id -> Integer,
        product_id -> Nullable<Integer>,
        variant_id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        order_id -> Nullable<Integer>,
        #[max_length = 20]
        change_type -> Varchar,
        quantity_before -> Integer,
        quantity_after -> Integer,
        quantity_change -> Integer,
        #[max_length = 500]
        reason -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(orders -> users (user_uuid));
diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(order_items -> products (product_id));
diesel::joinable!(product_variants -> products (product_id));
diesel::joinable!(inventory_audit_log -> products (product_id));
diesel::joinable!(inventory_audit_log -> product_variants (variant_id));

diesel::allow_tables_to_appear_in_same_query!(
    modules,
    pages,
    users,
    roles,
    user_roles,
    page_revisions,
    products,
    orders,
    order_items,
    product_variants,
    inventory_audit_log,
);
