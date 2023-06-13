use bcrypt::{hash, verify, DEFAULT_COST};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use std::env;

use crate::error::DatastoreError;

pub struct Db {
    pool: r2d2::Pool<SqliteConnectionManager>,
}

pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
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
        self.conn()?.execute(
            "CREATE TABLE IF NOT EXISTS user (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL UNIQUE,
                  email           TEXT NOT NULL UNIQUE,
                  password        TEXT NOT NULL
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

    pub fn add_user(&self, name: &str, email: &str, password: &str) -> Result<()> {
        let hashed_password = hash(password, DEFAULT_COST).unwrap();

        self.conn()?.execute(
            "INSERT INTO user (name, email, password) VALUES (?1, ?2, ?3)",
            params![name, email, hashed_password],
        )?;
        Ok(())
    }

    pub fn check_password(&self, name: &str, password: &str) -> Result<bool> {
        match self.get_user(name) {
            Ok(user) => Ok(verify(password, &user.password).unwrap()),
            Err(_) => Ok(false),
        }
    }

    pub fn get_user(&self, name: &str) -> Result<User> {
        let conn = self.conn()?;
        let mut stmt =
            conn.prepare("SELECT id, name, email, password FROM user WHERE name = ?1")?;
        let mut user_iter = stmt.query_map(params![name], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
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

    // Add more functions to handle claims and other stuff...
}
