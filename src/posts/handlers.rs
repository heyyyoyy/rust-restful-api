use super::models::{NewPost, Post};
use rocket_contrib::json::{Json};
use super::super::db::DbConn;
use super::queries;
use super::routes::JsonResponse;


#[get("/")]
pub fn all(con: DbConn) -> JsonResponse<Vec<Post>, String> {
    queries::all(&con)
        .map_or_else(|err| JsonResponse::Error(err.to_string()),
        |item| JsonResponse::Success(item))
}

#[get("/<id>")]
pub fn get(id: i32, con: DbConn) -> JsonResponse<Post, String> {
    queries::get(id, &con)
        .map_or_else(|err| JsonResponse::Error(err.to_string()),
        |post| JsonResponse::Success(post))
}

#[post("/", format = "application/json", data = "<post>")]
pub fn create_post(con: DbConn, post: Json<NewPost>) -> JsonResponse<Post, String> {
    queries::create(post.into_inner(), &con)
        .map_or_else(|err| JsonResponse::Error(err.to_string()),
        |post| JsonResponse::Success(post))
}

#[put("/<id>", format = "application/json", data = "<post>")]
pub fn put_post(id: i32, post: Json<NewPost>, con: DbConn) -> JsonResponse<Post, String> {
    queries::update(id, post.into_inner(), &con)
        .map_or_else(|err| JsonResponse::Error(err.to_string()),
                     |post| JsonResponse::Success(post))
}


#[delete("/<id>")]
pub fn delete_post(id: i32, con: DbConn) -> JsonResponse<usize, String> {
    queries::delete(id, &con)
        .map_or_else(|err| JsonResponse::Error(err.to_string()),
                     |num| {
                         if num == 0 {
                             JsonResponse::Error("Not value".to_string())
                         } else {
                             JsonResponse::Success(num)
                         }
                     })
}
