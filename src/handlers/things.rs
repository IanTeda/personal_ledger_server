//! Thing handler for receiving a request and providing a respond
//!
//! A template for creating a CRUD route.
//!
//! * `C`reate implements `POST`
//! * `R`ead implements `GET`
//! * `U`pdate implements `PUT/PATCH`
//! * `D`elete implements `DELETE`
//!
//! |- NAME -|- DESCRIPTION                                                    -|- SQL EQUIVALENT -|
//! | Create | Adds one or more new entries                                     | Insert           |
//! | Read   | Retrieves entries that match certain criteria (if there are any) | Select           |
//! | Update | Changes specific fields in existing entries                      | Update           |
//! | Delete | Entirely removes one or more existing entries                    | Delete           |
//! ---

#![allow(unused)] // For beginning only.

use crate::{prelude::*, services};

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use tracing::info;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::domain::{Thing, ThingBuilder, ThingDescription, ThingName};

#[derive(serde::Deserialize, Debug)]
pub struct ThingFormData {
	pub name: String,
	pub description: String
}

impl TryFrom<ThingFormData> for Thing {
	type Error = Error;

    fn try_from(form: ThingFormData) -> Result<Self> {
        let name = ThingName::parse(form.name)?;
        let description = ThingDescription::parse(form.description)?;

		Ok(Self {
			id: Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)),
			name,
			description: Some(description),
			created_at: Utc::now(),
			updated_at: Utc::now()
		})
    }
}

/// Handle `(POST): api/v1/thing` post requests and respond with a thing json
/// 
/// # Create Thing
/// 
/// Take post request to the endpoint, forward onto the database service and
/// provide a HTTP Response
/// 
/// # Parameter
/// 
/// * `form` - an Actix web form struct
/// * `pool` - an Actix web data wrapper around a Postgres connection pool
/// ---
#[tracing::instrument(
    name = "POST thing handler."
    skip(form, pool),
    fields(
        thing_name = %form.name,
		thing_description = %form.description
    )
)]
/// Handle [POST] `api/v1/thing` requests and respond with a json collection of 
/// the Thing created.
// #[post("")]
pub async fn create(
	form: web::Form<ThingFormData>,
	pool: web::Data<PgPool>,
) -> HttpResponse {
	// let name = ThingName::parse(form.name)?;
	let name = match ThingName::parse(&form.name) {
		Ok(name) => name,
		Err(_) => return HttpResponse::BadRequest().finish(),
	};

	// let description = ThingDescription::parse(form.description)?;
	let description = match ThingDescription::parse(&form.description) {
		Ok(description) => description,
		Err(_) => return HttpResponse::BadRequest().finish(),
	};

	let new_thing = match ThingBuilder::new(name)
    	.description(description)
    	.build() {
			Ok(thing) => thing,
			Err(_) => return HttpResponse::BadRequest().finish(),
		};

	match services::things::insert(&new_thing, &pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }

	// println!("{new_thing:#?}");

	// HttpResponse::Ok().body("Create a thing...")
}

/// Handle `api/v1/thing` get requests and respond with a json collection
/// 
/// # Index Thing
/// 
/// Take get request to the endpoint, forward onto the database service and
/// provide a HTTP Response
/// 
/// # Parameter
/// 
/// * `pool` - an Actix web data wrapper around a Postgres connection pool
/// ---
#[tracing::instrument(name = "Index things")]
#[get("")]
pub async fn index() -> impl Responder {
	HttpResponse::Ok().body("Respond with a list (index) of things...")
}

/// Read a thing with `thing_id``
///
/// Return a thing by ID
///
#[tracing::instrument(
    name = "Read a things"
    skip(info),
    // fields(
    //     thing_id = %info.thing_id,
    // )
)]
#[get("{thing_id}")]
pub async fn read(info: web::Path<ThingFormData>) -> HttpResponse {
	HttpResponse::Ok().body("Find a Thing by {thing_id} and return instance...")
}

/// Update a Thing instance
///
/// Find a Thing by {thing_id}, update and return instance
///
#[tracing::instrument(name = "Update things")]
#[put("{thing_id}")]
pub async fn update() -> impl Responder {
	HttpResponse::Ok()
		.body("Find a Thing by {thing_id}, update and return instance...")
}

/// Delete a Thing by thing_id
///
/// Find a Thing by {thing_id}, update and return confirmation
///
#[tracing::instrument(name = "Delete things")]
#[delete{"{thing_id}"}]
pub async fn delete() -> impl Responder {
	HttpResponse::Ok()
		.body("Find a Thing by {thing_id}, update and return confirmation...")
}

#[cfg(test)]
pub mod tests {
	// Bring file/module functions into unit test scope
	use super::*;

	// Override with more flexible error
	pub type Result<T> = core::result::Result<T, Error>;
	pub type Error = Box<dyn std::error::Error>;

	use fake::faker::{
		chrono::en::{DateTime, DateTimeAfter},
		lorem::en::*,
	};
	use fake::Fake;

	#[sqlx::test]
	async fn create_a_thing(database: sqlx::Pool<sqlx::Postgres>) -> Result<()> {
		//-- Setup and Fixtures (Arrange)
		let name: String = Word().fake();
		let query_name = name.clone(); // TODO: This is clone ugly
		let description: String = Sentence(3..7).fake();
		let query_description = description.clone(); // TODO: This clone is ugly

		let thing = ThingFormData { name, description };

		let form = web::Form(thing);
		
		let pool = web::Data::new(database.clone());

		//-- Execute Function (Act)
		let response = create(form, pool).await;
		// println!("{response:#?}");

		//-- Checks (Assertions)
		// Check http status is ok (200)
		assert_eq!(200, response.status().as_u16());

		// Check database record matches random name and description
		let database_record = sqlx::query!(
			r#"
				SELECT * 
				FROM things 
				WHERE name = $1
			"#,
			&query_name
		)
		.fetch_one(&database)
		.await?;
		// println!("{database_record:#?}");

		assert_eq!(database_record.name, query_name);
		assert_eq!(database_record.description.unwrap(), query_description);

		Ok(())
	}
}