use super::handlers;
use rocket::Rocket;


pub fn create_routes(server: Rocket) -> Rocket{
    server
        .mount("/posts",
               routes![handlers::all,
                              handlers::create_post,
                              handlers::put_post,
                              handlers::delete_post,
                              handlers::get])
}
