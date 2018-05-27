-- Your SQL goes here
CREATE TABLE orders (
    id UUID PRIMARY KEY,
    price NUMERIC NOT NULL CHECK (price > 0),
    quantity NUMERIC NOT NULL CHECK (quantity > 0),
    account_id VARCHAR NOT NULL,
    direction VARCHAR NOT NULL,
    created_at TIMESTAMP
);