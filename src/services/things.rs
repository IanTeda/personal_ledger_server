//! A service template for inserting, updating, finding and deleting data in the 
//! database.
//!
//! The module contains a Thing struct model, implementations for database actions
//! and a builder for constructing a Thing.
//!
//! # REFERENCES
//!
//! * [Rust & MySQL: delete, insert data using crate sqlx.](https://dev.to/behainguyen/rust-mysql-delete-insert-data-using-crate-sqlx-9ii)
//! * [A Brief Introduction about Rust SQLx](https://medium.com/@edandresvan/a-brief-introduction-about-rust-sqlx-5d3cea2e8544)
//! * [rust_actix_sqlx_boilerplate](https://github.com/FabriceBazzaro/rust_actix_sqlx_boilerplate)
//! * [](https://codevoweb.com/rust-build-a-crud-api-with-sqlx-and-postgresql/)


// #![allow(unused)] // For development only

use crate::prelude::*;
use chrono::prelude::*;
use uuid::Uuid;

/// A Thing struct model.
///
/// This struct contains the data model for Thing. The model should be consistent
/// with the database table model. The database table models are defined in the
/// folder `./migrations` using sql statements.
///
/// # REFERENCES
///
/// * [Module sqlx::postgres::types](https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html)
/// * [jeremychone-channel/rust-builder](https://github.com/jeremychone-channel/rust-builder)
#[derive(Debug, serde::Deserialize, sqlx::FromRow, serde::Serialize, Clone)]
pub struct Thing {
    /// The Thing `id` as a Unique identifier (v7) and cannot be null in the database.
    id: Uuid,
    /// The Thing `name` is a String and cannot be null in the database.
    name: String,
    /// The Thing `description` is a String that can be null with the database,
    /// so it is Optional within the struct.
    description: Option<String>,
    /// The Thing `created_at` is a time zone time stamp and cannot be null in
    /// the database.
    created_at: DateTime<Utc>,
    /// The Thing `updated_at` is a time zone time stamp and cannot be null in
    /// the database.
    updated_at: DateTime<Utc>,
}

