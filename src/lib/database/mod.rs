mod connection;
mod consts;
mod errors;
mod helpers;
pub mod models;
pub mod schema;

use anyhow::{Context, Error, Result};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection, R2D2Connection};
use log::{error, info};
use std::env;

use crate::database::helpers::seeds::try_to_seed;
use crate::database::models::role_group::RoleGroupInput;
use crate::utils::environment::Environment;
use consts::Consts;
use errors::DatabaseError;

use self::models::role_group::RoleGroup;

pub struct Database {
    seeded: bool,
    ready: bool,
    pool: Option<Pool<ConnectionManager<PgConnection>>>,
    connection: Option<PooledConnection<ConnectionManager<diesel::PgConnection>>>,
    consts: Consts,
}

impl Database {
    pub fn new(environment: &Environment) -> Self {
        let consts = Consts::new(environment);

        Self {
            seeded: false,
            ready: false,
            pool: None,
            connection: None,
            consts,
        }
    }

    pub fn connect_and_init(&mut self) -> Result<(), DatabaseError> {
        self.connect()?;
        self.seed()?;

        self.ready = true;
        info!("Database is ready for use!");

        Ok(())
    }

    fn connect(&mut self) -> Result<&mut Self, DatabaseError> {
        if self.pool.as_ref().is_some() {
            return Ok(self);
        }

        let database_url = match self.generate_database_url() {
            Ok(res) => res,
            Err(err) => {
                error!("{}", err);
                return Err(DatabaseError::URLGenerationFailed);
            }
        };

        match Pool::builder().build(ConnectionManager::<PgConnection>::new(database_url)) {
            Ok(pool) => self.pool = Some(pool),
            Err(err) => {
                error!("{}", err);
                return Err(DatabaseError::PoolCreationFailed);
            }
        }

        match self.pool.as_ref().unwrap().get() {
            Ok(connection) => self.connection = Some(connection),
            Err(err) => {
                error!("Error connecting to the PG database!: {}", err);
                return Err(DatabaseError::ConnectFailed);
            }
        };

        match self.connection.as_mut().unwrap().ping() {
            Ok(_) => info!("Successfully connected to database!"),
            Err(err) => {
                error!("Error connecting to the PG database!: {}", err);
                return Err(DatabaseError::ConnectFailed);
            }
        }

        Ok(self)
    }

    fn seed(&mut self) -> Result<&mut Self, DatabaseError> {
        info!("Starting seeding database...");
        let conn = self.connection.as_mut().unwrap();
        match try_to_seed::<RoleGroup, RoleGroupInput>(
            conn,
            &self.consts.role_group_seed_file_path,
            &"role_groups".to_string(),
        ) {
            Ok(_) => (),
            Err(err) => {
                error!("{}", err);
                return Err(DatabaseError::SeedFailed);
            }
        };

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

    // fn seed_system_configs(
    //     &mut self,
    //     date_time_now: &NaiveDateTime,
    // ) -> Result<&mut Self, DatabaseError> {
    //     let file_path = Path::new(&self.consts.system_config_seed_file_path);
    //     let json_file_contents = fs::read_to_string(file_path).expect(&format!(
    //         "The file {} cannot be found or read!\n
    //         Make sure you are running in the intended environment as needed:\n
    //         1) Dev - from the project's root directory (out of /src)\n
    //         2) Others - from the home directory (where Celestus is installed)\n",
    //         &self.consts.system_config_seed_file_path
    //     ));

    //     let system_config_seeds =
    //         serde_json::from_str::<Vec<SystemConfigSeed>>(&json_file_contents).expect(&format!(
    //             "JSON array in {} is invalid!",
    //             self.consts.system_config_seed_file_path
    //         ));

    //     use schema::system_configs::dsl::*;
    //     let mut conn = self.pool.as_ref().unwrap().get().unwrap();
    //     diesel::insert_into(system_configs)
    //         .values(&system_config_seeds)
    //         .execute(&mut conn)
    //         .unwrap();
    //     // let inserted_system_configs =

    //     Ok(self)
    // }

    fn generate_database_url(&self) -> Result<String, Error> {
        let url_prefix = env::var("DATABASE_URL_PREFIX")
            .map_err(anyhow::Error::msg)
            .context("environment variable DATABASE_URL_PREFIX must be set")?;
        let user = env::var("DATABASE_USER")
            .map_err(anyhow::Error::msg)
            .context("environment variable DATABASE_USER must be set")?;
        let password = env::var("DATABASE_PASSWORD")
            .map_err(anyhow::Error::msg)
            .context("environment variable DATABASE_PASSWORD must be set")?;
        let host = env::var("DATABASE_HOST")
            .map_err(anyhow::Error::msg)
            .context("environment variable DATABASE_HOST must be set")?;
        let port = env::var("DATABASE_PORT")
            .map_err(anyhow::Error::msg)
            .context("environment variable DATABASE_PORT must be set")?;
        let name = env::var("DATABASE_NAME")
            .map_err(anyhow::Error::msg)
            .context("environment variable DATABASE_NAME must be set")?;

        let full_url = format!(
            "{}://{}:{}@{}:{}/{}",
            url_prefix, user, password, host, port, name
        );
        Ok(full_url)
    }
}
