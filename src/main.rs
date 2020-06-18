#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_okapi;

use rocket_okapi::swagger_ui::*;

/// # Get Hello World
///
/// Returns a Hello World Test.
#[openapi]
#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

fn main() {
    rocket::ignite()
    .mount(
        "/",
        routes_with_openapi![
            index
        ],
    )
    .mount(
        "/swagger-ui/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "../openapi.json".to_owned(),
            ..Default::default()
        }),
    )
    .launch();
}