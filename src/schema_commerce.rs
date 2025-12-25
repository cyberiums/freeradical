// Commerce tables (append to existing schema.rs)

diesel::table! {
    products (id) {
        id -> BigInt,
        name -> Varchar,
        description -> Nullable<Text>,
        price_cents -> BigInt,
        currency -> Varchar,
        sku -> Nullable<Varchar>,
        inventory_count -> Nullable<Integer>,
        is_active -> Bool,
        metadata -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    orders (id) {
        id -> BigInt,
        user_uuid -> Varchar,
        total_cents -> BigInt,
        currency -> Varchar,
        status -> Varchar,
        payment_provider -> Nullable<Varchar>,
        payment_intent_id -> Nullable<Varchar>,
        metadata -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    order_items (id) {
        id -> BigInt,
        order_id -> BigInt,
        product_id -> BigInt,
        quantity -> Integer,
        price_cents -> BigInt,
    }
}

diesel::joinable!(orders -> users (user_uuid));
diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(order_items -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    products,
    orders,
    order_items,
);
