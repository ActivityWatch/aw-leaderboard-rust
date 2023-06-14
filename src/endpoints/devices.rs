use rocket::{State, serde::json::Json, http::Status};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{db, endpoints::util::Context, db::Db};

#[derive(Serialize, Deserialize)]
pub struct Device {
    id: Uuid,
    user_id: i32,
    name: String,
}

#[get("/api/devices")]
pub fn device(db: &State<Db>, context: Context) -> Json<Vec<db::Device>> {
    Json(db.get_devices(context.user.unwrap().id).unwrap())
}

#[post("/api/devices", format = "json", data = "<device>")]
pub fn device_post(db: &State<Db>, device: Json<Device>, context: Context) -> Status {
    match db.add_device(context.user.unwrap().id, device.id, &device.name) {
        Ok(_) => Status::Created,
        Err(err) => {
            println!("Error adding device: {}", err);
            Status::BadRequest
        }
    }
}