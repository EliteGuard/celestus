use super::{GetAll, HasConfig, HasName, Predefined};
use crate::database::errors::SeedDatabaseError;
use crate::database::helpers::security::{is_data_secure, set_data_secure};
use anyhow::Result;
use diesel::PgConnection;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::BufReader;
use std::vec;
use std::{fmt::Debug, fs::OpenOptions};

pub struct SeedProps {
    pub model: SeedModels,
    pub name: String,
    pub file_path: String,
    pub minimum_required: usize,
}

pub enum SeedModels {
    SystemConfig,
    RoleGroup,
}

pub trait Seedable<Model, Seed>
where
    for<'a> Model: GetAll<Model> + Debug + Ord + Serialize + HasName + HasConfig + Deserialize<'a>,
    for<'a> Seed:
        Predefined<Seed> + Serialize + Debug + Ord + HasName + HasConfig + Deserialize<'a>,
{
    fn is_seed_needed(
        connection: &mut PgConnection,
        minimum_required: usize,
    ) -> Result<bool, SeedDatabaseError> {
        let any_rows: Vec<Model> = match Model::get_all(connection) {
            Ok(rows) => rows,
            Err(err) => {
                error!("{}", err);
                return Err(SeedDatabaseError::SeedInfoGetFailed);
            }
        };

        Ok(any_rows.len() < minimum_required)
    }

    fn try_to_seed(
        connection: &mut PgConnection,
        seed_props: &SeedProps,
    ) -> Result<(), SeedDatabaseError> {
        info!("Seeding {}...", seed_props.name);

        let predefined = Seed::get_predefined();
        let exceptions = Seed::get_exceptions();

        let seed_needed = match Self::is_seed_needed(connection, seed_props.minimum_required) {
            Ok(res) => res,
            Err(err) => {
                error!("{}", err);
                return Err(SeedDatabaseError::SeedCheckFailed);
            }
        };
        if !seed_needed {
            return Ok(());
        }

        match Self::seed_file_check(&seed_props.file_path, &predefined, &exceptions) {
            Ok(()) => (),
            Err(err) => {
                error!("{}", err);
                return Err(SeedDatabaseError::SeedFileNotFound);
            }
        }

        info!("Successfully seeded {}!", seed_props.name);

        Ok(())
    }

    fn seed_file_check(
        path: &String,
        predefined: &Vec<Seed>,
        exceptions: &Vec<Seed>,
    ) -> Result<(), SeedDatabaseError> {
        let mut seeds = Self::get_seeds_from_file(path).unwrap_or(vec![]);

        info!("Found {} seeds in {}", seeds.len(), path);

        if seeds.len() >= predefined.len() {
            let filter = false;
            let secure = is_data_secure::<Seed>(&mut seeds, exceptions);

            if secure {
                return Ok(());
            }

            if !secure {
                warn!("The file {} is not secure!", path);
                warn!("Disarming...",);
                set_data_secure::<Seed>(&mut seeds, exceptions, filter);
                info!("Seeds left after disarm->\n{:#?}", seeds);
            }
        } else {
            warn!(
                "Found {} seeds while expecting at least {}.",
                seeds.len(),
                predefined.len()
            );
        }

        if seeds.len() >= predefined.len() {
            return Ok(());
        }

        warn!("Missing/corrupt file or seeds! Trying to recover");
        warn!("Overwriting {}!", path);

        create_dir_all("./data/seed").unwrap();

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .expect(format!("Failed to create the file {}", &path).as_str());
        match serde_json::to_writer_pretty(file, &serde_json::to_value(&predefined).unwrap()) {
            Ok(_) => (),
            Err(err) => {
                error!("{}", err);
                return Err(SeedDatabaseError::SeedRecoveryFailed);
            }
        };
        Ok(())
    }

    fn get_seeds_from_file(path: &String) -> Result<Vec<Seed>, SeedDatabaseError> {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(err) => {
                warn!("{}", err);
                warn!("Seed file {} was not found!", path);
                return Err(SeedDatabaseError::SeedFileNotFound);
            }
        };
        let reader = BufReader::new(file);
        let seeds = match serde_json::from_reader(reader).ok() {
            Some(res) => res,
            None => {
                info!("JSON array in {} is invalid!", path);
                vec![]
            }
        };
        Ok(seeds)
    }
}
