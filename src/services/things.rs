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
//!
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct ThingsData {
    first_name: String,
    middle_name: String,
    last_name: String,
    email: String,
}

#[tracing::instrument(name = "Insert new thing into the database", skip(pool, thing))]
pub async fn insert(pool: &PgPool, thing: &ThingsData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO things (id, first_name, middle_name, last_name, email, subscribed_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        thing.first_name,
        thing.middle_name,
        thing.last_name,
        thing.email,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

pub async fn find_by_id() {}

pub async fn find() {}

pub async fn update_by_id() {}

pub async fn delete_by_id() {}

#[cfg(test)]
pub mod tests {
    use sqlx::{Connection, Executor, PgConnection};

    use crate::configuration::{self, Database};

    use super::*;

    pub async fn init_test_database(database_config: &Database) -> PgPool {
        println!("Test database config used to initiate random test database: {:?}", database_config );

        // Connect to database
        let mut connection = PgConnection::connect_with(&database_config.without_database_name())
            .await
            .expect("Failed to connect to database instance...");

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

    #[actix_rt::test]
    async fn insert_thing() {
        let mut configuration: configuration::Settings =
            configuration::Settings::new().expect("Failed to read configuration...");
        // Assign a random database name to void test conflicts
        configuration.database.database_name = Uuid::new_v4().to_string();
        let connection_pool = init_test_database(&configuration.database).await;

        let things_data = ThingsData {
            first_name: "Joe".to_string(),
            middle_name: "Johns".to_string(),
            last_name: "Blogs".to_string(),
            email: "joe.blogs@email.com".to_string(),
        };

        let record = insert(&connection_pool, &things_data).await;
        assert!(record.is_ok());
    }
}
