#![feature(proc_macro_hygiene, plugin, decl_macro, custom_attribute, result_map_or_else)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use] extern crate diesel;
extern crate dotenv;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod schema;
mod posts;
mod db;

fn main() {
    dotenv::dotenv().ok();
    let server = rocket::ignite().manage(db::init_pool());
    posts::routes::create_routes(server).launch();
}