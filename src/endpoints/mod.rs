use rocket::{Rocket, Build, response::Redirect};

use std::collections::HashMap;
use rocket_dyn_templates::Template;

pub mod util;
pub mod user;
pub mod auth;
pub mod devices;

#[derive(Responder)]
pub enum Respondable {
    Template(Template),
    Status(rocket::http::Status),
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
impl From<rocket::http::Status> for Respondable {
    fn from(s: rocket::http::Status) -> Self {
        Respondable::Status(s)
    }
}

#[get("/")]
pub fn home() -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Home");
    Template::render("home", &context)
}

pub fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    return rocket
        // Index & assets
        .mount("/", routes![home])
        // Auth
        .mount("/", routes![auth::login, auth::login_post, auth::signup, auth::signup_post, auth::logout])
        // Views
        .mount("/", routes![user::user, user::user_self, user::users])
        // API
        .mount("/api", routes![devices::device, devices::device_post]);
}