//! Thing handler for receiving a request and providing a response
//!
//! A template for creating a `CRUD` route.
//!
//! * `C`reate implements `POST`
//! * `R`ead implements `GET`
//! * `U`pdate implements `PUT/PATCH`
//! * `D`elete implements `DELETE`
//!
//! |- NAME -|- DESCRIPTION                                                    -|- SQL EQUIVALENT -|
//! |--------|------------------------------------------------------------------|------------------|
//! | Create | Adds one or more new entries                                     | Insert           |
//! | Read   | Retrieves entries that match certain criteria (if there are any) | Select           |
//! | Update | Changes specific fields in existing entries                      | Update           |
//! | Delete | Entirely removes one or more existing entries                    | Delete           |
//! ---

#![allow(unused)] // For beginning only.

use crate::{prelude::*, services};

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use actix_web::web::{Data, Form};
use chrono::Utc;
use sqlx::PgPool;
use strum::Display;
use tracing::info;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;
use crate::configuration::DefaultApplicationSettings;

use crate::domain::{Thing, ThingBuilder, ThingDescription, ThingName};
use crate::services::things::get_by_id;

/// Expected Thing form struct.
#[derive(serde::Deserialize, Debug, PartialEq)]
pub struct ThingFormData {
	/// Name of the `Thing` as a `String`
	pub name: String,
	/// Description of the `Thing` as a `String`
	pub description: String
}

/// Optional Thing URL parameters.
#[derive(serde::Deserialize, Debug)]
pub struct ThingsParameters {
	id: Option<Uuid>,
    limit: Option<i64>,
	offset: Option<i64>
}

/// Handle `[POST] api/v1/thing` post requests and respond with a thing json
/// 
/// # Create Thing
/// 
/// Take post request to the endpoint, forward onto the database service and
/// provide an HTTP Response
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
pub async fn create(
	form: Form<ThingFormData>,
	pool: Data<PgPool>,
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

	// TODO: is a type conversion better than a builder?
	// let new_thing = match form.0.try_into() {
    //     Ok(form) => form,
    //     Err(_) => return HttpResponse::BadRequest().finish(),
    // };

	let new_thing = match ThingBuilder::new(name)
    	.description(description)
    	.build() {
			Ok(thing) => thing,
			Err(_) => return HttpResponse::BadRequest().finish(),
		};

	match services::things::insert(&new_thing, &pool).await {
        // Ok(_) => HttpResponse::Ok().finish(), // web::Json(obj)
		Ok(thing) => HttpResponse::Ok().json(thing),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }

	// println!("{new_thing:#?}");

	// HttpResponse::Ok().body("Create a thing...")
}

/// Handle `[GET] api/v1/thing` get requests and respond with a json collection
/// 
/// # Index Thing
/// 
/// Take get request to the endpoint, forward onto the database service and
/// provide an HTTP Response
/// 
/// # Parameter
///
/// * `parameters` - A collection of optional URL parameters defined in `ThingsParameters`
/// * `pool` - an Actix web data wrapper around a Postgres connection pool
/// ---
#[tracing::instrument(
    name = "GET index thing handler."
    skip(parameters, pool),
    // fields(
    //     query_limit = %parameters.limit.unwrap_or(10), // TODO: i64 does not have a display trait
	// 	query_offset = %parameters.offset.unwrap_or(0)
    // )
)]
pub async fn index(
	parameters: web::Query<ThingsParameters>,
	pool: Data<PgPool>
) -> HttpResponse {
	let limit = parameters.limit.unwrap_or(10);
	let offset = parameters.offset.unwrap_or(0);

	match services::things::index(&limit, &offset, &pool).await {
		Ok(things) => HttpResponse::Ok().json(things),
		Err(_) => HttpResponse::InternalServerError().finish(),
	}
}

/// Read a thing with `thing_id``
///
/// Return a thing by ID
///
#[tracing::instrument(
    name = "Read a things"
	skip(parameters, pool),
    // fields(
    //     thing_id = %info.thing_id,
    // )
)]
pub async fn read_by_id(
	parameters: web::Query<ThingsParameters>,
	pool: Data<PgPool>
) -> HttpResponse {
	let id = parameters.id.unwrap();
	// 	Ok(id) => id,
	// 	Err(_) => HttpResponse::InternalServerError().finish(),
	// };

	match get_by_id(&id, &pool).await {
		Ok(thing) => HttpResponse::Ok().json(thing),
		Err(_) => HttpResponse::InternalServerError().finish(),
	}
	// HttpResponse::Ok().body("Find a Thing by {thing_id} and return instance...")
}

