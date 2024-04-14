-- migrations/{timestamp}_create_things_table.sql
-- Create Things Table
CREATE TABLE IF NOT EXISTS things (
    id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
    name VARCHAR NOT NULL,
    description VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL
);

-- Create an index's for quicker find
-- https://www.slingacademy.com/article/postgresql-how-to-set-index-on-a-table-column/
-- CREATE INDEX index_name ON table_name (column_name);
CREATE INDEX index_things_id ON things (id);
