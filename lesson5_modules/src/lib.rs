#![allow(dead_code, unused_variables)]

mod database;
mod auth_utils;

pub use auth_utils::models::Credentials;
use auth_utils::login;
use database::{Status, connect_to_database};

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = connect_to_database() {
        login(creds);
    }
}

