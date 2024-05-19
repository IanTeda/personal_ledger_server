// -- ./src/service/thing.rs

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
//! * [Rust – Build a CRUD API with SQLX and PostgreSQL](https://codevoweb.com/rust-build-a-crud-api-with-sqlx-and-postgresql/)
//! * [Dataform SQLX](https://sqlx.dev/)
//! ---

// #![allow(unused)] // For development only

use crate::{
	domain::{Thing, ThingBuilder, ThingDescription, ThingName},
	prelude::*,
};
extern crate derive_more;
use chrono::prelude::Utc;
use tracing::debug;
use uuid::Uuid;

/// Insert a `Thing` into the database, returning the `Thing` created.
///
/// # Parameters
///
/// * `thing` - A Thing instance
/// * `database` - An Sqlx database connection pool
/// ---
#[tracing::instrument(
	name = "Insert a new Thing into the database."
	skip(thing, database)
)]
pub async fn insert(
	thing: &Thing,
	database: &sqlx::Pool<sqlx::Postgres>,
) -> Result<Thing> {
	let database_record = sqlx::query!(
		r#"
            INSERT INTO things (id, name, description, created_at, updated_at) 
            VALUES ($1, $2, $3, $4, $5) 
            RETURNING *
        "#,
		thing.id,
		thing.name.as_ref(),
		thing.description.as_ref().unwrap().as_ref(),
		thing.created_at,
		thing.updated_at,
	)
	.fetch_one(database)
	.await?;
	debug!("Record inserted into database: {database_record:#?}");

	let new_thing = ThingBuilder::new(ThingName::parse(database_record.name)?)
		.id(database_record.id)
		.description(ThingDescription::parse(
			database_record.description.unwrap(),
		)?)
		.created_at(database_record.created_at)
		.updated_at(database_record.updated_at)
		.build()?;
	debug!("New Thing: {new_thing:#?}");

	Ok(new_thing)
}

/// Update a `Thing` in the database, returning the updated `Thing`
/// 
/// # Parameters
/// 
/// * `thing` - A Thing instance with updated properties
/// * `database` - An Sqlx database connection pool
/// ---
#[tracing::instrument(
	name = "Update a Thing in the database."
	skip(thing, database)
)]
pub async fn update(
	thing: &Thing,
	database: &sqlx::Pool<sqlx::Postgres>,
) -> Result<Thing> {
	let database_record = sqlx::query!(
		r#"
            UPDATE things 
            SET name = $2, description = $3, updated_at = $4
            WHERE id = $1 
            RETURNING *
        "#,
		thing.id,
		thing.name.as_ref(),
		thing.description.as_ref().unwrap().as_ref(),
		Utc::now(),
	)
	.fetch_one(database)
	.await?;
	debug!("Record updated into database: {database_record:#?}");

	let updated_thing = ThingBuilder::new(ThingName::parse(database_record.name)?)
		.id(database_record.id)
		.description(ThingDescription::parse(
			database_record.description.unwrap(),
		)?)
		.created_at(database_record.created_at)
		.updated_at(database_record.updated_at)
		.build()?;
	debug!("Updated Thing: {updated_thing:#?}");

	Ok(updated_thing)
}

/// Delete a `Thing` in the database with its id, returning the number of 
/// `Things` deleted
/// 
/// # Parameters
/// 
/// * `id` - The Uuid of the Thing database row you want to delete
/// * `database` - An Sqlx database connection pool
/// ---
#[tracing::instrument(
	name = "Delete a Thing in the database using it id (uuid)."
	skip(id, database)
)]
pub async fn delete_by_id(
	id: &Uuid,
	database: &sqlx::Pool<sqlx::Postgres>,
) -> Result<u64> {
	let record = sqlx::query!(
		r#"
			DELETE
			FROM things
			WHERE id = $1
		"#,
		id
	)
	.execute(database)
	.await?;
	debug!("Record deleted form database: {record:#?}");

	 Ok(record.rows_affected())
}

/// Get thing row from the database table `things' by querying the thing uuid,
/// returning a thing instance or sqlx error.
///
/// # Parameters
///
/// * `id` - The uuid of thing to be returned
/// * `database` - An sqlx database pool that the thing will be searched in.
#[tracing::instrument(
	name = "Get a Thing from the database using its id (uuid)."
	skip(id, database)
)]
pub async fn get_by_id(
	id: &Uuid,
	database: &sqlx::Pool<sqlx::Postgres>
) -> Result<Thing> {
	let database_record = sqlx::query!(
		r#"
			SELECT * 
			FROM things 
			WHERE id = $1
		"#,
		id
	)
	.fetch_one(database)
	.await?;
	debug!("Record retrieved form database: {database_record:#?}");

	// Build a thing base on the database record (row) found
	let thing = ThingBuilder::new(ThingName::parse(database_record.name)?)
		.id(database_record.id)
		.description(ThingDescription::parse(
			database_record.description.unwrap(),
		)?)
		.created_at(database_record.created_at)
		.updated_at(database_record.updated_at)
		.build()?;
	debug!("Thing found: {thing:#?}");

	Ok(thing)
}