/// Update a Thing instance
///
/// Find a Thing by {thing_id}, update and return instance
///
#[tracing::instrument(name = "Update things")]
// #[put("{thing_id}")]
pub async fn update() -> impl Responder {
	HttpResponse::Ok()
		.body("Find a Thing by {thing_id}, update and return instance...")
}

/// Delete a Thing by thing_id
///
/// Find a Thing by {thing_id}, update and return confirmation
///
#[tracing::instrument(name = "Delete things")]
// #[delete{"{thing_id}"}]
pub async fn delete_by_id() -> impl Responder {
	HttpResponse::Ok()
		.body("Find a Thing by {thing_id}, update and return confirmation...")
}

#[cfg(test)]
pub mod tests {
	use chrono::DateTime;
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

	use crate::services::things::tests::create_random_test_thing;
	use actix_web::{body, test, web, App, rt::pin};
	use actix_web::body::MessageBody;
	use serde_json::json;
	use crate::services::things::insert;

	#[sqlx::test]
	async fn create_a_thing(database: sqlx::Pool<sqlx::Postgres>) -> Result<()> {
		//-- Setup and Fixtures (Arrange)
		let name: String = Word().fake();
		let query_name = name.clone(); // TODO: This is clone ugly
		let description: String = Sentence(3..7).fake();
		let query_description = description.clone(); // TODO: This clone is ugly

		let thing = ThingFormData { name, description };

		let form = Form(thing);
		
		let pool = Data::new(database.clone());

		//-- Execute Function (Act)
		let response = create(form, pool).await;
		// println!("{response:#?}");

		//-- Checks (Assertions)
		// Check http status is success and ok (200)
		assert_eq!(200, response.status().as_u16());
		assert!(response.status().is_success());

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

		assert_eq!(query_name, database_record.name);
		assert_eq!(query_description, database_record.description.unwrap());

		// Check return body json equals query name and description
		let body = response.into_body().try_into_bytes().unwrap();
		// pin!(body);
		let response_thing: Thing = serde_json::from_slice(&body).unwrap();

		assert_eq!(&query_name, response_thing.name.as_ref());
		assert_eq!(&query_description, response_thing.description.unwrap().as_ref());

		Ok(())
	}

	#[sqlx::test]
	async fn get_thing_index(database: sqlx::Pool<sqlx::Postgres>) -> Result<()> {
		//-- Setup and Fixtures (Arrange)
		// Random number of Things
		let random_count: i64 = (10..30).fake::<i64>();
		// Create vector of Things for test assertions
		let mut test_vec: Vec<Thing> = Vec::new();
		// Iterate over random number
		for _count in 0..random_count {
			// Create a test instance
			let test_thing = create_random_test_thing().await?;
			// Add Thing to database
			insert(&test_thing, &database).await?;
			// Add thing to vector
			test_vec.push(test_thing);
		}

		//-- Execute Function (Act)
		// Random query limit
		// TODO: Does the limit need to at least be 1?
		let random_limit = (0..random_count).fake::<i64>();
		// Random offset
		let random_offset = (0..random_count).fake::<i64>();
		// Build URL parameters
		let web_parameters = web::Query( ThingsParameters {
			id: None,
			limit: Some(random_limit),
			offset: Some(random_offset)
		});
		// Wrap database around Actix Data type
		let pool = Data::new(database.clone());
		let response = index(web_parameters, pool).await;
		println!("{response:#?}");
		let body = response.into_body();
		println!("{body:#?}");


		Ok(())
	}

	#[sqlx::test]
	async fn read_thing_by_id(database: sqlx::Pool<sqlx::Postgres>) -> Result<()> {
		// Create a test instance
		let test_thing = create_random_test_thing().await?;
		// Add Thing to database
		insert(&test_thing, &database).await?;

		//-- Execute Function (Act)
		let web_parameters = web::Query( ThingsParameters {
			id: Some(test_thing.id),
			limit: None,
			offset: None
		});
		let pool = Data::new(database.clone());
		let response = read_by_id(web_parameters, pool).await;

		//-- Checks (Assertions)
		// Check http response is success
		assert!(response.status().is_success());
		// Check http status is ok (200)
		assert_eq!(200, response.status().as_u16());
		let body = response.into_body().try_into_bytes().unwrap();
		// pin!(body);
		let response_thing: Thing = serde_json::from_slice(&body).unwrap();

		assert_eq!(test_thing.id, response_thing.id);
		assert_eq!(test_thing.name, response_thing.name);
		assert_eq!(test_thing.description, response_thing.description);
		assert_eq!(
			test_thing.created_at.timestamp_millis(),
			response_thing.created_at.timestamp_millis()
		);
		assert_eq!(
			test_thing.updated_at.timestamp_millis(),
			response_thing.updated_at.timestamp_millis()
		);

		Ok(())
	}

}