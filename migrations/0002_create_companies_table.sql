-- migrations/{timestamp}_create_companies_table.sql
-- Create Companies Table
CREATE TABLE IF NOT EXISTS companies(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(50) NOT NULL UNIQUE,
    description VARCHAR(100),
    website VARCHAR(50),
    logo VARCHAR(50)
);

-- Create an index's for quicker find
-- CREATE INDEX index_name ON table_name (column_name);
CREATE INDEX index_companies_id ON companies (id);
CREATE INDEX index_companies_name ON companies (name);