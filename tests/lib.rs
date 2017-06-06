#![feature(plugin, const_fn)]
#![plugin(stainless)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate diesel;
extern crate parking_lot;
extern crate uuid;
extern crate serde_json;

extern crate rust_web_boilerplate;

mod factories;
mod utils;
mod test_api_auth;