/// Implementations of the ThingModel
///
/// #Reference
///
/// * [Rust Book - Keyword impl](https://doc.rust-lang.org/std/keyword.impl.html)
impl Thing {
    /// Insert a ThingModel into a database, returning a Result with the inserted
    /// database row.
    ///
    /// # Parameters
    ///
    /// * `new_thing` - An instance of the ThingModel that will be added to the
    /// database
    /// * `database` - An sqlx database pool that the thing will be added to.
    #[tracing::instrument(name = "Insert Thing")]
    pub async fn insert(
        new_thing: &Thing,
        database: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Thing> {
        let record =         sqlx::query_as!(
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
        .await?;

        Ok(record)
    }

    /// Update thing in the database, returning a Result with either the updated
    /// thing database row or an sqlx error.
    ///
    /// # Parameters
    ///
    /// * `updated_thing` - An updated thing instance to update in the database
    /// * `database` - An sqlx database pool that the thing will be added to.
     #[tracing::instrument(name = "Update Thing")]
    pub async fn update(
        updated_thing: &Thing,
        database: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Thing> {
        let record = sqlx::query_as!(
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
        .await?;

        Ok(record)
    }

    /// Delete a database row from `things` table, returning the Result with the
    /// number of rows deleted or an sqlx error.
    ///
    /// # Parameters
    ///
    /// * `id` - The uuid of the database row to delete in the `things` database
    /// table.
    /// * `database` - An sqlx database pool that the thing will be deleted from.
     #[tracing::instrument(name = "Delete Thing by id")]
    pub async fn delete_by_id(
        id: Uuid,
        database: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<u64> {
        let result: sqlx::postgres::PgQueryResult = sqlx::query!(
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
    #[tracing::instrument(name = "Select Thing by id")]
    pub async fn select_by_id(
        id: Uuid,
        database: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Thing> {
        let record: Thing = sqlx::query_as!(
            Thing,
            r#"
                SELECT * 
                FROM things 
                WHERE id = $1
            "#,
            id
        )
        .fetch_one(database)
        .await?;

        Ok(record)
    }

    /// Get a row from the database table `things' by querying the thing name,
    /// returning a thing instance or sqlx error.
    ///
    /// # Parameters
    ///
    /// * `name` - Is a String containing the thing name
    /// * `database` - An sqlx database pool that the thing will be searched in.
    #[tracing::instrument(name = "Select Thing by name")]
    pub async fn get_by_name(
        name: &String,
        database: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Thing> {
        let record: Thing = sqlx::query_as!(
            Thing,
            r#"
                SELECT * 
                FROM things 
                WHERE name = $1
            "#,
            name
        )
        .fetch_one(database)
        .await?;

        Ok(record)
    }

    /// Get a count of all things row, returning a i64 or sqlx::Error
    /// 
    /// # Parameters
    /// 
    /// * `database` - An sqlx database pool that the thing will be searched in.
    #[tracing::instrument(name = "Count all Things")]
    pub async fn count_all(
        database: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<i64> {
        let count: Option<i64> = sqlx::query!(
            r#"
                SELECT COUNT(*)
                FROM things
            "#,
        )
        .fetch_one(database)
        .await?
        .count;

        Ok(count.unwrap())
    }

    /// Get an index of things, returning a vector of Things
    /// 
    /// # Parameters
    /// 
    /// * `limit` - An i64 limiting the page length
    /// * `offset` - An i64 of where the limit should start
    /// * `database` - An sqlx database pool that the things will be searched in.
    #[tracing::instrument(name = "Index of Things with offset and limit")]
    pub async fn index(
        limit: i64,
        offset: i64,
        database: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Vec<Thing>> {
        let records = sqlx::query_as!(
            Thing,
            r#"
                SELECT * 
                FROM things 
                LIMIT $1 OFFSET $2
            "#,
            limit,
            offset,
        )
        .fetch_all(database)
        .await?;

        Ok(records)
    }
}

/// The ThingBuilder model struct
/// 
/// The struct uses option type
#[derive(Clone)]
pub struct ThingBuilder {
    id: Option<Uuid>,
    name: Option<String>,
    description: Option<String>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>
}

/// Implementation of the default Thing for creating a new thing.
/// 
/// You can also use the #[derive(default)]
impl Default for ThingBuilder {
    fn default() -> Self {
        Self {
            id: Some(Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext))),
            name: Some("No Name".to_string()),
            description: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now())
        }
    }
}

/// Implement ThingBuilder functions
///
/// Consuming builder pattern example.
/// 
/// # References 
/// * [Rust Programming: The Ultimate Builder Pattern Tutorial](https://youtu.be/Z_3WOSiYYFY)
/// * [jeremychone-channel/rust-builder](https://github.com/jeremychone-channel/rust-builder/tree/main)
/// * [letsgetrusty/builder_pattern](https://github.com/letsgetrusty/builder_pattern)
/// * [Idiomatic Rust - Builder Pattern](https://www.youtube.com/watch?v=5DWU-56mjmg)
impl ThingBuilder {

    /// Create a new Thing instance, based on the name and default values
    pub fn new(name: impl Into<String>) -> Self {
        ThingBuilder {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    /// Nominate the id to use
    pub fn id(mut self, id: Uuid) -> Self {
		let _ = self.id.insert(id.into());
		self
	}

    /// Generate Uuid with a defined DateTime<Utc>
    pub fn id_set_date_time(mut self, date_time: DateTime<Utc> ) -> Self {
        // println!("{date_time:#?}");
        let uuid_timestamp: uuid::Timestamp = uuid::Timestamp::from_unix(
            uuid::NoContext,
            date_time.timestamp() as u64,
            date_time.timestamp_nanos_opt().unwrap() as u32,
        );
        // println!("{uuid_timestamp:#?}");
        let id: Uuid = Uuid::new_v7(uuid_timestamp);
        // println!("{id:#?}");
        let _ = self.id.insert(id.into());
        self
    }

    /// Use a different Thing name
    pub fn name(mut self, name: impl Into<String>) -> Self {
		let _ = self.name.insert(name.into());
		self
	}

    /// Add a description to the ThingBuilder
    pub fn description(mut self, description: impl Into<String>) -> Self {
		let _ = self.description.insert(description.into());
		self
	}

    /// Specify the creation date for the ThingBuilder
    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
		let _ = self.created_at.insert(created_at);
		self
	}

    /// Specify the updated_at date for the TingBuilder
    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
		let _ = self.updated_at.insert(updated_at);
		self
	}

    /// Build the new Thing
    pub fn build(self) -> Result<Thing> {
        // Run time check that `id` is not null
        let Some(id) = self.id else {
            return Err(Error::Static("No Uuid provided"));
        };

        // Run time check that `name` is not null
        let Some(name) = self.name.as_ref() else {
            return Err(Error::Static("No name provided"));
        };

        // Run time check that `created_at` is not null
        let Some(created_at) = self.created_at else {
            return Err(Error::Static("No created_at date provided"));
        };

        // Run time check that `created_at` is not null
        let Some(updated_at) = self.updated_at else {
            return Err(Error::Static("No updated_at date provided"));
        };

        Ok(Thing {
            id,
            name: name.to_string(),
            description: self.description,
            created_at,
            updated_at
        })
    }
}

//-- Unit Tests
#[cfg(test)]
pub mod tests {

    // Override with more flexible error
    pub type Result<T> = core::result::Result<T, Error>;
	pub type Error = Box<dyn std::error::Error>;

    // Bring module functions into test scope
    use super::*;

    use fake::faker::{
        chrono::en::DateTime, 
        chrono::en::DateTimeAfter, 
        lorem::en::*
    };
    use fake::Fake;
    use sqlx::{Pool, Postgres};
    use tracing::debug;

    // Create a random Thing for testing
    async fn create_random_test_thing() -> Result<Thing> {
        //-- Setup random thing data
        let thing_id: Uuid = Uuid::now_v7();
        let thing_name: &str = Word().fake();
        let thing_description: String = Sentence(3..7).fake();
        let thing_created_at: DateTime<Utc> = DateTime().fake();
        let thing_updated_at: DateTime<Utc> = DateTime().fake();

        //-- Return random test
        let random_thing: Thing = ThingBuilder::new(thing_name)
            .id(thing_id)
            .description(&thing_description)
            .created_at(thing_created_at)
            .updated_at(thing_updated_at)
            .build()?;

        Ok(random_thing)
    }

    // Test creating a new Thing without a description
    #[actix_rt::test]
    async fn create_new_thing() -> Result<()> {
        //-- Setup and Fixtures (Arrange)
        let thing_name: &str = Word().fake();

        //-- Execute Function (Act)
        let test_new_thing: Thing = ThingBuilder::new(thing_name).build()?;
        // println!("{test_new_thing:#?}");

         //-- Checks (Assertions)
        assert_eq!(test_new_thing.name, thing_name);
        assert_eq!(test_new_thing.description, None);

        Ok(())
    }

    // Test creating a new Thing with a description
    #[actix_rt::test]
    async fn create_new_thing_with_description() -> Result<()> {
        //-- Setup and Fixtures (Arrange)
        let thing_name: &str = Word().fake();
        let thing_description: String = Sentence(3..7).fake();

        //-- Execute Function (Act)
        let test_new_thing: Thing = ThingBuilder::new(thing_name)
            .description(&thing_description)
            .build()?;
        // println!("{test_new_thing:#?}");

         //-- Checks (Assertions)
        assert_eq!(test_new_thing.name, thing_name);
        assert_eq!(test_new_thing.description.unwrap(), thing_description);

        Ok(())
    }

    // Test creating a new Thing with an id
    #[actix_rt::test]
    async fn create_new_thing_with_id() -> Result<()> {
        //-- Setup and Fixtures (Arrange)
        let thing_id: Uuid = Uuid::now_v7();
        let thing_name: &str = Word().fake();
        let thing_description: String = Sentence(3..7).fake();
        let thing_created_at: DateTime<Utc> = DateTime().fake();
        let thing_updated_at: DateTime<Utc> = DateTime().fake();

        //-- Execute Function (Act)
        let test_new_thing: Thing = ThingBuilder::new(thing_name)
            .id(thing_id)
            .description(&thing_description)
            .created_at(thing_created_at)
            .updated_at(thing_updated_at)
            .build()?;
        // println!("{test_new_thing:#?}");

         //-- Checks (Assertions)
        assert_eq!(test_new_thing.id, thing_id);
        assert_eq!(test_new_thing.name, thing_name);
        assert_eq!(test_new_thing.description.unwrap(), thing_description);
        assert_eq!(test_new_thing.created_at, thing_created_at);
        assert_eq!(test_new_thing.updated_at, thing_updated_at);

        Ok(())
    }

    // Test creating a new Thing with an timestamp id
    #[actix_rt::test]
    async fn create_new_thing_with_id_timestamp() -> Result<()> {
        //-- Setup and Fixtures (Arrange)
        let thing_datetime: DateTime<Utc> = DateTimeAfter(chrono::DateTime::UNIX_EPOCH).fake();
        // println!("{thing_datetime:#?}");
        let thing_name: &str = Word().fake();
        let thing_description: String = Sentence(3..7).fake();
        let thing_created_at: DateTime<Utc> = DateTime().fake();
        let thing_updated_at: DateTime<Utc> = DateTime().fake();

        //-- Execute Function (Act)
        let test_new_thing: Thing = ThingBuilder::new(thing_name)
            .description(&thing_description)
            .id_set_date_time(thing_datetime)
            .created_at(thing_created_at)
            .updated_at(thing_updated_at)
            .build()?;
        // println!("{test_new_thing:#?}");

         //-- Checks (Assertions)
        assert_eq!(test_new_thing.name, thing_name);
        assert_eq!(test_new_thing.description.unwrap(), thing_description);
        assert_eq!(test_new_thing.created_at, thing_created_at);
        assert_eq!(test_new_thing.updated_at, thing_updated_at);

        Ok(())
    }

    // Test inserting a thing into the database
    //
    // `#[sqlx::test]` The test will automatically be executed in the async
    // runtime. For every annotated function, a new test database is created so
    // tests can run against a live database but are isolated from each other.
    // Test databases are automatically cleaned up as tests succeed, but failed
    // tests will leave their databases in-place to facilitate debugging.
    //
    // `pool: Pool<Postgres>` needs to be added to the test function parameters
    //
    // #### References
    //
    // * [Attribute Macro sqlx::test](https://docs.rs/sqlx/latest/sqlx/attr.test.html)
    #[sqlx::test]
    async fn insert(pool: Pool<Postgres>) -> Result<()> {
        //-- Setup and Fixtures (Arrange) (Arrange)
        let test_thing: Thing = create_random_test_thing().await?;
        debug!("test_thing equals: {:?}", test_thing);

        //-- Execute Function (Act) (Act)
        let record = Thing::insert(&test_thing, &pool).await?;

         //-- Checks (Assertions) (Assert)
        // println!("unwrapped inserted record is {:?}", thing_record);
        debug!("thing_record is: {:?}", record);
        debug!("thing_record id is: {:?}", record.id);
        debug!("thing_data id is: {:?}", record.id);

        assert_eq!(record.id, test_thing.id);
        assert_eq!(record.name, test_thing.name);
        assert_eq!(record.description, test_thing.description);
        assert_eq!(
            record.created_at.timestamp(),
            test_thing.created_at.timestamp()
        );
        assert_eq!(
            record.updated_at.timestamp(),
            test_thing.updated_at.timestamp()
        );

        Ok(())
    }

    //   Test updating a thing row in the database
    #[sqlx::test]
    async fn update(pool: Pool<Postgres>) -> Result<()> {
        //-- Setup and Fixtures (Arrange)
        let original_test_thing: Thing = create_random_test_thing().await?;

        Thing::insert(&original_test_thing, &pool).await?;

        let mut updated_test_thing: Thing = original_test_thing.clone();

        updated_test_thing.name = Word().fake();
        updated_test_thing.description = Sentence(1..2).fake();

        //-- Execute Function (Act)
        let update_record: Thing =
            Thing::update(&updated_test_thing, &pool).await?;

         //-- Checks (Assertions)
        assert_eq!(update_record.id, original_test_thing.id);
        assert_eq!(update_record.name, updated_test_thing.name);
        assert_eq!(update_record.description, updated_test_thing.description);
        assert_eq!(
            update_record.created_at.timestamp_millis(),
            original_test_thing.created_at.timestamp_millis()
        );
        assert_ne!(
            update_record.updated_at.timestamp_millis(),
            original_test_thing.updated_at.timestamp_millis()
        );

        Ok(())
    }

    // Test deleting a thing row in the database
    #[sqlx::test]
    async fn delete_by_id(pool: Pool<Postgres>) -> Result<()> {
        //-- Setup and Fixtures (Arrange)
        let test_thing: Thing = create_random_test_thing().await?;
        let record: Thing = Thing::insert(&test_thing, &pool).await?;

        //-- Execute Function (Act)
        let rows_deleted: u64 =
            Thing::delete_by_id(record.id, &pool).await?;

         //-- Checks (Assertions)
        assert_eq!(rows_deleted, 1);

        Ok(())
    }

    // Test getting a thing row in the database by id
    #[sqlx::test]
    async fn get_thing_by_id(pool: Pool<Postgres>) -> Result<()> {
        //-- Setup and Fixtures (Arrange)
        let test_thing: Thing = create_random_test_thing().await?;
        Thing::insert(&test_thing, &pool).await?;

        //-- Execute Function (Act)
        let record: Thing =
            Thing::select_by_id(test_thing.id, &pool).await?;

         //-- Checks (Assertions)
        assert_eq!(record.id, test_thing.id);
        assert_eq!(record.name, test_thing.name);
        assert_eq!(record.description, test_thing.description);
        assert_eq!(
            record.created_at.timestamp(),
            test_thing.created_at.timestamp()
        );
        assert_eq!(
            record.updated_at.timestamp(),
            test_thing.updated_at.timestamp()
        );

        Ok(())
    }

    // Test getting a thing row in the database by name
    #[sqlx::test]
    async fn get_thing_by_name(pool: Pool<Postgres>) -> Result<()> {
        //-- Setup and Fixtures (Arrange)
        let test_thing: Thing = create_random_test_thing().await?;
        Thing::insert(&test_thing, &pool).await?;

        //-- Execute Function (Act)
        let record: Thing =
            Thing::select_by_id(test_thing.id, &pool).await?;

         //-- Checks (Assertions)
        assert_eq!(record.id, test_thing.id);
        assert_eq!(record.name, test_thing.name);
        assert_eq!(record.description, test_thing.description);
        assert_eq!(
            record.created_at.timestamp_millis(),
            record.created_at.timestamp_millis()
        );
        assert_eq!(
            record.updated_at.timestamp_millis(),
            record.updated_at.timestamp_millis()
        );

        Ok(())
    }

    // Test count of thing rows
    #[sqlx::test]
    async fn count_things(pool: Pool<Postgres>) -> Result<()> {
        //-- Setup and Fixtures (Arrange)
        let random_count: i64 = (1..20).fake::<i64>();

        debug!("The count is {}", random_count);

        for _count in 0..random_count {
        let test_thing: Thing = create_random_test_thing().await?;
        Thing::insert(&test_thing, &pool).await?;
        }

        //-- Execute Function (Act)
        let test_count = 
            Thing::count_all(&pool).await?;

         //-- Checks (Assertions)
        debug!("The test_count is: {}", test_count);
        assert_eq!(test_count, random_count);

        Ok(())
    }

    // Test thing query
    #[sqlx::test]
    async fn index(pool: Pool<Postgres>) -> Result<()> {
        //-- Setup and Fixtures (Arrange)
        let random_count: i64 = (10..30).fake::<i64>();
        let mut test_vec: Vec<Thing> = Vec::new();
        for _count in 0..random_count {
            let test_thing: Thing = create_random_test_thing().await?;
            test_vec.push(Thing::insert(&test_thing, &pool).await?);
        }

        //-- Execute Function (Act)
        let random_limit: i64 = (1..random_count).fake::<i64>();
        let random_offset: i64 = (1..random_count).fake::<i64>();
        let records: Vec<Thing> = 
            Thing::index(random_limit, random_offset, &pool).await?;

         //-- Checks (Assertions)
        let count_less_offset: i64 = random_count - random_offset;
        let expected_records: i64;

        if count_less_offset <  random_limit {
            expected_records = count_less_offset
        } else {
            expected_records = random_limit
        }

        let random_vec_index: i64 = (1..expected_records).fake::<i64>() - 1;
        let random_test_vec_index = random_offset + random_vec_index;
        let random_record_thing = &records[random_vec_index as usize];
        let random_test_thing = &test_vec[random_test_vec_index as usize];

        // println!("{random_record_thing:?}");
        // println!("{random_test_thing:?}");

        assert_eq!(records.len() as i64, expected_records);
        assert_eq!(random_record_thing.id, random_test_thing.id);


        Ok(())
    }


}