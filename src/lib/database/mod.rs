mod consts;
mod errors;
pub mod models;
pub mod schema;

use anyhow::{Result, Error};
use chrono::{NaiveDateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::{env, fs, path::Path};
use log::{info, error};

use crate::{utils::environment::Environment};
use consts::Consts;
use errors::DatabaseError;

use models::system_config::{SystemConfigSeed};

use self::models::role_group::RoleGroup;

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
        self.connect()?;
        info!("Successfully connected to database!");
        self.seed()?;

        Ok(())
    }

    fn connect(&mut self) -> Result<&mut Self, DatabaseError> {
        if !self.connection.is_none() {
            return Ok(self);
        }

        let database_url = match self.generate_database_url()
         {
            Ok(res) => res,
            Err(_) => return Err(DatabaseError::URLGenerationFailed),
        };

        match PgConnection::establish(&database_url) {
            Ok(conn) => self.connection = Some(conn),
            Err(e) => {
                error!("Error connecting to the PG database!: {}", e.to_string());
                return Err(DatabaseError::ConnectFailed);
            }
        }

        Ok(self)
    }

    fn seed(&mut self) -> Result<&mut Self, DatabaseError> {
        //let now = Utc::now().naive_utc();

        self.seeded = self.is_seeded();


        // self.seed_system_configs(&now)?;

        self.seeded = true;
        Ok(self)
    }

    fn is_seeded(&mut self) -> bool {
        let mut conn = self.connection.as_ref().unwrap();
        RoleGroup::is_seeded(&mut conn);
        false
    }

    fn seed_system_configs(
        &mut self,
        date_time_now: &NaiveDateTime,
    ) -> Result<&mut Self, DatabaseError> {
        let file_path = Path::new(&self.consts.system_config_seed_file_path);
        let json_file_contents = fs::read_to_string(file_path).expect(&format!(
            "The file {} cannot be found or read!\n
            Make sure you are running in the intended environment as needed:\n
            1) Dev - from the project's root directory (out of /src)\n
            2) Others - from the home directory (where Celestus is installed)\n",
            &self.consts.system_config_seed_file_path
        ));

        let system_config_seeds =
            serde_json::from_str::<Vec<SystemConfigSeed>>(&json_file_contents).expect(&format!(
                "JSON array in {} is invalid!",
                self.consts.system_config_seed_file_path
            ));

        use schema::system_configs::dsl::*;
        if let Some(conn) = &mut self.connection {
            diesel::insert_into(system_configs)
                .values(&system_config_seeds)
                .execute(conn)
                .unwrap();
        } else {
            panic!("WROOONG!");
        };
        // let inserted_system_configs =

        Ok(self)
    }

    fn generate_database_url(&self) -> Result<String, Error> {
        let url_prefix = env::var("DATABASE_URL_PREFIX").expect("environment variable DATABASE_URL_PREFIX must be set");
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

        let full_url = format!(
            "{}://{}:{}@{}:{}/{}",
            url_prefix, user, password, host, port, name);
        Ok(full_url)
    }
}
