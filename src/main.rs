#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate rusqlite;
extern crate r2d2;

use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

mod db;
mod tests;
mod error;
mod endpoints;

use db::Db;

fn rocket() -> rocket::Rocket<rocket::Build> {
    let db = match Db::new() {
        Ok(db) => db,
        Err(e) => panic!("Error: {:?}", e),
    };
    db.init_test().expect("Failed to init test db");
    let rocket = rocket::build()
        .attach(Template::fairing())
        .mount("/static", FileServer::from(relative!("static")))
        .manage(db);
    endpoints::mount(rocket)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket()
        .launch()
        .await;

    Ok(())
}
