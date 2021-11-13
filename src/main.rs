#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use reqwest;
use rocket::{Build, Data, Request, Rocket, State, catch, catchers, fairing::{self, Fairing, Info, Kind}, get, http::{Cookie, CookieJar, Method}, launch, post, response::{content::Html, status::Created}, routes, serde::json::Json, uri};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use std::str;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;

#[get("/")]
fn hello_world() -> &'static str {
    "hello world!"
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/api", routes![
        hello_world,
    ])
}
