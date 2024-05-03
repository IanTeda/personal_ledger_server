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

// use crate::prelude::*;
use crate::services::things::*;
use crate::domain::{NewThing, ThingName};

use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;
use unicode_segmentation::UnicodeSegmentation;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ThingFormData {
	pub name: String,
	pub description: String
}

/// Handle `api/v1/thing` post requests and respond with a thing json
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
    name = "Create a new things"
    skip(form, pool),
    fields(
        thing_name = %form.name,
		thing_description = %form.description
    )
)]
#[post("")]
pub async fn create(
	form: web::Form<ThingFormData>,
	pool: web::Data<PgPool>,
) -> HttpResponse {
	let thing: Thing = ThingBuilder::new(&form.name)
		.description(&form.description)
		.build()
		.expect("Error building a new thing");
	//  println!("{thing:#?}");

	let record = Thing::insert(&thing, &pool.as_ref()).await;
	// println!("{record:#?}");

	match record {
		Ok(_) => HttpResponse::Ok().json(thing),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
	}
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

/// Returns `true` if the input satisfies all our validation constraints
/// on subscriber names, `false` otherwise.
pub fn is_valid_name(s: &str) -> bool {
	// `.trim()` returns a view over the input `s` without trailing
	// whitespace-like characters.
	// `.is_empty` checks if the view contains any character.
	let is_empty_or_whitespace = s.trim().is_empty();

	// A grapheme is defined by the Unicode standard as a "user-perceived"
	// character: `å` is a single grapheme, but it is composed of two characters
	// (`a` and ``).
	//
	// `graphemes` returns an iterator over the graphemes in the input `s`.
	// `true` specifies that we want to use the extended grapheme definition set,
	// the recommended one.
	let is_too_long = s.graphemes(true).count() > 256;

	// Iterate over all characters in the input `s` to check if any of them matches
	// one of the characters in the forbidden array.
	let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
	let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

	// Return `false` if any of our conditions have been violated
	!(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}