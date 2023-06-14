use bcrypt::{hash, verify, DEFAULT_COST};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::{Serialize, Deserialize};
use serde_with::serde_as;
use std::{env, time::Duration, ops::Add};
use chrono::{prelude::*};
use uuid::Uuid;

use crate::error::DatastoreError;

pub struct Db {
    pool: r2d2::Pool<SqliteConnectionManager>,
}

#[derive(Debug, Serialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Device {
    pub id: Uuid,
    pub user_id: i64,
    pub name: String,
    pub last_seen: Option<DateTime<Utc>>,
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
    pub id: i64,
    pub timestamp: DateTime<Utc>,
    pub device_id: Uuid,
    pub events: Vec<Event>,
    pub ruleset_id: i64,
}

use serde_with::{DurationSeconds};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    timestamp: DateTime<Utc>,
    #[serde_as(as = "DurationSeconds<u64>")]
    duration: Duration,
    category: String,
}

#[derive(Debug, Serialize)]
pub struct Ruleset {
    pub id: i64,
    pub name: String,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Serialize)]
pub struct Rule {
    pub name: Vec<String>,
    pub regex: String,
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
        // Contains hourly rows of activity for each device
        self.conn()?.execute(
            "CREATE TABLE IF NOT EXISTS activity (
                  id              INTEGER PRIMARY KEY,
                  timestamp       INTEGER NOT NULL,
                  device_id       TEXT NOT NULL,
                  events          BLOB NOT NULL,
                  ruleset_id      INTEGER NOT NULL,
                  FOREIGN KEY(device_id) REFERENCES device(id)
                  FOREIGN KEY(ruleset_id) REFERENCES ruleset(id)
             )",
            [],
        )?;
        // Create ruleset table
        self.conn()?.execute(
            "CREATE TABLE IF NOT EXISTS ruleset (
                  id              INTEGER PRIMARY KEY,
                  user_id         INTEGER NOT NULL,
                  name            TEXT NOT NULL,
                  rules           BLOB NOT NULL,
                  FOREIGN KEY(user_id) REFERENCES user(id)
             )",
            [],
        )?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn init_test(&self) -> Result<()> {
        // Creates a test user
        self.add_user("test", "test@example.com", "test")?;

        // Create a test device
        let user = self.get_user("test")?;
        let device_id = Uuid::new_v4();
        self.add_device(user.id, device_id, "test")?;

        // Create a ruleset
        let rules: Vec<Rule> = vec![
            Rule {
                name: vec!["Work".to_string()],
                regex: ".*".to_string(),
            },
            Rule {
                name: vec!["Media".to_string()],
                regex: ".*".to_string(),
            },
        ];
        let ruleset_id = self.create_ruleset(user.id, "Ruleset example", rules)?;

        // Create some test activity
        // Whole hour
        let now = Utc::now()
            .with_minute(0)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();
        let events = vec![
            Event {
                timestamp: now,
                duration: Duration::from_secs(60),
                category: "Work".to_string(),
            },
            Event {
                timestamp: now.add(chrono::Duration::seconds(60)),
                duration: Duration::from_secs(60),
                category: "Media".to_string(),
            },
        ];
        self.report_activity(&device_id, ruleset_id, now, events)?;
        Ok(())
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

    pub fn add_device(&self, user_id: i64, device_id: Uuid, name: &str) -> Result<()> {
        self.conn()?.execute(
            "INSERT INTO device (id, user_id, name) VALUES (?1, ?2, ?3)",
            params![device_id.to_string(), user_id, name],
        )?;
        Ok(())
    }

    pub fn get_devices(&self, user_id: i64) -> Result<Vec<Device>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare("SELECT id, user_id, name FROM device WHERE user_id = ?1")?;
        let mut device_iter = stmt.query_map(params![user_id], |row| {
            let device_id: String = row.get(0)?;
            let device_id_uuid = Uuid::parse_str(&device_id).unwrap();
            let activities = self.get_activity_by_device(&device_id_uuid).unwrap();
            Ok(Device {
                id: device_id_uuid,
                user_id: row.get(1)?,
                name: row.get(2)?,
                last_seen: activities.iter().map(|a| a.timestamp).max(),
            })
        })?;

        let mut devices = Vec::new();
        while let Some(device) = device_iter.next() {
            devices.push(device?);
        }
        Ok(devices)
    }

    pub fn get_activity_by_device(&self, device_id: &Uuid) -> Result<Vec<Activity>> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "SELECT id, timestamp, device_id, events, ruleset_id FROM activity WHERE device_id = ?1",
        )?;
        let mut activity_iter = stmt.query_map(params![device_id.to_string()], |row| {
            let device_id_str: String = row.get(2)?;
            let events: String = row.get(3)?;
            let ruleset_id: i64 = row.get(4)?;
            Ok(Activity {
                id: row.get(0)?,
                timestamp: Utc.timestamp_opt(row.get(1)?, 0).unwrap(),
                device_id: Uuid::parse_str(&device_id_str).unwrap(),
                events: serde_json::from_str(&events).unwrap(),
                ruleset_id,
            })
        })?;

        let mut activities = Vec::new();
        while let Some(activity) = activity_iter.next() {
            activities.push(activity?);
        }
        Ok(activities)
    }

    pub fn create_ruleset(&self, user_id: i64, name: &str, rules: Vec<Rule>) -> Result<i64> {
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "INSERT INTO ruleset (user_id, name, rules) VALUES (?1, ?2, ?3)",
        )?;
        let rules_json = serde_json::to_string(&rules).unwrap();
        stmt.execute(params![user_id, name, rules_json])?;
        Ok(conn.last_insert_rowid() as i64)
    }

    pub fn report_activity(&self, device_id: &Uuid, ruleset_id: i64, hour: DateTime<Utc>, events: Vec<Event>) -> Result<()> {
        // Check that hour is exactly on the hour
        if hour.minute() != 0 || hour.second() != 0 {
            return Err(DatastoreError::BadRequest("Hour must be on the hour".to_string()));
        }
        let conn = self.conn()?;
        let mut stmt = conn.prepare(
            "INSERT INTO activity (timestamp, device_id, events, ruleset_id) VALUES (?1, ?2, ?3, ?4)",
        )?;
        let events_json = serde_json::to_string(&events).unwrap();
        stmt.execute(params![hour.timestamp(), device_id.to_string(), events_json, ruleset_id])?;
        Ok(())
    }

    // Add more functions to handle claims and other stuff...
}
