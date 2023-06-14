use rocket::{State, response::Redirect};
use rocket_dyn_templates::Template;
use serde::Serialize;

use crate::db::{Db, self};
use crate::endpoints::{util::Context, Respondable};

#[derive(Serialize)]
struct UserWithDevices {
    user: db::User,
    devices: Vec<db::Device>,
}

#[get("/user/<id>")]
pub fn user(db: &State<Db>, id: String, mut context: Context) -> Template {
    let user_profile = db.get_user(&id);
    match user_profile {
        Ok(user) => {
            context.requested = Some(serde_json::to_value(UserWithDevices {
                user: user.clone(),
                devices: db.get_devices(user.id).unwrap(),
            }).unwrap());
            Template::render("user", &context)
        },
        Err(_) => {
            context.error = Some("User not found".to_string());
            Template::render("error", &context)
        },
    }
}

#[get("/user")]
pub fn user_self(context: Context) -> Redirect {
    match context.user {
        Some(user) => Redirect::to(uri!(user(user.username))),
        None => Redirect::to(uri!(super::auth::login)),
    }
}

#[get("/users")]
pub fn users(db: &State<Db>, mut context: Context) -> Respondable {
    let users = db.get_users().unwrap();
    context.requested = Some(serde_json::to_value(users).unwrap());
    Template::render("users", &context).into()
}