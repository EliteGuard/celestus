mod consts;
mod errors;
mod helpers;
pub mod models;
pub mod schema;

use anyhow::{Error, Result};
use chrono::{NaiveDateTime};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use log::{error, info};
use std::{env, fs, path::Path};

use crate::utils::environment::Environment;
use consts::Consts;
use errors::DatabaseError;

use models::system_config::SystemConfigSeed;

use self::errors::SeedDatabaseError;
use self::models::role_group::RoleGroup;

pub struct Database {
    seeded: bool,
    ready: bool,
    pool: Option<Pool<ConnectionManager<PgConnection>>>,
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
            pool: None,
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
        if self.pool.is_some() {
            return Ok(self);
        }

        let database_url = match self.generate_database_url() {
            Ok(res) => res,
            Err(_) => return Err(DatabaseError::URLGenerationFailed),
        };

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        match Pool::builder().build(manager) {
            Ok(pool) => self.pool = Some(pool),
            Err(e) => {
                error!("Error connecting to the PG database!: {}", e.to_string());
                return Err(DatabaseError::ConnectFailed);
            }
        }

        Ok(self)
    }

    fn seed(&mut self) -> Result<&mut Self, DatabaseError> {
        
        let mut conn = self.pool.as_ref().unwrap().get().unwrap();
        RoleGroup::try_to_seed(&mut conn, &self.consts.role_group_seed_file_path);

        //let now = Utc::now().naive_utc();

        // self.seeded = match self.is_seeded() {
        //     Ok(result) => result,
        //     Err(err) => {
        //         error!("{}", err.to_string());
        //         return Err(DatabaseError::SeedFailed);
        //     }
        // };

        // self.seed_system_configs(&now)?;

        self.seeded = true;
        Ok(self)
    }

    // fn is_seeded(&mut self) -> Result<bool, SeedDatabaseError> {
    //     let mut conn = self.pool.as_ref().unwrap().get().unwrap();
    //     RoleGroup::try_to_seed(&mut conn)?;
    //     Ok(true)
    // }

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
        let mut conn = self.pool.as_ref().unwrap().get().unwrap();
        diesel::insert_into(system_configs)
            .values(&system_config_seeds)
            .execute(&mut conn)
            .unwrap();
        // let inserted_system_configs =

        Ok(self)
    }

    fn generate_database_url(&self) -> Result<String, Error> {
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

        let full_url = format!(
            "{}://{}:{}@{}:{}/{}",
            url_prefix, user, password, host, port, name
        );
        Ok(full_url)
    }
}
