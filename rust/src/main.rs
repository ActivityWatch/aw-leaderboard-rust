#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate rusqlite;
extern crate r2d2;

use std::collections::HashMap;

use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::http::{Cookie, CookieJar};
use rocket::response::{Redirect};
use rocket::{uri, State};
use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

mod db;
mod tests;
mod error;

use error::DatastoreError;

#[derive(FromForm)]
struct Login {
    username: String,
    password: String,
}

#[derive(FromForm)]
struct User {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
}

#[derive(Responder)]
enum Respondable {
    Template(Template),
    Redirect(Redirect),
}
impl From<Template> for Respondable {
    fn from(t: Template) -> Self {
        Respondable::Template(t)
    }
}
impl From<Redirect> for Respondable {
    fn from(r: Redirect) -> Self {
        Respondable::Redirect(r)
    }
}

#[get("/profile/<id>")]
fn profile(id: Option<String>, cookies: &CookieJar) -> Respondable {
    if id.is_none() {
        match get_current_user(cookies) {
            Some(user_id) => return Redirect::to(uri!(profile(user_id))).into(),
            None => return Template::render("error", &HashMap::<String, String>::new()).into(),
        }
    }
    let mut context = HashMap::new();
    context.insert("username", id);
    context.insert("user", get_current_user(cookies));
    Template::render("profile", &context).into()
}

#[get("/register")]
fn register() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Register");
    Template::render("register", &context)
}

#[post("/register", data = "<user_form>")]
fn register_post(db: &State<db::Db>, user_form: Form<User>) -> Result<Redirect, DatastoreError> {
    match db.add_user(&user_form.username, &user_form.email, &user_form.password) {
        Ok(_) => Ok(Redirect::to(uri!(profile(user_form.username.to_string())))),
        Err(_) => Err(DatastoreError::UserAlreadyExists { username: user_form.username.to_string() }),
    }
}

#[get("/login")]
fn login() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Login");
    Template::render("login", &context)
}

#[post("/login", data = "<login_form>")]
fn login_post(db: &State<db::Db>, login_form: Form<Login>, cookies: &CookieJar) -> Result<Redirect, String> {
    match db.check_password(&login_form.username, &login_form.password) {
        Ok(true) => {
            let my_claims = Claims {
                sub: login_form.username.to_owned(),
                company: "ACME".to_owned(),
            };
            let key = b"secret";
            let token = encode(
                &Header::new(Algorithm::HS256),
                &my_claims,
                &EncodingKey::from_secret(key),
            )
            .unwrap();

            cookies.add_private(Cookie::new("user_id", login_form.username.to_string()));
            cookies.add_private(Cookie::new("jwt", token));

            Ok(Redirect::to(uri!(profile(login_form.username.to_string()))))
        }
        _ => Err("Invalid user or password".to_string()),
    }
}

#[get("/logout")]
fn logout(cookies: &CookieJar) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    cookies.remove_private(Cookie::named("jwt"));
    Redirect::to(uri!(home))
}

#[get("/")]
fn home() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Home");
    Template::render("home", &context)
}

fn get_current_user(cookies: &CookieJar) -> Option<String> {
    cookies.get_private("user_id").map(|cookie| cookie.value().to_string())
}

fn rocket() -> rocket::Rocket<rocket::Build> {
    let db = match db::Db::new() {
        Ok(db) => db,
        Err(e) => panic!("Error: {:?}", e),
    };
    db.init_test().expect("Failed to init test db");
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![home, register, register_post, login, login_post, logout, profile])
        .mount("/static", FileServer::from(relative!("static")))
        .manage(db)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket()
        .launch()
        .await;

    Ok(())
}
