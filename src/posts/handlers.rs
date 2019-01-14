use super::Post;
use rocket_contrib::json::{Json, JsonValue};
use super::super::db::DbConn;
use super::queries;


#[get("/")]
pub fn all(con: DbConn) -> Result<JsonValue, JsonValue> {
    queries::all(&con)
        .map(|post| json!({"status": "success", "response": post}))
        .map_err(|err| json!({"status": "error", "response": err.to_string()}))
}

#[get("/<id>")]
pub fn get(id: i32, con: DbConn) -> Result<JsonValue, JsonValue> {
    queries::get(id, &con)
        .map(|post| json!({"status": "success", "response": post}))
        .map_err(|err| json!({"status": "error", "response": err.to_string()}))
}

#[post("/", format = "application/json", data = "<post>")]
pub fn create_post(con: DbConn, post: Json<Post>) -> Result<JsonValue, JsonValue> {
    queries::create(post.into_inner(), &con)
        .map(|post| json!({"status": "success", "response": post}))
        .map_err(|err| json!({"status": "error", "response": err.to_string()}))
}

#[put("/<id>", format = "application/json", data = "<person>")]
pub fn put_post(id: i32, person: Json<Post>, con: DbConn) -> Result<JsonValue, JsonValue> {
    queries::update(id, person.into_inner(), &con)
        .map(|post| json!({"status": "success", "response": post}))
        .map_err(|err| json!({"status": "error", "response": err.to_string()}))
}


#[delete("/<id>", format = "application/json")]
pub fn delete_post(id: i32, con: DbConn) -> Result<JsonValue, JsonValue> {
    queries::delete(id, &con)
        .map(|post| json!({"status": "success", "response": post}))
        .map_err(|err| json!({"status": "error", "response": err.to_string()}))
}
