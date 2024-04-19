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
pub struct Thing {
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
impl Thing {
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
    #[tracing::instrument(
        name = "Create a new thing instance",
    )]
    pub async fn new(name: &str) -> Self {
        Thing {
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
    // /// ```
    // /// use personal_ledger_server::services::things::*;
    // ///
    // /// let mut wiz_bang = ThingModel::new("Wiz Bang");
    // /// wiz_bang.add_description("Lots of bang and wiz");
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
        new_thing: &Thing,
        database: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Thing, sqlx::Error> {
        sqlx::query_as!(
            Thing,
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
    pub async fn update(
        updated_thing: &Thing,
        database: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Thing, sqlx::Error> {
        sqlx::query_as!(
            Thing,
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

    /// Delete a database row from `things` table, returning the Result with the
    /// number of rows deleted or an sqlx error.
    ///
    /// # Parameters
    ///
    /// * `id` - The uuid of the database row to delete in the `things` database
    /// table.
    /// * `database` - An sqlx database pool that the thing will be deleted from.
    pub async fn delete_by_id(
        id: Uuid,
        database: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
                DELETE
                FROM things
                WHERE id = $1
            "#,
            id
        )
        .execute(database)
        .await?;

        Ok(result.rows_affected())
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
    ) -> Result<Thing, sqlx::Error> {
        sqlx::query_as!(
            Thing,
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

    /// Get a row from the database table `things' by querying the thing name,
    /// returning a thing instance or sqlx error.
    ///
    /// # Parameters
    ///
    /// * `name` - Is a String containing the thing name
    /// * `database` - An sqlx database pool that the thing will be searched in.
    pub async fn get_by_name(
        name: &String,
        database: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Thing, sqlx::Error> {
        sqlx::query_as!(
            Thing,
            r#"
                SELECT * 
                FROM things 
                WHERE name = $1
            "#,
            name
        )
        .fetch_one(database)
        .await
    }

    // pub async fn get_count(
    //     tx: &mut crate::Transaction<'_>,
    //     id: i64,
    // ) -> Result<usize, DatabaseError> {
    //     Ok(sqlx::query!(
    //         "SELECT COUNT(id) as size FROM _tblmedia WHERE library_id = ?",
    //         id
    //     )
    //     .fetch_one(&mut *tx)
    //     .await?
    //     .size as usize)
    // }
}

// https://qxf2.com/blog/data-generation-in-rust-using-fake-crate/
#[cfg(test)]
pub mod tests {
    use fake::faker::{chrono::en::DateTime, lorem::en::*, name::en::Name};
    use fake::uuid::UUIDv4;
    use fake::Fake;
    use sqlx::{Pool, Postgres};
    use tracing::debug;

    use super::*;

    async fn create_test_thing() -> Thing {
        let thing_id: Uuid = UUIDv4.fake();
        let thing_name: String = Name().fake();
        let thing_description: Option<String> = Sentence(1..2).fake();
        let thing_created_at: DateTime<Utc> = DateTime().fake();
        let thing_updated_at: DateTime<Utc> = DateTime().fake();

        Thing {
            id: thing_id,
            name: thing_name,
            description: thing_description,
            created_at: thing_created_at,
            updated_at: thing_updated_at,
        }
    }

    // Test creating a new Thing without a description
    #[actix_rt::test]
    async fn new_thing_instance() {
        let thing_name: &str = Word().fake();
        let test_new_thing = Thing::new(thing_name).await;

        assert_eq!(test_new_thing.name, thing_name);
        assert_eq!(test_new_thing.description, None);
    }

    // Test creating a new Thing with description
    #[actix_rt::test]
    async fn new_thing_instance_description() {
        let thing_name: &str = Word().fake();
        let thing_description: &str = "I am a test sentence";
        let mut test_thing = Thing::new(thing_name).await;
        test_thing.add_description(thing_description).await;

        assert_eq!(test_thing.name, thing_name);
        assert_eq!(test_thing.description.unwrap(), thing_description);
    }

    /// Test inserting a thing into the database
    ///
    /// `#[sqlx::test]` The test will automatically be executed in the async
    /// runtime. For every annotated function, a new test database is created so
    /// tests can run against a live database but are isolated from each other.
    /// Test databases are automatically cleaned up as tests succeed, but failed
    /// tests will leave their databases in-place to facilitate debugging.
    ///
    /// `pool: Pool<Postgres>` needs to be added to the test function parameters
    ///
    /// #### References
    ///
    /// * [Attribute Macro sqlx::test](https://docs.rs/sqlx/latest/sqlx/attr.test.html)
    #[sqlx::test]
    async fn insert(pool: Pool<Postgres>) {
        let test_thing: Thing = create_test_thing().await;
        debug!("test_thing equals: {:?}", test_thing);

        let record: Result<Thing, sqlx::Error> = Thing::insert(&test_thing, &pool).await;

        assert!(record.is_ok());

        let thing_record: Thing = record.unwrap();

        // println!("unwrapped inserted record is {:?}", thing_record);
        debug!("thing_record is: {:?}", thing_record);
        debug!("thing_record id is: {:?}", thing_record.id);
        debug!("thing_data id is: {:?}", thing_record.id);

        assert_eq!(thing_record.id, test_thing.id);
        assert_eq!(thing_record.name, test_thing.name);
        assert_eq!(thing_record.description, test_thing.description);
        assert_eq!(
            thing_record.created_at.timestamp_millis(),
            test_thing.created_at.timestamp_millis()
        );
        assert_eq!(
            thing_record.updated_at.timestamp_millis(),
            test_thing.updated_at.timestamp_millis()
        );
    }

    /// Test updating a thing row in the database
    #[sqlx::test]
    async fn update(pool: Pool<Postgres>) {
        let original_test_thing: Thing = create_test_thing().await;

        let _ = Thing::insert(&original_test_thing, &pool).await;

        let mut updated_test_thing: Thing = original_test_thing.clone();

        updated_test_thing.name = Name().fake();
        updated_test_thing.description = Sentence(1..2).fake();

        let update_record: Result<Thing, sqlx::Error> =
            Thing::update(&updated_test_thing, &pool).await;

        let update_thing_row: Thing = update_record.unwrap();

        assert_eq!(update_thing_row.id, original_test_thing.id);
        assert_eq!(update_thing_row.name, updated_test_thing.name);
        assert_eq!(update_thing_row.description, updated_test_thing.description);
        assert_eq!(
            update_thing_row.created_at.timestamp_millis(),
            original_test_thing.created_at.timestamp_millis()
        );
        assert_ne!(
            update_thing_row.updated_at.timestamp_millis(),
            original_test_thing.updated_at.timestamp_millis()
        );
    }

    /// Test deleting a thing row in the database
    #[sqlx::test]
    async fn delete_by_id(pool: Pool<Postgres>) {
        let test_thing: Thing = create_test_thing().await;

        let insert_record: Result<Thing, sqlx::Error> = 
            Thing::insert(&test_thing, &pool).await;

        let thing_id: Uuid = insert_record.unwrap().id;

        let delete_record: Result<u64, sqlx::Error> =
            Thing::delete_by_id(thing_id, &pool).await;

        let things_deleted: u64 = delete_record.unwrap();

        assert_eq!(things_deleted, 1);
    }

    /// Test getting a thing row in the database by id
    #[sqlx::test]
    async fn get_thing_by_id(pool: Pool<Postgres>) {
        let test_thing: Thing = create_test_thing().await;
        let _ = Thing::insert(&test_thing, &pool).await;

        let test_thing_record: Result<Thing, sqlx::Error> =
            Thing::get_by_id(test_thing.id, &pool).await;

        let test_thing_row: Thing = test_thing_record.unwrap();

        assert_eq!(test_thing_row.id, test_thing.id);
        assert_eq!(test_thing_row.name, test_thing.name);
        assert_eq!(test_thing_row.description, test_thing.description);
        assert_eq!(
            test_thing_row.created_at.timestamp_millis(),
            test_thing.created_at.timestamp_millis()
        );
        assert_eq!(
            test_thing_row.updated_at.timestamp_millis(),
            test_thing.updated_at.timestamp_millis()
        );
    }

    /// Test getting a thing row in the database by name
    #[sqlx::test]
    async fn get_thing_by_name(pool: Pool<Postgres>) {
        let test_thing: Thing = create_test_thing().await;
        let _ = Thing::insert(&test_thing, &pool).await;

        let test_thing_record: Result<Thing, sqlx::Error> =
            Thing::get_by_name(&test_thing.name, &pool).await;

        let test_thing_row: Thing = test_thing_record.unwrap();

        assert_eq!(test_thing_row.id, test_thing.id);
        assert_eq!(test_thing_row.name, test_thing.name);
        assert_eq!(test_thing_row.description, test_thing.description);
        assert_eq!(
            test_thing_row.created_at.timestamp_millis(),
            test_thing.created_at.timestamp_millis()
        );
        assert_eq!(
            test_thing_row.updated_at.timestamp_millis(),
            test_thing.updated_at.timestamp_millis()
        );
    }
}
