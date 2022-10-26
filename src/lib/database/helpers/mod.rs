pub mod security;

use super::errors::{DatabaseError, SeedDatabaseError};
use anyhow::Result;
use log::{error, info, warn};
use security::is_data_secure;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::{BufReader, Write};
use std::{cell::RefCell, rc::Rc};
use std::{fmt::Debug, fs::OpenOptions};

pub trait HasName {
    fn get_name(&self) -> String;
}

pub trait HasConfig {
    fn get_config<'a>(&'a self) -> &'a Option<serde_json::Value>;
}

pub trait Predefined<'a, Model>
where
    Model: HasName + HasConfig + Serialize + Deserialize<'a>,
{
    fn get_predefined() -> Vec<Model>;
    fn get_exceptionals() -> Vec<Model>;
}

pub fn seed_file_check<Seed, SeedConfig>(
    path: &String,
    predefined: &Vec<Seed>,
    exceptionals: &Vec<Seed>,
) -> Result<(), SeedDatabaseError>
where
    for<'a> Seed: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
    for<'a> SeedConfig: Debug + Serialize + Deserialize<'a>,
{
    let seeds_from_file = get_seeds_from_file::<Seed>(path);
    let seeds = Rc::new(RefCell::new(seeds_from_file));

    info!(
        "Found {}/{} seeds in {}",
        seeds.as_ref().borrow().len(),
        predefined.len(),
        path
    );

    let secure = match is_data_secure::<Seed, SeedConfig>(
        predefined,
        &seeds.as_ref().borrow(),
        exceptionals,
    ) {
        Ok(result) => result,
        Err(err) => {
            error!("{}", err);
            return Err(SeedDatabaseError::SeedCorruptionAttempt);
        }
    };

    if seeds.as_ref().borrow().len() < predefined.len() || secure == false {
        info!("Overwriting {}", path);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .expect(format!("Failed to create the file {}", &path).as_str());
        let text = serde_json::to_string(&predefined).unwrap();
        writeln!(file, "{}", text).expect(&format!("Failed to write to file {}", &path));
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
