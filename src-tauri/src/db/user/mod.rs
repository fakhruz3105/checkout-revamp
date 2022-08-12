pub mod models;

use crate::schema::{users};
use diesel::prelude::*;
use models::{NewUser, User};

pub fn user_create(conn: &SqliteConnection) -> String {
  let new_person = NewUser { username: "", password: "" };
  let todo = diesel::insert_into(users::table)
      .values(&new_person)
      .execute(conn)
      .expect("Error saving new post");
  let todo_json  = serde_json::to_string(&todo).unwrap();
  todo_json
}

pub fn all_users(conn: &SqliteConnection) -> String {
  let users = users::table.load::<User>(conn);
  match users {
    Ok(ref us) => serde_json::to_string(us).unwrap(),
    Err(_) => "".to_string()
  }
}
