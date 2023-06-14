
use rocket::{request::{FromRequest, Outcome}, Request, State, http::{CookieJar}};
use serde::Serialize;
use serde_json::Value;

use crate::db::{Db, self};


// Template context
#[derive(Default, Serialize)]
pub struct Context {
    // Logged in user, if any
    pub user: Option<db::User>,
    // Error
    pub error: Option<String>,
    // A requested entity, differs by endpoint
    pub requested: Option<Value>,
}

#[rocket::async_trait]
impl <'a> FromRequest<'a> for Context {
    type Error = ();

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let db = request.guard::<&State<Db>>().await.unwrap();
        let cookies = request.guard::<&CookieJar>().await.unwrap();
        let username = cookies.get_private("user_id").map(|cookie| cookie.value().to_string());
        let user = match username {
            Some(username) => db.get_user(&username).ok(),
            None => None,
        };
        let mut context = Context::default();
        context.user = user;
        Outcome::Success(context)
    }
}

// use std::io::Cursor;
// use rocket::response::Responder;

// #[derive(Serialize, Debug)]
// pub struct HttpErrorJson {
//     #[serde(skip_serializing)]
//     status: Status,
//     message: String,
// }

// impl HttpErrorJson {
//     pub fn new(status: Status, err: String) -> HttpErrorJson {
//         HttpErrorJson {
//             status,
//             message: err,
//         }
//     }
// }

// impl<'r> Responder<'r, 'static> for HttpErrorJson {
//     fn respond_to(self, _: &Request) -> response::Result<'static> {
//         // TODO: Fix unwrap
//         let body = serde_json::to_string(&self).unwrap();
//         Response::build()
//             .status(self.status)
//             .sized_body(body.len(), Cursor::new(body))
//             .header(ContentType::new("application", "json"))
//             .ok()
//     }
// }