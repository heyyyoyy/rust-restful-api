use super::super::schema::post;

#[table_name = "post"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
}
