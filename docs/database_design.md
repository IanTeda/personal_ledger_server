# Database Design

Test diagram below

```mermaid
erDiagram
    THINGS {
        uuid id PK "NOT NULL"
        varchar email "NOT NULL UNIQUE"
        varchar first_name "" "User first name:
        varchar last_name "" "User last name"
        timestamptz created_at "NOT NULL" "Entry date creation date"
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