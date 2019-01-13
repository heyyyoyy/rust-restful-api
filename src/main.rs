#![feature(proc_macro_hygiene, plugin, decl_macro, custom_attribute)]


#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod schema;
mod posts;
mod db;

fn main() {
    dotenv::dotenv().ok();
    posts::routes::create_routes().launch();
}