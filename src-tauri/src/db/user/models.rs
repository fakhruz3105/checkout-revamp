use crate::schema::users;
use serde::{Serialize, Deserialize};

#[derive(Queryable, PartialEq, Debug, Serialize, Deserialize)]
pub struct User {
  pub id: Option<i32>,
  pub username: String,
  pub password: String,
  pub created_at: i64,
  pub updated_at: i64,
}


#[derive(Insertable, Serialize, Deserialize, Debug, Clone)]
#[table_name = "users"]
pub struct NewUser<'a> {
  pub username: &'a str,
  pub password: &'a str,
}