pub mod security;

use crate::database::helpers::security::set_data_secure;

use super::errors::SeedDatabaseError;
use anyhow::Result;
use log::{error, info};
use security::is_data_secure;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::vec;
use std::{fmt::Debug, fs::OpenOptions};

pub trait HasName {
    fn get_name(&self) -> &String;
    fn set_name<'a>(&'a mut self, name: &'a String);
}

pub trait HasConfig {
    fn get_config(&self) -> &Option<serde_json::Value>;
    fn get_config_mut<'a>(&'a mut self) -> &'a mut Option<serde_json::Value>;
    fn set_config<'a>(&'a mut self, config: &'a serde_json::Value);
}

pub trait Predefined<'a, Model>
where
    Model: HasName + HasConfig + Serialize + Deserialize<'a>,
{
    fn get_predefined() -> Vec<Model>;
    fn get_exceptions() -> Vec<Model>;
}

pub fn seed_file_check<Seed>(
    path: &String,
    predefined: &Vec<Seed>,
    exceptions: &Vec<Seed>,
) -> Result<(), SeedDatabaseError>
where
    for<'a> Seed: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
{
    let mut seeds = get_seeds_from_file::<Seed>(path);

    info!(
        "Found {}/{} seeds in {}",
        seeds.len(),
        predefined.len(),
        path
    );

    // set_data_secure::<Seed>(&mut seeds, predefined, exceptions, false);
    // println!("secure2->{:?}", seeds.len());

    let secure = is_data_secure::<Seed>(&seeds, predefined, exceptions);
    println!("secure->{:?}", secure);

    // set_data_secure::<Seed>(&mut seeds, predefined, exceptions, true);
    // println!("secure3->{:?}", seeds.len());

    if seeds.len() < predefined.len() {
        //} || !secure {
        info!("Overwriting {}", path);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .expect(format!("Failed to create the file {}", &path).as_str());
        match serde_json::to_writer(file, &serde_json::to_value(&predefined).unwrap()) {
            Ok(_) => (),
            Err(err) => {
                error!("{}", err);
                return Err(SeedDatabaseError::SeedRecoveryFailed);
            }
        };
        // let text = serde_json::to_string_pretty(&predefined).unwrap();
        // writeln!(file, "{}", text).expect(&format!("Failed to write to file {}", &path));
    }

    Ok(())
}

fn get_seeds_from_file<T: for<'a> Deserialize<'a>>(path: &String) -> Vec<T> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    match serde_json::from_reader(reader).ok() {
        Some(res) => res,
        None => {
            info!("JSON array in {} is invalid! Recovering...", path);
            vec![]
        }
    }
}
