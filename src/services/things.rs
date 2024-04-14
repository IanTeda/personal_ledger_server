//! A template for inserting, updating, finding and deleting data in the database.
//!
//! The thing service module is a boilerplate template for adding a new database
//! service.
//!
//! The module contains a thing model struct that implements.
//!
//! #### REFERENCES
//!
//! * [Rust & MySQL: delete, insert data using crate sqlx.](https://dev.to/behainguyen/rust-mysql-delete-insert-data-using-crate-sqlx-9ii)
//! * [A Brief Introduction about Rust SQLx](https://medium.com/@edandresvan/a-brief-introduction-about-rust-sqlx-5d3cea2e8544)
//! * [rust_actix_sqlx_boilerplate](https://github.com/FabriceBazzaro/rust_actix_sqlx_boilerplate)
//! * [](https://codevoweb.com/rust-build-a-crud-api-with-sqlx-and-postgresql/)

use chrono::prelude::*;
use sqlx::postgres::PgQueryResult;
use uuid::Uuid;

/// A Thing model structure.
///
/// This struct contains the data model for Thing. The model should be consistent
/// with the database table model. The database table models are defined in the
/// folder `./migrations` using sql statements.
///
/// #### REFERENCES
///
/// * [Module sqlx::postgres::types](https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html)
#[derive(Debug, serde::Deserialize, sqlx::FromRow, serde::Serialize, Clone)]
pub struct ThingModel {
    /// The Thing `id` as a Unique identifier and cannot be null in the database.
    pub id: Uuid,
    /// The Thing `name` is a String and cannot be null in the database.
    pub name: String,
    /// The Thing `description` is a String that can be null with the database,
    /// so it is Optional within the struct.
    pub description: Option<String>,
    /// The Thing `created_at` is a time zone time stamp and cannot be null in
    /// the database.
    pub created_at: DateTime<Utc>,
    /// The Thing `updated_at` is a time zone time stamp and cannot be null in
    /// the database.
    pub updated_at: DateTime<Utc>,
}

