# Database Design

Test diagram below

```mermaid
erDiagram
    THINGS {
        uuid id "PRIMARY KEY" "DEFAULT uuid_generate_v4()"
        VARCHAR(50) email "NOT NULL UNIQUE"
        VARCHAR(50) first_name "" "User first name"
        VARCHAR(50) middle_name "" "user middle name"
        VARCHAR(50) last_name "NOT NULL" "User last name"
        VARCHAR(50) email "UNIQUE" "User email address"
        TIMESTAMPTZ created_at "NOT NULL" "Entry date creation date"
    }
    COMPANIES {
        UUID id "PRIMARY KEY" "DEFAULT uuid_generate_v4()"
        VARCHAR(50) name "NOT NULL UNIQUE" "Unique company name"
        VARCHAR(100) description "" "Short description for the company"
        VARCHAR(50) website "" "URL to company website"
        VARCHAR(50) logo "" "URL for company logo
        UUID company_note FK "" "Foreign key to notes table"
    }
    CUSTOMER ||--o{ ORDER : places
    CUSTOMER {
        string name
        string custNumber
        string sector
    }
    ORDER ||--|{ LINE-ITEM : contains
    ORDER {
        int orderNumber
        string deliveryAddress
    }
    LINE-ITEM {
        string productCode
        int quantity
        float pricePerUnit
    }
```

#### References

* [PostgreSQL Tutorial](https://www.postgresqltutorial.com/)
* [Introduction to Databases and SQL](https://www.programiz.com/sql/database-introduction)
* [Sling Academy - PostgreSQL](https://www.slingacademy.com/cat/postgresql/)