#![feature(plugin, const_fn, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use crate::graphql::create_schema;

mod db;
mod graphql;
mod routes;

use dotenv::dotenv;
use std::env;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("missing env variable DATABASE_URL");

    let pool = db::init_pool(database_url);

    rocket::ignite()
        .manage(pool)
        .manage(create_schema())
        .mount(
            "/api/v1/users",
            routes![
                routes::rest::get_all,
                routes::rest::new_user,
                routes::rest::find_user
            ],
        )
        .mount(
            "/",
            routes![
                routes::graphql::graphiql,
                routes::graphql::get_graphql_handler,
                routes::graphql::post_graphql_handler
            ],
        )
}

fn main() {
    rocket().launch();
}
