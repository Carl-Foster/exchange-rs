table! {
    depth_cache (id) {
        id -> Int4,
        contract_id -> Int4,
        direction -> Varchar,
        order_id -> Uuid,
        quantity_remaining -> Numeric,
    }
}

table! {
    order_matches (id) {
        id -> Uuid,
        buy_order_id -> Uuid,
        sell_order_id -> Uuid,
        quantity_matched -> Numeric,
        price_matched -> Numeric,
        created_at -> Timestamp,
    }
}

table! {
    orders (id) {
        id -> Uuid,
        price -> Numeric,
        quantity -> Numeric,
        account_id -> Varchar,
        direction -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

joinable!(depth_cache -> orders (order_id));

allow_tables_to_appear_in_same_query!(
    depth_cache,
    order_matches,
    orders,
);
