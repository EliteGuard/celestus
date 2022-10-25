use super::errors::{DatabaseError, SeedDatabaseError};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{
    fmt::Debug,
    fs::{self, OpenOptions},
    path::Path,
};

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

pub fn seed_file_check<'a, PredefinedSeed, CandidateSeed, SeedConfig>(
    path: &String,
    predefined: Vec<PredefinedSeed>,
) -> Result<(), SeedDatabaseError>
where
    PredefinedSeed: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
    CandidateSeed: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
    SeedConfig: Debug + Serialize + Deserialize<'a>,
{
    let file_path = Path::new(&path);
    let json_file_contents = match fs::read_to_string(file_path).ok() {
        Some(contents) => contents,
        None => {
            info!("The file {} is not found! Will try to create it...", path);
            "".to_owned()
        }
    };

    let seeds = match serde_json::from_str::<Vec<CandidateSeed>>(&json_file_contents).ok() {
        Some(res) => res,
        None => {
            info!("JSON array in {} is invalid! Recovering...", path);
            vec![]
        }
    };

    info!(
        "Found {}/{} seeds in {}",
        seeds.len(),
        predefined.len(),
        path
    );

    let secure =
        match is_data_secure::<PredefinedSeed, CandidateSeed, SeedConfig>(&predefined, &seeds) {
            Ok(result) => result,
            Err(err) => {
                error!("{}", err);
                return Err(SeedDatabaseError::SeedCorruptionAttempt);
            }
        };

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

pub fn get_max_level() -> u32 {
    u32::MAX
}
