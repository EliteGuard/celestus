use ::thiserror::Error;
use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

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

    pub fn prepare(&mut self) -> Result<bool, DatabaseError> {
        if !self.connect() {
            return Err(DatabaseError::ConnectFailed);
        }

        Ok(true)
    }

    fn connect(&mut self) -> bool {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        match PgConnection::establish(&database_url) {
            Ok(conn) => self.connection = Some(conn),
            Err(e) => println!("Error connecting to the PG database!: {}", e.to_string()),
        }

        !self.connection.is_none()
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Failed to connect to database!")]
    ConnectFailed,
    #[error("Failed to insert populate data!")]
    PopulateFailed,
    #[error("Failed to recover from an unsuccessful operation!")]
    RecoveryFailed,
}
