//! ./src/services/things.rs
//!
//! # THINGS DATABASE SERVICE
//!
//! A template for insert, update, find and delete data in the database
//!
//! #### REFERENCES
//!
//! * [Rust & MySQL: delete, insert data using crate sqlx.](https://dev.to/behainguyen/rust-mysql-delete-insert-data-using-crate-sqlx-9ii)
//! * [A Brief Introduction about Rust SQLx](https://medium.com/@edandresvan/a-brief-introduction-about-rust-sqlx-5d3cea2e8544)
//! * [rust_actix_sqlx_boilerplate](https://github.com/FabriceBazzaro/rust_actix_sqlx_boilerplate)
//!
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres};

#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]

pub struct Thing {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>,
}

impl Thing {
    pub async fn new(
        database: &Pool<Postgres>,
        thing: &Thing
    ) -> Result<Thing, sqlx::Error> {
        let statement = "INSERT INTO things (id, name, description) VALUES ($1, $2, $3) RETURNING *";
        let query = sqlx::query_as::<_,Thing>(statement);
        // let query = sqlx::query_as!(Thing, statement, &thing.id, &thing.name, &thing.description);
        // let query = sqlx::query_as!(Thing, statement)
        let thing = query
            .bind(&thing.id)
            .bind(&thing.name)
            .bind(&thing.description)
            .fetch_one(database)
            .await?;
        Ok(thing)
    }
}

#[cfg(test)]
pub mod tests {
    use sqlx::{Connection, Executor, PgConnection, PgPool};
    use tracing::debug;
    use fake::Fake;
    use fake::faker::lorem::en::*;

    use crate::configuration::{self, Database};

    use super::*;

    pub async fn create_test_database(database_config: &Database) -> PgPool {
        debug!("Test database config used to initiate random test database: {:?}", database_config );

        // Connect to database
        let mut connection = PgConnection::connect_with(&database_config.without_database_name())
            .await
            .expect("Failed to connect to random test database instance...");

        // Create random test database
        connection
            .execute(&*format!(
                r#"CREATE DATABASE "{}";"#,
                database_config.database_name
            ))
            .await
            .expect("Failed to create random test database...");

        // Connect to database pool using random test database
        let connection_pool = sqlx::PgPool::connect_with(database_config.with_database_name())
            .await
            .expect("Failed to connect to random test database connection pool...");

        // Apply database migrations to random test database
        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to apply migrations to random test database...");

        connection_pool
    }

    pub async fn drop_test_database(database_config: &Database) {

        // Connect to database
        let mut connection = PgConnection::connect_with(&database_config.without_database_name())
            .await
            .expect("Failed to connect to database instance...");

        // Create random test database
        connection
            .execute(&*format!(
                r#"DROP DATABASE IF EXISTS "{}";"#,
                database_config.database_name
            ))
            .await
            .expect("Failed to drop random test database...");
    }

    #[actix_rt::test]
    async fn thing_new_ok() {
        let mut configuration: configuration::Settings =
            configuration::Settings::new().expect("Failed to read configuration...");
        // Assign a random database name to avoid test conflicts
        configuration.database.database_name = uuid::Uuid::new_v4().to_string();
        let random_test_database = create_test_database(&configuration.database).await;

        let thing_data = Thing {
            id: uuid::Uuid::new_v4(),
            name: Word().fake(),
            description:  Sentence(3..7).fake::<String>(),
            created_at: None,
            updated_at: None,
        };
        debug!("thing_data is: {:?}", thing_data);

        let record = Thing::new(&random_test_database, &thing_data).await;
        // debug!("record is: {:?}", record);
        assert!(record.is_ok());

        let thing_record = record.unwrap();
        debug!("thing_record is: {:?}", thing_record);
        debug!("thing_record id is: {:?}", thing_record.id);
        debug!("thing_data id is: {:?}", thing_data.id);

        assert_eq!(thing_data.id, thing_record.id);
        assert_eq!(thing_data.name, thing_record.name);
        assert_eq!(thing_data.description, thing_record.description);
        assert_ne!(thing_record.created_at, None);
        assert_ne!(thing_record.updated_at, None);

        // drop_test_database(&configuration.database).await;
    }
}