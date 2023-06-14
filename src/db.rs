use bcrypt::{hash, verify, DEFAULT_COST};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Serialize;
use std::{env, collections::HashMap};
use chrono::prelude::*;
use uuid::Uuid;
use serde_json::value::Value as JsonValue;

use crate::error::DatastoreError;

pub struct Db {
    pool: r2d2::Pool<SqliteConnectionManager>,
}

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Device {
    pub id: String,
    pub user_id: i32,
    pub name: String,
}

// Data is a JSON object of the shape:
// {
//  "time": {
//    "category 1": <float duration in seconds>,
//    "category 2": <float duration in seconds>,
//  },
// }
#[derive(Debug, Serialize)]
pub struct Activity {
    pub id: i32,
    pub timestamp: DateTime<Utc>,
    pub device_id: Uuid,
    pub data: JsonValue,
    pub rules: HashMap<String, String>,
}

type Result<T> = std::result::Result<T, DatastoreError>;

impl Db {
    pub fn new() -> Result<Db> {
        let manager = if let Ok(database_url) = env::var("DATABASE_URL") {
            SqliteConnectionManager::file(database_url)
        } else {
            log::warn!("DATABASE_URL was unset, using in-memory database");
            SqliteConnectionManager::memory()
        };

        let pool = r2d2::Pool::new(manager).expect("Failed to create pool.");
        let db = Db { pool };
        db.init()?;
        Ok(db)
    }

    fn conn(&self) -> Result<PooledConnection<SqliteConnectionManager>> {
        Ok(self.pool.get()?)
    }

    pub fn init(&self) -> Result<()> {
        // Create user table
        self.conn()?.execute(
            "CREATE TABLE IF NOT EXISTS user (
                  id              INTEGER PRIMARY KEY,
                  username        TEXT NOT NULL UNIQUE,
                  email           TEXT NOT NULL UNIQUE,
                  password        TEXT NOT NULL
             )",
            [],
        )?;
        // Create device table
        // It's ID is a UUID
        self.conn()?.execute(
            "CREATE TABLE IF NOT EXISTS device (
                  id              TEXT PRIMARY KEY,
                  user_id         INTEGER NOT NULL,
                  name            TEXT NOT NULL,
                  FOREIGN KEY(user_id) REFERENCES user(id)
             )",
            [],
        )?;
        // Create activity table
        // Contains rows with hourly resolution of activity for each device
        self.conn()?.execute(
            "CREATE TABLE IF NOT EXISTS activity (
                  id              INTEGER PRIMARY KEY,
                  timestamp       INTEGER NOT NULL,
                  device_id       TEXT NOT NULL,
                  data            BLOB NOT NULL,
                  FOREIGN KEY(device_id) REFERENCES device(id)
             )",
            [],
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn init_test(&self) -> Result<()> {
        // Creates a test user
        self.add_user("test", "test@example.com", "test")
    }

    pub fn add_user(&self, username: &str, email: &str, password: &str) -> Result<()> {
        let hashed_password = hash(password, DEFAULT_COST).unwrap();

        self.conn()?.execute(
            "INSERT INTO user (username, email, password) VALUES (?1, ?2, ?3)",
            params![username, email, hashed_password],
        )?;
        Ok(())
    }

    pub fn check_password(&self, username: &str, password: &str) -> Result<bool> {
        match self.get_user(username) {
            Ok(user) => Ok(verify(password, &user.password).unwrap()),
            Err(_) => Ok(false),
        }
    }

    pub fn get_user(&self, username: &str) -> Result<User> {
        let conn = self.conn()?;
        let mut stmt =
            conn.prepare("SELECT id, username, email, password FROM user WHERE username = ?1")?;
        let mut user_iter = stmt.query_map(params![username], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                password: row.get(3)?,
            })
        })?;

        match user_iter.next() {
            Some(user) => Ok(user?),
            None => Err(DatastoreError::Rusqlite(
                rusqlite::Error::QueryReturnedNoRows,
            )),
        }
    }

    pub fn get_users(&self) -> Result<Vec<User>> {
        let conn = self.conn()?;
        let mut stmt =
            conn.prepare("SELECT id, username, email, password FROM user")?;
        let mut user_iter = stmt.query_map(params![], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                password: row.get(3)?,
            })
        })?;

        let mut users = Vec::new();
        while let Some(user) = user_iter.next() {
            users.push(user?);
        }
        Ok(users)
    }

    pub fn add_device(&self, user_id: i32, device_id: Uuid, name: &str) -> Result<()> {
        self.conn()?.execute(
            "INSERT INTO device (id, user_id, name) VALUES (?1, ?2, ?3)",
            params![device_id.to_string(), user_id, name],
        )?;
        Ok(())
    }

    pub fn get_devices(&self, user_id: i32) -> Result<Vec<Device>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare("SELECT id, user_id, name FROM device WHERE user_id = ?1")?;
        let mut device_iter = stmt.query_map(params![user_id], |row| {
            Ok(Device {
                id: row.get(0)?,
                user_id: row.get(1)?,
                name: row.get(2)?,
            })
        })?;

        let mut devices = Vec::new();
        while let Some(device) = device_iter.next() {
            devices.push(device?);
        }
        Ok(devices)
    }

    // Add more functions to handle claims and other stuff...
}
