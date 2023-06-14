use rocket::response::Responder;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatastoreError {
    #[error("user `{username}` already exists")]
    UserAlreadyExists { username: String },
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("rusqlite error: {0}")]
    Rusqlite(#[from] rusqlite::Error),
    #[error("r2d2 error: {0}")]
    R2d2(#[from] r2d2::Error),
}

impl<'a> Responder<'a, 'static> for DatastoreError {
    fn respond_to(self, _: &'a rocket::Request<'_>) -> rocket::response::Result<'static> {
        let status = match self {
            DatastoreError::UserAlreadyExists { .. } => rocket::http::Status::Conflict,
            DatastoreError::BadRequest(_) => rocket::http::Status::BadRequest,
            DatastoreError::Rusqlite(_) => rocket::http::Status::InternalServerError,
            DatastoreError::R2d2(_) => rocket::http::Status::InternalServerError,
        };
        let msg = self.to_string();
        rocket::Response::build()
            .status(status)
            .header(rocket::http::ContentType::Plain)
            .sized_body(msg.len(), std::io::Cursor::new(msg))
            .ok()
    }
}
