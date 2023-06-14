use std::collections::HashMap;

use rocket::{response::Redirect, form::Form, State, http::{Cookie, CookieJar}};
use rocket_dyn_templates::Template;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use crate::{error::DatastoreError, db::Db};

#[derive(FromForm)]
pub struct Login {
    username: String,
    password: String,
}

#[derive(FromForm)]
pub struct Signup {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
}

#[get("/signup")]
pub fn signup() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Signup");
    Template::render("signup", &context)
}

#[post("/signup", data = "<user_form>")]
pub fn signup_post(db: &State<Db>, user_form: Form<Signup>) -> Result<Redirect, DatastoreError> {
    match db.add_user(&user_form.username, &user_form.email, &user_form.password) {
        Ok(_) => Ok(Redirect::to(uri!(super::user::user(user_form.username.to_string())))),
        Err(_) => Err(DatastoreError::UserAlreadyExists { username: user_form.username.to_string() }),
    }
}

#[get("/login")]
pub fn login() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Login");
    Template::render("login", &context)
}

#[post("/login", data = "<login_form>")]
pub fn login_post(db: &State<Db>, login_form: Form<Login>, cookies: &CookieJar) -> Result<Redirect, String> {
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

            Ok(Redirect::to(uri!(super::user::user(login_form.username.to_string()))))
        }
        _ => Err("Invalid user or password".to_string()),
    }
}

#[get("/logout")]
pub fn logout(cookies: &CookieJar) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    cookies.remove_private(Cookie::named("jwt"));
    Redirect::to(uri!(super::home))
}