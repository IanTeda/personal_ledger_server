-- migrations/{timestamp}_create_companies_table.sql
-- Create Companies Table
CREATE TABLE companies(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    name varchar(255) NOT NULL UNIQUE,
    description varchar(255),
    website varchar(255)
);