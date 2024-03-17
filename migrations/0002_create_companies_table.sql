-- migrations/{timestamp}_create_companies_table.sql
-- Create Companies Table
CREATE TABLE IF NOT EXISTS companies(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(50) NOT NULL UNIQUE,
    description VARCHAR(100),
    website VARCHAR(50),
    logo VARCHAR(50)
);