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
use uuid::Uuid;

// ## Thing Struct
//
// A Thing data structure
//
// #### REFERENCES
// 
// * [Module sqlx::postgres::types](https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html)
#[derive(Debug, serde::Deserialize, sqlx::FromRow, serde::Serialize, Clone)]

pub struct Thing {
    pub id: Uuid,
    pub name: String,
    // description needs to be in an Option<> because it can be null in database table
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Thing {
    // Create a new thing without a description, which is optional
    pub async fn new(name: &str) -> Self {
        Thing {
            id: Uuid::new_v4(),
            name: name.to_owned(),
            description: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        }
    }

    // Add description to a thing
    pub async fn add_description(&mut self, description: &str) {
        self.description = Some(description.to_owned());
    }

    // Insert thing into database
    pub async fn insert(new_thing: &Thing, database: &sqlx::Pool<sqlx::Postgres>) -> Result<Thing, sqlx::Error> {
        //// Without sqlx macro type checking
        // let statement =
        //     r#"INSERT INTO things (name, description) VALUES ($1, $2) RETURNING *"#;
        // let query = sqlx::query_as::<_,Thing>(statement);
        // let thing = query
        //     .bind(&thing.name)
        //     .bind(&thing.description)
        //     .fetch_one(database)
        //     .await?;

        sqlx::query_as!(
            Thing,
            r#"
                INSERT INTO things (id, name, description) 
                VALUES ($1, $2, $3) 
                RETURNING *
            "#,
            new_thing.id,
            new_thing.name,
            new_thing.description,
        )
        .fetch_one(database)
        .await
    }

    
}

#[cfg(test)]
pub mod tests {
    use fake::faker::lorem::en::*;
    use fake::Fake;
    use sqlx::{Connection, Executor, PgConnection, PgPool};
    use tracing::debug;

    use crate::configuration::{self, Database};

    use super::*;

    pub async fn create_test_database(database_config: &Database) -> PgPool {
        debug!(
            "Test database config used to initiate random test database: {:?}",
            database_config
        );

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

    // Test creating a new Thing without a description
    #[actix_rt::test]
    async fn new_thing(){
        let thing_name: &str = Word().fake();
        let test_thing = Thing::new(thing_name).await;

        assert_eq!(test_thing.name, thing_name);
        assert_eq!(test_thing.description, None);
        assert_ne!(test_thing.created_at, None);
        assert_ne!(test_thing.updated_at, None);
    }

    // Test creating a new Thing with description
    #[actix_rt::test]
    async fn new_thing_description(){
        let thing_name: &str = Word().fake();
        let thing_description: &str = "I am a test sentence";
        let mut test_thing = Thing::new(thing_name).await;
        test_thing.add_description(thing_description).await;

        assert_eq!(test_thing.name, thing_name);
        assert_eq!(test_thing.description.unwrap(), thing_description);
        assert_ne!(test_thing.created_at, None);
        assert_ne!(test_thing.updated_at, None);
    }

    #[actix_rt::test]
    async fn thing_new_ok() {
        let mut configuration: configuration::Settings =
            configuration::Settings::new().expect("Failed to read configuration...");
        // Assign a random database name to avoid test conflicts
        configuration.database.database_name = uuid::Uuid::new_v4().to_string();
        let random_test_database = create_test_database(&configuration.database).await;

        let mut test_thing = Thing::new("Marble").await;
        test_thing.add_description("A round hard ball").await;

        println!("test_thing is {:?}", test_thing);

        let test_thing_data = Thing {
            id: uuid::Uuid::new_v4(),
            name: Word().fake(),
            description: Sentence(3..7).fake(),
            created_at: None,
            updated_at: None,
        };
        debug!("thing_data is: {:?}", test_thing_data);

        let record = Thing::insert(&test_thing_data, &random_test_database).await;
        // debug!("record is: {:?}", record);
        assert!(record.is_ok());

        let thing_record = record.unwrap();
        debug!("thing_record is: {:?}", thing_record);
        debug!("thing_record id is: {:?}", thing_record.id);
        debug!("thing_data id is: {:?}", test_thing_data.id);

        assert_eq!(test_thing_data.id, thing_record.id);
        assert_eq!(test_thing_data.name, thing_record.name);
        assert_eq!(test_thing_data.description, thing_record.description);
        assert_ne!(thing_record.created_at, None);
        assert_ne!(thing_record.updated_at, None);

        // drop_test_database(&configuration.database).await;
    }
}
