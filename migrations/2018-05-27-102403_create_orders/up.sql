-- Your SQL goes here
CREATE TABLE orders (
    id UUID PRIMARY KEY,
    price INTEGER NOT NULL CHECK (price > 0),
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    account_id VARCHAR NOT NULL,
    direction VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL
);