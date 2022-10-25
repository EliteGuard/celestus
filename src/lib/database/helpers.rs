use super::errors::DatabaseError;
use log::info;
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

pub fn is_data_secure<Secure, Candidate, ConfigType>(
    consts: &Vec<Secure>,
    candidates: &Vec<Candidate>,
) -> Result<bool, DatabaseError>
where
    Secure: Debug + HasName + HasConfig,
    Candidate: Debug + HasName + HasConfig,
    ConfigType: Serialize + for<'a> Deserialize<'a>,
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

pub fn seed_file_check<T: Serialize + for<'a> Deserialize<'a>>(path: &String, predefined: Vec<T>) {
    let file_path = Path::new(&path);
    let json_file_contents = match fs::read_to_string(file_path).ok() {
        Some(contents) => contents,
        None => {
            info!("The file {} is not found! Will try to create it...", path);
            "".to_owned()
        }
    };

    let seeds = match serde_json::from_str::<Vec<T>>(&json_file_contents).ok() {
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

    if seeds.len() < predefined.len() {
        info!("Overwriting {}", path);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .expect(format!("Failed to create the file {}", &path).as_str());
        let text = serde_json::to_string(&predefined).unwrap();
        writeln!(file, "{}", text).expect(&format!("Failed to write to file {}", &path));
    }
}
