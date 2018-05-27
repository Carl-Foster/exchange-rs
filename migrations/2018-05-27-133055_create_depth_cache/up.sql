-- Your SQL goes here
CREATE TABLE depth_cache (
    id SERIAL PRIMARY KEY,
    contract_id INTEGER NOT NULL,
    direction VARCHAR NOT NULL,
    order_id UUID NOT NULL REFERENCES orders,
    quantity_remaining NUMERIC NOT NULL CHECK (quantity_remaining > 0)
);