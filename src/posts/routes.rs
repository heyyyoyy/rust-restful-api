use super::handlers;
use rocket::Rocket;
use serde::Serialize;
use std::io::Cursor;
use serde_json;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::{ContentType, Status};


#[derive(Serialize)]
#[serde(tag = "status", content = "result", rename_all="lowercase")]
pub enum JsonResponse<T, E>
    where
        T: Serialize,
        E: Serialize
{
    Success(T),
    Error(E)
}

impl<'r, T: Serialize, E: Serialize> Responder<'r> for JsonResponse<T, E> {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        let json = serde_json::to_string(&self).expect("error convert struct");
        Response::build()
            .status(Status::Ok)
            .header(ContentType::JSON)
            .sized_body(Cursor::new(json))
            .ok()
    }
}

pub fn create_routes(server: Rocket) -> Rocket{
    server
        .mount("/posts",
               routes![handlers::all,
                              handlers::create_post,
                              handlers::put_post,
                              handlers::delete_post,
                              handlers::get])
}
