-- Add migration script here
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    order_name TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    order_date TIMESTAMPTZ NOT NULL,
    order_price FLOAT NOT NULL,
    order_status TEXT NOT NULL,
    last_update TIMESTAMPTZ NOT NULL
);
