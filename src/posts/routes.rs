use super::handlers;
use rocket;
use super::super::db;


pub fn create_routes() -> rocket::Rocket{
    rocket::ignite()
        .manage(db::init_pool())
        .mount("/posts",
               routes![handlers::all,
                              handlers::create_post,
                              handlers::put_post,
                              handlers::delete_post,
                              handlers::get])
}
