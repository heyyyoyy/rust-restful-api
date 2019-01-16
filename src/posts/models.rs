use super::super::schema::post;


#[derive(Serialize, Deserialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub description: String,
}


#[table_name = "post"]
#[derive(Serialize, Deserialize, Insertable, AsChangeset)]
pub struct NewPost {
    pub title: String,
    pub description: String,
}