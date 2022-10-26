use super::errors::{DatabaseError, SeedDatabaseError};
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::{BufReader, Write};
use std::{fmt::Debug, fs::OpenOptions};

pub trait HasName {
    fn get_name(&self) -> String;
}

pub trait HasConfig {
    fn get_config<'a>(&'a self) -> &'a Option<serde_json::Value>;
}

pub trait Predefined<'a, Model>
where
    Model: Serialize + Deserialize<'a>,
{
    fn get_predefined() -> Vec<Model>;
}

pub fn is_data_secure<'a, Secure, Candidate, ConfigType>(
    consts: &Vec<Secure>,
    candidates: &Vec<Candidate>,
) -> Result<bool, DatabaseError>
where
    Secure: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
    Candidate: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
    ConfigType: Debug + Serialize + Deserialize<'a>,
{
    for secure in consts.iter() {
        let found = candidates
            .iter()
            .find(|&c| c.get_name() == secure.get_name());
        if let Some(rg) = found {
            info!("candidate->{:?}", rg);
            // let config = rg.get_config();
            // let candidate_config: ConfigType =
            //     serde_json::from_value(config.as_ref().unwrap()).unwrap();
        }
    }
    Ok(true)
}

pub fn get_secure_data<'a, Secure, Candidate, ConfigType>(
    consts: &Vec<Secure>,
    candidates: &Vec<Candidate>,
    exceptionals: &'a Vec<Secure>,
) -> Result<Vec<Candidate>, DatabaseError>
where
    Secure: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
    Candidate: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
    ConfigType: Debug + Serialize + Deserialize<'a>,
{
    let mut ok: Vec<Candidate> = vec![];
    for secure in consts.iter() {
        for candidate in candidates.iter() {

            if is_secure(secure, candidate, exceptionals)
            info!("candidate->{:?} is secure", candidate);
            warn!("candidate->{:?} is NOT secure", candidate);
        }
    }
    Ok(ok)
}

fn is_secure<'a, Secure, Candidate, ConfigType>(
    secure: &'a Secure,
    candidate: &'a Candidate,
    exceptionals: &'a Vec<Secure>,
) -> bool
where
    Secure: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
    Candidate: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
    ConfigType: Debug + Serialize + Deserialize<'a>,
{
    let no_config = json!("{}");

    if secure.get_name() != candidate.get_name() {
        return true;
    } else {
        let config = secure.get_config().as_ref().unwrap_or(&no_config);
        if config.get("level").is_none() {
            return true;
        } else {
            let exceptional = exceptionals
                .iter()
                .find(|&exc| exc.get_name() == candidate.get_name());
            if exceptional.is_some() {
                return false;
            } else {
                return true;
            }
        }
    }
}

pub fn seed_file_check<'a, Seed, SeedConfig>(
    path: &'a String,
    predefined: Vec<Seed>,
    exceptionals: &'a Vec<Seed>,
) -> Result<(), SeedDatabaseError>
where
    Seed: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
    SeedConfig: Debug + Serialize + Deserialize<'a>,
{
    let seeds = get_seeds_from_file(path);

    info!(
        "Found {}/{} seeds in {}",
        seeds.len(),
        predefined.len(),
        path
    );

    // let secure = match is_data_secure::<Seed, Seed, SeedConfig>(&predefined, &seeds) {
    //     Ok(result) => result,
    //     Err(err) => {
    //         error!("{}", err);
    //         return Err(SeedDatabaseError::SeedCorruptionAttempt);
    //     }
    // };

    if seeds.len() < predefined.len() || secure == false {
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

pub fn get_max_level() -> u32 {
    u32::MAX
}
