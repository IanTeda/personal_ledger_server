//! Consuming builder pattern example.
//! See video: https://youtu.be/Z_3WOSiYYFY
//! [jeremychone-channel/rust-builder](https://github.com/jeremychone-channel/rust-builder/tree/main)
//! [letsgetrusty/builder_pattern](https://github.com/letsgetrusty/builder_pattern)
//! https://www.youtube.com/watch?v=5DWU-56mjmg

// #![allow(unused)] // For development only

use crate::prelude::*;
use chrono::prelude::*;
use uuid::Uuid;

#[derive(Debug)]
pub struct Thing {
    id: Uuid,
    name: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct ThingBuilder {
    id: Option<Uuid>,
    name: Option<String>,
    description: Option<String>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>
}

// Implementation of the default thing. You can also use the #[derive(default)]
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

impl ThingBuilder {

    pub fn new(name: impl Into<String>) -> Self {
        ThingBuilder {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    pub fn id(mut self, id: Uuid) -> Self {
		let _ = self.id.insert(id.into());
		self
	}

    // https://github.com/Graetdragonn/kitsune/blob/9cc036434a4a136c56ac3f097bdf47c7b9ef2cda/kitsune/src/util/mod.rs#L29
    pub fn id_set_date_time(mut self, date_time: DateTime<Utc> ) -> Self {
        // println!("{date_time:#?}");
        let uuid_timestamp: uuid::Timestamp = uuid::Timestamp::from_unix(
            uuid::NoContext,
            date_time.timestamp() as u64,
            date_time.timestamp_nanos_opt().unwrap() as u32,
        );
        // println!("{uuid_timestamp:#?}");
        let id = Uuid::new_v7(uuid_timestamp);
        // println!("{id:#?}");
        let _ = self.id.insert(id.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
		let _ = self.name.insert(name.into());
		self
	}

    pub fn description(mut self, description: impl Into<String>) -> Self {
		let _ = self.description.insert(description.into());
		self
	}

    pub fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
		let _ = self.created_at.insert(created_at);
		self
	}

    pub fn updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
		let _ = self.updated_at.insert(updated_at);
		self
	}

    pub fn build(self) -> Result<Thing> {
        // Run time check that `id` is not null
        let Some(id) = self.id else {
            return Err(Error::Static("No Uuid provided")); // TODO: update with static error
        };

        // Run time check that `name` is not null
        let Some(name) = self.name.as_ref() else {
            return Err(Error::Static("No name provided"));  // TODO: update with static error
        };

        // Run time check that `created_at` is not null
        let Some(created_at) = self.created_at else {
            return Err(Error::Static("No created_at date provided"));  // TODO: update with static error
        };

        // Run time check that `created_at` is not null
        let Some(updated_at) = self.updated_at else {
            return Err(Error::Static("No updated_at date provided"));  // TODO: update with static error
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

#[cfg(test)]
pub mod tests {

    use fake::faker::chrono::en::DateTimeAfter;
    use fake::faker::{chrono::en::DateTime, lorem::en::*};
    use fake::Fake;

    pub type Result<T> = core::result::Result<T, Error>;
	pub type Error = Box<dyn std::error::Error>; // For tests.

    use super::*;

    // Test creating a new Thing without a description
    #[actix_rt::test]
    async fn create_new_thing() -> Result<()> {
        //-- Setup and Fixtures
        let thing_name: &str = Word().fake();

        //-- Execute Function
        let test_new_thing: Thing = ThingBuilder::new(thing_name).build()?;
        // println!("{test_new_thing:#?}");

        //-- Checks
        assert_eq!(test_new_thing.name, thing_name);
        assert_eq!(test_new_thing.description, None);

        Ok(())
    }

    // Test creating a new Thing with a description
    #[actix_rt::test]
    async fn create_new_thing_with_description() -> Result<()> {
        //-- Setup and Fixtures
        let thing_name: &str = Word().fake();
        let thing_description: String = Sentence(3..7).fake();

        //-- Execute Function
        let test_new_thing: Thing = ThingBuilder::new(thing_name)
            .description(&thing_description)
            .build()?;
        // println!("{test_new_thing:#?}");

        //-- Checks
        assert_eq!(test_new_thing.name, thing_name);
        assert_eq!(test_new_thing.description.unwrap(), thing_description);

        Ok(())
    }

    // Test creating a new Thing with an id
    #[actix_rt::test]
    async fn create_new_thing_with_id() -> Result<()> {
        //-- Setup and Fixtures
        let thing_id: Uuid = Uuid::now_v7();
        let thing_name: &str = Word().fake();
        let thing_description: String = Sentence(3..7).fake();
        let thing_created_at: DateTime<Utc> = DateTime().fake();
        let thing_updated_at: DateTime<Utc> = DateTime().fake();

        //-- Execute Function
        let test_new_thing: Thing = ThingBuilder::new(thing_name)
            .id(thing_id)
            .description(&thing_description)
            .created_at(thing_created_at)
            .updated_at(thing_updated_at)
            .build()?;
        // println!("{test_new_thing:#?}");

        //-- Checks
        assert_eq!(test_new_thing.id, thing_id);
        assert_eq!(test_new_thing.name, thing_name);
        assert_eq!(test_new_thing.description.unwrap(), thing_description);
        assert_eq!(test_new_thing.created_at, thing_created_at);
        assert_eq!(test_new_thing.updated_at, thing_updated_at);

        Ok(())
    }

    // Test creating a new Thing with an timestamp id
    #[actix_rt::test]
    async fn create_new_thing_with_timestamp() -> Result<()> {
        //-- Setup and Fixtures
        let thing_datetime: DateTime<Utc> = DateTimeAfter(chrono::DateTime::UNIX_EPOCH).fake();
        // println!("{thing_datetime:#?}");
        let thing_name: &str = Word().fake();
        let thing_description: String = Sentence(3..7).fake();
        let thing_created_at: DateTime<Utc> = DateTime().fake();
        let thing_updated_at: DateTime<Utc> = DateTime().fake();

        //-- Execute Function
        let test_new_thing: Thing = ThingBuilder::new(thing_name)
            .description(&thing_description)
            .id_set_date_time(thing_datetime)
            .created_at(thing_created_at)
            .updated_at(thing_updated_at)
            .build()?;
        // println!("{test_new_thing:#?}");

        //-- Checks
        assert_eq!(test_new_thing.name, thing_name);
        assert_eq!(test_new_thing.description.unwrap(), thing_description);
        assert_eq!(test_new_thing.created_at, thing_created_at);
        assert_eq!(test_new_thing.updated_at, thing_updated_at);

        Ok(())
    }


}