/// Implementations of the ThingModel
///
/// #### Reference
///
/// [Rust Book - Keyword impl](https://doc.rust-lang.org/std/keyword.impl.html)
impl ThingModel {
    /// Create a new Thing without a description, which is optional. It creates an
    /// instance with `created_at` and `updated_at` set to now. It returns an
    /// instance of ThingModel (Self).
    ///
    /// # Parameters
    ///
    /// * `name` - A string slice containing the name of the thing to instantiated.
    ///
    /// # Example
    ///
    /// ```
    /// use personal_ledger_server::services::things::ThingModel;
    ///
    /// let wiz_bang = ThingModel::new("Wiz Bang");
    /// ```
    pub async fn new(name: &str) -> Self {
        ThingModel {
            id: Uuid::new_v4(),
            name: name.to_owned(),
            description: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Add a description to a ThingModel instance
    ///
    /// # Parameters
    ///
    /// * `description` - A string slice that is the description of the thing to
    /// be added to the ThingModel instance.
    ///
    // /// # Example
    // ///
    // /// ```
    // /// use personal_ledger_server::services::things::ThingModel;
    // ///
    // /// let wiz_bang = ThingModel::new("Wiz Bang");
    // /// wiz_bang.add_description = "Lots of Bang and Wiz!"
    // /// ```
    pub async fn add_description(&mut self, description: &str) {
        self.description = Some(description.to_owned());
    }

    /// Insert a ThingModel into a database, returning a Result with the inserted
    /// database row or an sqlx error.
    ///
    /// # Parameters
    ///
    /// * `new_thing` - An instance of the ThingModel that will be added to the
    /// database
    /// * `database` - An sqlx database pool that the thing will be added to.
    pub async fn insert(
        new_thing: &ThingModel,
        database: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<ThingModel, sqlx::Error> {
        sqlx::query_as!(
            ThingModel,
            r#"
                INSERT INTO things (id, name, description, created_at, updated_at) 
                VALUES ($1, $2, $3, $4, $5) 
                RETURNING *
            "#,
            new_thing.id,
            new_thing.name,
            new_thing.description,
            new_thing.created_at,
            new_thing.updated_at
        )
        .fetch_one(database)
        .await
    }

    /// Update thing in the database, returning a Result with either the updated
    /// thing database row or an sqlx error.
    ///
    /// # Parameters
    ///
    /// * `updated_thing` - An updated thing instance to update in the database
    /// * `database` - An sqlx database pool that the thing will be added to.
    ///
    pub async fn update(
        updated_thing: &ThingModel,
        database: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<ThingModel, sqlx::Error> {
        sqlx::query_as!(
            ThingModel,
            r#"
                UPDATE things 
                SET name = $2, description = $3, updated_at = $4
                WHERE id = $1 
                RETURNING *
            "#,
            updated_thing.id,
            updated_thing.name,
            updated_thing.description,
            Utc::now(),
        )
        .fetch_one(database)
        .await
    }

    /// Delete a database row from `things` table, returning boolean or an sqlx
    /// error.
    ///
    /// # Parameters
    ///
    /// * `id` - The uuid of the database row to delete in the `things` database
    /// table.
    /// * `database` - An sqlx database pool that the thing will be deleted from.
    pub async fn delete_by_id(
        id: Uuid,
        database: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            r#"
                DELETE
                FROM things
                WHERE id = $1
            "#,
            id
        )
        .execute(database)
        .await

        // match result {
        //     Err(e) => {
        //         // println!("Error deleting employee: {}\n", e.to_string());
        //         // return false;
        //         Ok(false);
        //     }

        //     Ok(res) => {
        //         // println!("Employee number: {} has been deleted.", emp_no);
        //         // println!("Number of Employees deleted: {}", res.rows_affected());
        //     }
        // }

        // true
    }

    /// Get thing row from the database table `things' by querying the thing uuid,
    /// returning a thing instance or sqlx error.
    /// 
    /// # Parameters
    /// 
    /// * `id` - The uuid of thing to be returned
    /// * `database` - An sqlx database pool that the thing will be searched in.
    pub async fn get_by_id(
        id: Uuid,
        database: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<ThingModel, sqlx::Error> {
        sqlx::query_as!(
            ThingModel,
            r#"
                SELECT * 
                FROM things 
                WHERE id = $1
            "#,
            id
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
    async fn new_thing_instance() {
        let thing_name: &str = Word().fake();
        let test_new_thing = ThingModel::new(thing_name).await;

        assert_eq!(test_new_thing.name, thing_name);
        assert_eq!(test_new_thing.description, None);
    }

    // Test creating a new Thing with description
    #[actix_rt::test]
    async fn new_thing_instance_description() {
        let thing_name: &str = Word().fake();
        let thing_description: &str = "I am a test sentence";
        let mut test_thing = ThingModel::new(thing_name).await;
        test_thing.add_description(thing_description).await;

        assert_eq!(test_thing.name, thing_name);
        assert_eq!(test_thing.description.unwrap(), thing_description);

    }
    #[actix_rt::test]
    async fn insert_thing_into_database() {
        let mut configuration: configuration::Settings =
            configuration::Settings::new().expect("Failed to read configuration...");
        // Assign a random database name to avoid test conflicts
        configuration.database.database_name = uuid::Uuid::new_v4().to_string();
        let random_test_database = create_test_database(&configuration.database).await;

        let thing_name: &str = Word().fake();
        let thing_description: &str = "I am a test sentence";
        let mut test_thing_data = ThingModel::new(thing_name).await;
        test_thing_data.add_description(thing_description).await;

        debug!("test_thing_data is: {:?}", test_thing_data);

        let record = ThingModel::insert(&test_thing_data, &random_test_database).await;
        // debug!("record is: {:?}", record);
        assert!(record.is_ok());

        let thing_record = record.unwrap();
        debug!("thing_record is: {:?}", thing_record);
        debug!("thing_record id is: {:?}", thing_record.id);
        debug!("thing_data id is: {:?}", test_thing_data.id);

        assert_eq!(test_thing_data.id, thing_record.id);
        assert_eq!(test_thing_data.name, thing_record.name);
        assert_eq!(test_thing_data.description, thing_record.description);
        assert_eq!(thing_record.created_at, thing_record.created_at);
        assert_eq!(thing_record.updated_at, thing_record.updated_at);

        // drop_test_database(&configuration.database).await;
    }
}
