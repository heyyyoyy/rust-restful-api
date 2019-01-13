#![allow(proc_macro_derive_resolution_fallback)]

use super::schema::post;

pub mod handlers;
pub mod routes;
pub mod queries;


#[table_name = "post"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
}
