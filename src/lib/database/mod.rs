mod consts;
mod errors;
pub mod models;
pub mod schema;

use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::{insert_into, pg::PgConnection};
use std::{env, fs, path::Path};

use crate::utils::environment::Environment;
use consts::Consts;
use errors::DatabaseError;

// use models::system_config::{SystemConfig, SystemConfigSeed};

pub struct Database {
    seeded: bool,
    ready: bool,
    connection: Option<PgConnection>,
    consts: Consts,
    environment: Environment,
}

impl Database {
    pub fn new(environment: Environment) -> Self {
        // dotenv().ok();

        let consts = Consts::new(environment);

        Self {
            seeded: false,
            ready: false,
            connection: None,
            consts,
            environment,
        }
    }

    pub fn connect_and_init(&mut self) -> Result<(), DatabaseError> {
        self.connect()?.seed()?;

        Ok(())
    }

    fn connect(&mut self) -> Result<&mut Self, DatabaseError> {
        if !self.connection.is_none() {
            return Ok(self);
        }

        let url_prefix = env::var("DATABASE_URL_PREFIX")
            .expect("environment variable DATABASE_URL_PREFIX must be set");
        let user =
            env::var("DATABASE_USER").expect("environment variable DATABASE_USER must be set");
        let password = env::var("DATABASE_PASSWORD")
            .expect("environment variable DATABASE_PASSWORD must be set");
        let host =
            env::var("DATABASE_HOST").expect("environment variable DATABASE_HOST must be set");
        let port =
            env::var("DATABASE_PORT").expect("environment variable DATABASE_PORT must be set");
        let name =
            env::var("DATABASE_NAME").expect("environment variable DATABASE_NAME must be set");

        let database_url = format!(
            "{}://{}:{}@{}:{}/{}",
            url_prefix, user, password, host, port, name
        );

        match PgConnection::establish(&database_url) {
            Ok(conn) => self.connection = Some(conn),
            Err(e) => {
                println!("Error connecting to the PG database!: {}", e.to_string());
                return Err(DatabaseError::ConnectFailed);
            }
        }

        Ok(self)
    }

    fn seed(&mut self) -> Result<&mut Self, DatabaseError> {
        let now = Utc::now().naive_utc();

        self.seed_system_configs(&now)?;

        self.seeded = true;
        Ok(self)
    }

    fn seed_system_configs(
        &mut self,
        date_time_now: &NaiveDateTime,
    ) -> Result<&mut Self, DatabaseError> {
        let file_path = Path::new(&self.consts.SYSTEM_CONFIG_SEED_FILE_PATH);
        let json_file_contents = fs::read_to_string(file_path).expect(&format!(
            "The file {} cannot be found or read!\n
            Make sure you are running in the intended environment as needed:\n
            1) Dev - from the project's root directory (out of /src)\n
            2) Others - from the home directory (where Celestus is installed)\n",
            &self.consts.SYSTEM_CONFIG_SEED_FILE_PATH
        ));

        println!("{}", json_file_contents);

        // let system_config_seeds =
        //     serde_json::from_str::<Vec<SystemConfigSeed>>(&json_file_contents).expect(&format!(
        //         "JSON in {} is invalid!",
        //         MODEL_SYSTEM_CONFIG_SEED_FILE_PATH
        //     ));

        use schema::system_configs::dsl::*;

        // let inserted_system_configs = insert_into(system_configs)
        //     .values(&system_config_seeds)
        //     .get_results(self.connection)?;

        Ok(self)
    }
}
