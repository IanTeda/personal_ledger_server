-- migrations/{timestamp}_create_things_table.sql
-- Create Subscriptions Table
CREATE TABLE things(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at timestamptz NOT NULL
);