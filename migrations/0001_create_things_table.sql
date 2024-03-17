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

-- Create an index's for quicker find
-- https://www.slingacademy.com/article/postgresql-how-to-set-index-on-a-table-column/
-- CREATE INDEX index_name ON table_name (column_name);
CREATE INDEX index_things_id ON things (id);
CREATE INDEX index_things_first_last_name ON things (first_name, last_name);
CREATE INDEX index_things_email ON things (email);