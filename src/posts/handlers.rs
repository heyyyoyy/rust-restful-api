use super::Post;
use rocket_contrib::json::{Json, JsonValue};
use rocket::http::Status;
use super::super::db::DbConn;
use super::queries;


#[get("/")]
pub fn all(con: DbConn) -> Result<Json<Vec<Post>>, Status> {
    queries::all(&con)
        .map(|post| Json(post))
        .map_err(|_| Status::BadRequest)
}

#[get("/<id>")]
pub fn get(id: i32, con: DbConn) -> Result<Json<Post>, Status> {
    queries::get(id, &con)
        .map(|post| Json(post))
        .map_err(|_| Status::BadRequest)
}

#[post("/", format = "application/json", data = "<post>")]
pub fn create_post(con: DbConn, post: Json<Post>) -> Result<Json<Post>, Status>{
    queries::create(post.into_inner(), &con)
        .map(|post| Json(post))
        .map_err(|_| Status::BadRequest)
}

#[put("/<id>", format = "application/json", data = "<person>")]
pub fn put_post(id: i32, person: Json<Post>, con: DbConn) -> Result<Json<Post>, Status> {
    queries::update(id, person.into_inner(), &con)
        .map(|post| Json(post))
        .map_err(|_| Status::BadRequest)
}


#[delete("/<id>", format = "application/json")]
pub fn delete_post(id: i32, con: DbConn) -> Result<Json<JsonValue>, Json<JsonValue>> {
    queries::delete(id, &con)
        .map(|num| Json(json!({"status": num.to_string()})))
        .map_err(|error| Json(json!({"status": error.to_string()})))
}
