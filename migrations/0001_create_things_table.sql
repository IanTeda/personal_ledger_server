-- migrations/{timestamp}_create_things_table.sql
-- Create Subscriptions Table
CREATE TABLE IF NOT EXISTS things(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    first_name VARCHAR(50),
    middle_name VARCHAR(50),
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(50) NOT NULL UNIQUE,
    subscribed_at TIMESTAMPTZ NOT NULL
);