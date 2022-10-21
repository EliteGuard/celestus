use ::thiserror::Error;
use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub struct Database {
    populated: bool,
    ready: bool,
    connection: Option<PgConnection>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            populated: false,
            ready: false,
            connection: None,
        }
    }

    pub fn connect_and_init(&mut self) -> Result<bool, DatabaseError> {
        if !self.connect() {
            return Err(DatabaseError::ConnectFailed);
        }

        if !self.populate() {
            return Err(DatabaseError::PopulateFailed);
        }

        Ok(true)
    }

    fn connect(&mut self) -> bool {
        dotenv().ok();

        let url_prefix = env::var("DATABASE_URL_PREFIX").expect("DATABASE_URL_PREFIX must be set");
        let user = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
        let password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
        let host = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
        let port = env::var("DATABASE_PORT").expect("DATABASE_PORT must be set");
        let name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

        let database_url = format!(
            "{}://{}:{}@{}:{}/{}",
            url_prefix, user, password, host, port, name
        );
        // postgres://postgres:Tatkomil0@localhost:5433/celestus

        match PgConnection::establish(&database_url) {
            Ok(conn) => self.connection = Some(conn),
            Err(e) => println!("Error connecting to the PG database!: {}", e.to_string()),
        }

        !self.connection.is_none()
    }

    fn populate(&mut self) -> bool {
        false
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Failed to connect to database!")]
    ConnectFailed,
    #[error("Failed to populate data!")]
    PopulateFailed,
    #[error("Failed to recover from an unsuccessful operation!")]
    RecoveryFailed,
}
