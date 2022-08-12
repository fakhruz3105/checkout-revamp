#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use diesel_migrations::{embed_migrations};

// use schema::todos;
use std::{sync::Mutex};

// Start of DB example
// use super::db::{};
#[macro_use]
extern crate diesel;
#[macro_use] 
extern crate diesel_migrations;
extern crate chrono;
extern crate dotenv;

embed_migrations!("./migrations/");

use diesel::prelude::*;
use dotenv::dotenv;

pub mod schema;
pub mod db;
pub mod commands;

pub struct AppState {
  conn: Mutex<SqliteConnection>,
}

fn main() {
  dotenv().ok();
  let conn = db::init::establish_connection();
    let state = AppState {
      conn: Mutex::new(db::init::establish_connection()),
    };

    // embedded_migrations::run(&conn);
    diesel_migrations::run_pending_migrations(&conn).expect("Error migrating");
    tauri::Builder::default()
      .manage(state)
      .invoke_handler(tauri::generate_handler![
        commands::user::user_create,
        commands::user::get_users
      ])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