/// Get a row from the database table `things' by querying the thing name,
/// returning a thing instance or sqlx error.
///
/// # Parameters
///
/// * `name` - Is a String containing the thing name
/// * `database` - An sqlx database pool that the thing will be searched in.
/// ---
#[tracing::instrument(
	name = "Get a Thing from the database using its name."
	skip(name, database)
)]
pub async fn get_by_name(
	name: impl Into<String>,
	database: &sqlx::Pool<sqlx::Postgres>
) -> Result<Thing> {
	// let name = name.into();
	let database_record = sqlx::query!(
		r#"
			SELECT * 
			FROM things 
			WHERE name = $1
		"#,
		name.into()
	)
	.fetch_one(database)
	.await?;
	debug!("Record retrieved form database: {database_record:#?}");

	// Build a thing base on the database record (row) found
	let thing = ThingBuilder::new(ThingName::parse(database_record.name)?)
		.id(database_record.id)
		.description(ThingDescription::parse(
			database_record.description.unwrap(),
		)?)
		.created_at(database_record.created_at)
		.updated_at(database_record.updated_at)
		.build()?;
	debug!("Thing found: {thing:#?}");

	Ok(thing)
}

/// Get a count of all Things in the database, returning a i64
/// 
/// # Parameters
/// 
/// * `database` - An sqlx database pool that the thing will be searched in.
/// ---
#[tracing::instrument(
	name = "Get a count of all Things in the database."
	skip(database)
)]
pub async fn count_all(	
	database: &sqlx::Pool<sqlx::Postgres>
) -> Result<i64> {
	let count = sqlx::query!(
		r#"
			SELECT COUNT(*)
			FROM things
		"#,
	)
	.fetch_one(database)
	.await?
	.count;
	debug!("Database count: {count:#?}");

	Ok(count.unwrap())
}

