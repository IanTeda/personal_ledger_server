// #![allow(unused)] // For development only

mod thing_description;
mod thing_name;

pub use thing_description::ThingDescription;
pub use thing_name::ThingName;

use chrono::prelude::*;
use uuid::Uuid;

use crate::prelude::*;
extern crate derive_more;

/// A Thing struct model.
///
/// This struct contains the data model for Thing. The model should be consistent
/// with the database table model. The database table models are defined in the
/// folder `./migrations` using sql statements.
///
/// # References
///
/// * [Module sqlx::postgres::types](https://docs.rs/sqlx/latest/sqlx/postgres/types/index.html)
/// * [jeremychone-channel/rust-builder](https://github.com/jeremychone-channel/rust-builder)
#[derive(
	Clone,
	Debug,
    PartialEq,
	derive_more::From,
	derive_more::Into,
	serde::Deserialize,
	serde::Serialize,
	sqlx::FromRow,
)]
pub struct Thing {
	/// The Thing `id` as a Unique identifier (v7) and cannot be null in the database.
	pub id: Uuid,
	/// The Thing `name` is a String and cannot be null in the database.
	pub name: ThingName,
	/// The Thing `description` is a String that can be null with the database,
	/// so it is Optional within the struct.
	pub description: Option<ThingDescription>,
	/// The Thing `created_at` is a time zone time stamp and cannot be null in
	/// the database.
	pub created_at: DateTime<Utc>,
	/// The Thing `updated_at` is a time zone time stamp and cannot be null in
	/// the database.
	pub updated_at: DateTime<Utc>,
}

/// Implementation of the default Thing for creating a new thing.
///
/// You can also use the #[derive(default)]
impl Default for Thing {
	fn default() -> Self {
		Self {
			id: Uuid::now_v7(),
			name: ThingName::default(),
			description: Some(ThingDescription::default()),
			created_at: Utc::now(),
			updated_at: Utc::now(),
		}
	}
}

/// The ThingBuilder model struct
#[derive(Clone)]
pub struct ThingBuilder {
	id: Option<Uuid>,
	name: Option<ThingName>,
	description: Option<ThingDescription>,
	created_at: Option<DateTime<Utc>>,
	updated_at: Option<DateTime<Utc>>,
}

impl ThingBuilder {
	/// Create a new Thing instance, based on the `name` passed and default values.
	pub fn new(name: ThingName) -> Self {
		ThingBuilder {
			id: Some(Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext))),
			name: Some(name),
			description: None,
			created_at: Some(Utc::now()),
			updated_at: Some(Utc::now()),
		}
	}

	/// Overwrite default `id` in builder.
	pub fn id(&mut self, id: Uuid) -> &mut Self {
		let _ = self.id.insert(id);
		self
	}

	/// Overwrite default `id` ind builder with a DateTime<Utc> generated Uuid.
	pub fn id_set_date_time(&mut self, date_time: DateTime<Utc>) -> &mut Self {
		// println!("{date_time:#?}");
		let uuid_timestamp: uuid::Timestamp = uuid::Timestamp::from_unix(
			uuid::NoContext,
			date_time.timestamp() as u64,
			date_time.timestamp_nanos_opt().unwrap() as u32,
		);
		// println!("{uuid_timestamp:#?}");
		let id: Uuid = Uuid::new_v7(uuid_timestamp);
		// println!("{id:#?}");
		let _ = self.id.insert(id);
		self
	}

	/// Overwrite `name` passed to new() in builder.
	pub fn name(&mut self, name: ThingName) -> &mut Self {
		let _ = self.name.insert(name);
		self
	}

	/// Overwrite default `None` description in builder.
	pub fn description(&mut self, description: ThingDescription) -> &mut Self {
		let _ = self.description.insert(description);
		self
	}

	/// Overwrite default `created_at` in builder.
	pub fn created_at(&mut self, created_at: DateTime<Utc>) -> &mut Self {
		let _ = self.created_at.insert(created_at);
		self
	}

	/// Overwrite default `updated_at` in builder.
	pub fn updated_at(&mut self, updated_at: DateTime<Utc>) -> &mut Self {
		let _ = self.updated_at.insert(updated_at);
		self
	}

	pub fn build(&self) -> Result<Thing> {
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
			name: name.clone(),
			description: self.description.clone(),
			created_at,
			updated_at,
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
		chrono::en::DateTime, chrono::en::DateTimeAfter, lorem::en::*,
	};
	use fake::Fake;

	// Test creating a new Thing without a description
	#[actix_rt::test]
	async fn create_new_thing() -> Result<()> {
		//-- Setup and Fixtures (Arrange)
        let name: String = Word().fake();
		let thing_name = ThingName::parse(name)?;

		//-- Execute Function (Act)
		let test_new_thing = ThingBuilder::new(thing_name.clone()).build()?;
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
        let name: String = Word().fake();
		let thing_name = ThingName::parse(name)?;
        let description: String = Sentence(3..7).fake();
		let thing_description = ThingDescription::parse(description)?;

		//-- Execute Function (Act)
		let test_new_thing = ThingBuilder::new(thing_name.clone())
			.description(thing_description.clone())
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
		let thing_id = Uuid::now_v7();
        let name: String = Word().fake();
		let thing_name = ThingName::parse(name)?;
        let description: String = Sentence(3..7).fake();
		let thing_description = ThingDescription::parse(description)?;
		let thing_created_at = DateTime().fake();
		let thing_updated_at = DateTime().fake();

		//-- Execute Function (Act)
		let test_new_thing = ThingBuilder::new(thing_name.clone())
			.id(thing_id)
			.description(thing_description.clone())
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
		let thing_datetime = DateTimeAfter(chrono::DateTime::UNIX_EPOCH).fake();
		// println!("{thing_datetime:#?}");
        let name: String = Word().fake();
		let thing_name = ThingName::parse(name)?;
        let description: String = Sentence(3..7).fake();
		let thing_description = ThingDescription::parse(description)?;
		let thing_created_at = DateTime().fake();
		let thing_updated_at = DateTime().fake();

		//-- Execute Function (Act)
		let test_new_thing = ThingBuilder::new(thing_name.clone())
			.description(thing_description.clone())
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

	// TODO: Test errors
}
