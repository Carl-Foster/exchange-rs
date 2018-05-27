-- Your SQL goes here
CREATE TABLE order_matches (
    id UUID PRIMARY KEY,
    buy_order_id UUID NOT NULL REFERENCES orders check (buy_order_id != sell_order_id),
    sell_order_id UUID NOT NULL REFERENCES orders check (sell_order_id != buy_order_id),
    quantity_matched INTEGER NOT NULL CHECK (quantity_matched > 0),
    price_matched INTEGER NOT NULL CHECK (price_matched > 0),
    created_at TIMESTAMPTZ NOT NULL
);