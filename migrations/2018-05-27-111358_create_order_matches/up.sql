-- Your SQL goes here
CREATE TABLE order_matches (
    id UUID PRIMARY KEY,
    buy_order_id UUID REFERENCES orders check (buy_order_id != sell_order_id),
    sell_order_id UUID REFERENCES orders check (sell_order_id != buy_order_id),
    quantity_matched NUMERIC NOT NULL CHECK (quantity_matched > 0),
    price_matched NUMERIC NOT NULL CHECK (price_matched > 0),
    created_at TIMESTAMP NOT NULL
);