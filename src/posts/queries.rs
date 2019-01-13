use diesel;
use diesel::prelude::*;
use super::super::schema;
use super::Post;


pub fn all(con: &PgConnection) -> QueryResult<Vec<Post>> {
    schema::post::table.load::<Post>(con)
}

pub fn create(post: Post, con: &PgConnection) -> QueryResult<Post> {
    diesel::insert_into(schema::post::table)
        .values(&post)
        .get_result(con)
}

pub fn update(id: i32, post: Post, con: &PgConnection) -> QueryResult<Post> {
    diesel::update(schema::post::table.find(id))
        .set(&post)
        .get_result(con)
}

pub fn get(id: i32, con: &PgConnection) -> QueryResult<Post> {
    schema::post::table.find(id).get_result::<Post>(con)
}

pub fn delete(id: i32, con: &PgConnection) -> QueryResult<usize> {
    diesel::delete(schema::post::table.find(id))
        .execute(con)
}