pub mod security;

use crate::database::helpers::security::filter_out_unsecure_data;

use super::errors::SeedDatabaseError;
use anyhow::Result;
use log::{error, info};
use security::is_data_secure;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write};
use std::{cell::RefCell, rc::Rc, vec};
use std::{fmt::Debug, fs::OpenOptions};

pub trait HasName {
    fn get_name(&self) -> String;
}

pub trait HasConfig {
    fn get_config<'a>(&'a self) -> &'a Option<serde_json::Value>;
    fn get_config_mut<'a>(&'a mut self) -> &'a mut Option<serde_json::Value>;
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

    let secure2 = match filter_out_unsecure_data::<Seed, SeedConfig>(
        &mut seeds.borrow_mut(),
        predefined,
        exceptionals,
        false,
    ) {
        Ok(result) => result,
        Err(err) => {
            error!("{}", err);
            return Err(SeedDatabaseError::SeedCorruptionAttempt);
        }
    };
    println!("secure2->{:?}", secure2);

    let secure = match is_data_secure::<Seed, SeedConfig>(
        &seeds.as_ref().borrow(),
        predefined,
        exceptionals,
    ) {
        Ok(result) => result,
        Err(err) => {
            error!("{}", err);
            return Err(SeedDatabaseError::SeedCorruptionAttempt);
        }
    };
    println!("secure->{:?}", secure);

    // let secure3 = match filter_out_unsecure_data::<Seed, SeedConfig>(
    //     &mut seeds.borrow_mut(),
    //     predefined,
    //     exceptionals,
    //     true,
    // ) {
    //     Ok(result) => result,
    //     Err(err) => {
    //         error!("{}", err);
    //         return Err(SeedDatabaseError::SeedCorruptionAttempt);
    //     }
    // };
    // println!("secure3->{:?}", secure3);

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
