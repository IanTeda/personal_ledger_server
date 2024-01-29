use actix_web::web;

use crate::handlers::*;

pub fn ping(config: &mut web::ServiceConfig) {
    config.service(ping::index);
}