/// Get an index of things, returning a vector of Things
/// 
/// # Parameters
/// 
/// * `limit` - An i64 limiting the page length
/// * `offset` - An i64 of where the limit should start
/// * `database` - An sqlx database pool that the things will be searched in.
/// ---
#[tracing::instrument(
	name = "Index of Things with offset and limit"
	skip(database)
)]
pub async fn index(
	limit: i64,
	offset: i64,
	database: &sqlx::Pool<sqlx::Postgres>,
) -> Result<Vec<Thing>> {
	let records = sqlx::query!(
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
	debug!("Database records returned from database: {records:#?}");

	let mut things: Vec<Thing> = Vec::new();
	for record in records {
		let thing = ThingBuilder::new(ThingName::parse(record.name)?)
			.id(record.id)
			.description(ThingDescription::parse(
				record.description.unwrap(),
			)?)
			.created_at(record.created_at)
			.updated_at(record.updated_at)
			.build()?;
		things.push(thing);
	}

	Ok(things)
}

#[cfg(test)]
pub mod tests {
	// Bring file/module functions into unit test scope
	use super::*;

	// Override with more flexible error
	pub type Result<T> = core::result::Result<T, Error>;
	pub type Error = Box<dyn std::error::Error>;

	// use claim::{assert_err, assert_ok};
	use chrono::{DateTime, Utc};
	use fake::faker::{
		chrono::en::{DateTime, DateTimeAfter},
		lorem::en::*,
	};
	use fake::Fake;
	use sqlx::{Pool, Postgres};
	use tracing::debug;
	use uuid::Uuid;

	// Create a random Thing for testing
	async fn create_random_test_thing() -> Result<Thing> {
		//-- Setup random thing data
		let thing_datetime: DateTime<Utc> =
			DateTimeAfter(chrono::DateTime::UNIX_EPOCH).fake();
		let uuid_timestamp: uuid::Timestamp = uuid::Timestamp::from_unix(
			uuid::NoContext,
			thing_datetime.timestamp() as u64,
			thing_datetime.timestamp_nanos_opt().unwrap() as u32,
		);
		// println!("{uuid_timestamp:#?}");
		let thing_id: Uuid = Uuid::new_v7(uuid_timestamp);
		let name: String = Word().fake();
		let thing_name = ThingName::parse(name)?;
		let description: String = Sentence(3..7).fake();
		let thing_description = ThingDescription::parse(description)?;
		let thing_created_at = DateTime().fake();
		let thing_updated_at = DateTime().fake();

		//-- Return random test
		let random_thing: Thing = ThingBuilder::new(thing_name)
			.id(thing_id)
			.description(thing_description)
			.created_at(thing_created_at)
			.updated_at(thing_updated_at)
			.build()?;

		Ok(random_thing)
	}

	// Test inserting into database
	#[sqlx::test]
	async fn insert_database_record(database: Pool<Postgres>) -> Result<()> {
		//-- Setup and Fixtures (Arrange)
		let test_thing = create_random_test_thing().await?;
		debug!("{test_thing:#?}");

		//-- Execute Function (Act)
		let record = insert(&test_thing, &database).await?;
		debug!("{record:#?}");

        //-- Checks (Assertions)
		// assert_eq!(test_thing, record); // TODO: Time precision is diff to Postgres

		assert_eq!(record.id, test_thing.id);
		assert_eq!(record.name, test_thing.name);
		assert_eq!(record.description, test_thing.description);
		// Use timestamp because Postgres time precision is less than Rust
		assert_eq!(
			record.created_at.timestamp(),
			test_thing.created_at.timestamp()
		);
		assert_eq!(
			record.updated_at.timestamp(),
			test_thing.updated_at.timestamp()
		);

		// -- Return
		Ok(())
	}

	// Test updating entry in the database
	#[sqlx::test]
	async fn update_database_record(database: Pool<Postgres>) -> Result<()> {
        //-- Setup and Fixtures (Arrange)
		let original_test_thing = create_random_test_thing().await?;
		debug!("{original_test_thing:#?}");

		let record = insert(&original_test_thing, &database).await?;
		debug!("{record:#?}");

		//-- Execute Function (Act)
        let mut updated_test_thing: Thing = original_test_thing.clone();
		let updated_name: String = Word().fake();
		let updated_description: String = Sentence(3..7).fake();
        updated_test_thing.name = ThingName::parse(updated_name)?;
        updated_test_thing.description = Some(ThingDescription::parse(updated_description)?);
		let updated_thing_record = update(&updated_test_thing, &database).await?;

        //-- Checks (Assertions)
        assert_eq!(updated_thing_record.id, original_test_thing.id);
        assert_ne!(updated_thing_record.name, original_test_thing.name);
		assert_eq!(updated_thing_record.name, updated_test_thing.name);
        assert_ne!(updated_thing_record.description, original_test_thing.description);
		assert_eq!(updated_thing_record.description, updated_test_thing.description);
        assert_eq!(
            updated_thing_record.created_at.timestamp_millis(),
            original_test_thing.created_at.timestamp_millis()
        );
        assert_ne!(
            updated_thing_record.updated_at.timestamp_millis(),
            original_test_thing.updated_at.timestamp_millis()
        );

		// -- Return
		Ok(())
	}

	// Test deleting a Thing row in the database
    #[sqlx::test]
    async fn delete_database_record(database: Pool<Postgres>) -> Result<()> {
        //-- Setup and Fixtures (Arrange)
        let test_thing = create_random_test_thing().await?;
        let record = insert(&test_thing, &database).await?;

        //-- Execute Function (Act)
        let rows_deleted = delete_by_id(&record.id, &database).await?;

        //-- Checks (Assertions)
        assert_eq!(rows_deleted, 1);

		// -- Return
        Ok(())
    }

	// Test finding a Thing row in the database by id
    #[sqlx::test]
    async fn find_database_record_by_id(database: Pool<Postgres>) -> Result<()> {
        //-- Setup and Fixtures (Arrange)
        let test_thing = create_random_test_thing().await?;
		insert(&test_thing, &database).await?;

        //-- Execute Function (Act)
        let record = get_by_id(&test_thing.id, &database).await?;

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

		// -- Return
        Ok(())
    }

	// Test finding a Thing row in the database by name
    #[sqlx::test]
    async fn find_database_record_by_name(database: Pool<Postgres>) -> Result<()> {
        //-- Setup and Fixtures (Arrange)
        let test_thing = create_random_test_thing().await?;
		insert(&test_thing, &database).await?;

		let test_name = test_thing.name.as_ref().to_string();

        //-- Execute Function (Act)
        let record = get_by_name(&test_name, &database).await?;

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

		// -- Return
        Ok(())
    }

    // Test counting Thing rows in the database
    #[sqlx::test]
    async fn count_things_in_database(pool: Pool<Postgres>) -> Result<()> {
        //-- Setup and Fixtures (Arrange)
        let random_count: i64 = (1..20).fake::<i64>();
        debug!("The count is {}", random_count);

        for _count in 0..random_count {
			let test_thing: Thing = create_random_test_thing().await?;
			insert(&test_thing, &pool).await?;
        }

        //-- Execute Function (Act)
        let test_count = count_all(&pool).await?;

        //-- Checks (Assertions)
        debug!("The test_count is: {}", test_count);
        assert_eq!(test_count, random_count);

        Ok(())
    }	

    // Test thing query
    #[sqlx::test]
    async fn get_things_in_database(pool: Pool<Postgres>) -> Result<()> {
        //-- Setup and Fixtures (Arrange)
        let random_count: i64 = (10..30).fake::<i64>();
        let mut test_vec: Vec<Thing> = Vec::new();
        for _count in 0..random_count {
            let test_thing: Thing = create_random_test_thing().await?;
            test_vec.push(insert(&test_thing, &pool).await?);
        }

        //-- Execute Function (Act)
        let random_limit = (1..random_count).fake::<i64>();
        let random_offset = (1..random_count).fake::<i64>();
        let records = index(random_limit, random_offset, &pool).await?;

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

        assert_eq!(records.len() as i64, expected_records);
        assert_eq!(random_record_thing.id, random_test_thing.id);


        Ok(())
    }
}
