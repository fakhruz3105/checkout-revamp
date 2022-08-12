use crate::{AppState};
use crate::db;
use chrono::{Utc};

#[tauri::command]
pub fn user_create(name: String, age: i32, state: tauri::State<AppState>) -> String{
  let conn = state.conn.lock().unwrap();
  db::user::user_create(&conn).to_string()
}

#[tauri::command]
pub fn get_users(state: tauri::State<AppState>) -> String{
  let conn = state.conn.lock().unwrap();
  let now = Utc::now().timestamp_millis();
  println!("{:?}", now);
  db::user::all_users(&conn)
}