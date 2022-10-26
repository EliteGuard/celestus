use log::warn;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt::Debug;

use crate::database::errors::DatabaseError;

use super::{HasConfig, HasName};

pub fn is_data_secure<Seed, ConfigType>(
    candidates: &Vec<Seed>,
    consts: &Vec<Seed>,
    exceptionals: &Vec<Seed>,
) -> Result<bool, DatabaseError>
where
    for<'a> Seed: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
    for<'a> ConfigType: Debug + Serialize + Deserialize<'a>,
{
    for secure in consts.iter() {
        for candidate in candidates.iter() {
            if !is_secure::<Seed, ConfigType>(candidate, secure, exceptionals) {
                warn!("candidate->{:?} is NOT secure", candidate);
                return Ok(false);
            }
        }
    }
    Ok(true)
}

pub fn is_secure<Seed, ConfigType>(
    candidate: &Seed,
    secure: &Seed,
    exceptionals: &Vec<Seed>,
) -> bool
where
    for<'a> Seed: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
    for<'a> ConfigType: Debug + Serialize + Deserialize<'a>,
{
    let no_config = json!("{}");

    if secure.get_name() != candidate.get_name() {
        return true;
    } else {
        let exceptional = exceptionals
            .iter()
            .find(|&exc| exc.get_name() == candidate.get_name());
        let is_exception = exceptional.is_some();
        let max_allowed_level = exceptionals.iter().fold(u64::MAX, |min_val, exc| {
            let val = exc
                .get_config()
                .as_ref()
                .unwrap()
                .get("level")
                .unwrap()
                .as_u64()
                .unwrap();
            val.min(min_val)
        });

        let cfg = candidate.get_config().as_ref().unwrap_or(&no_config);
        is_level_ok(&cfg, secure, is_exception, max_allowed_level.into()).unwrap()
    }
}

fn is_level_ok<Secure>(
    candidate_config: &Value,
    secure: &Secure,
    is_exception: bool,
    max_allowed_level: u64,
) -> Result<bool, DatabaseError>
where
    for<'a> Secure: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
{
    let candidate_level = candidate_config.get("level");
    if candidate_level.is_none() {
        return Ok(false);
    } else {
        if !candidate_level.unwrap().is_u64() {
            return Ok(false);
        }

        let secure_level = secure.get_config().as_ref().unwrap().get("level").unwrap();

        warn!(
            "candidate_level.unwrap().as_u64().unwrap()->{}",
            candidate_level.unwrap().as_u64().unwrap()
        );

        if is_exception {
            if candidate_level.unwrap().as_u64().unwrap() != secure_level.as_u64().unwrap() {
                return Err(DatabaseError::DataCorruptionAttempt);
            }
        } else {
            if candidate_level.unwrap().as_u64().unwrap() >= max_allowed_level {
                return Err(DatabaseError::DataCorruptionAttempt);
            }
        }

        return Ok(true);
    }
}

// pub fn get_secure_data<Seed, ConfigType>(
//     candidates: &mut Vec<Seed>,
//     consts: &Vec<Seed>,
//     exceptionals: &Vec<Seed>,
//     stomp: bool,
// ) -> Result<Vec<Seed>, DatabaseError>
// where
//     for<'a> Seed: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
//     for<'a> ConfigType: Debug + Serialize + Deserialize<'a>,
// {
//     let mut secured: Vec<Seed> = vec![];
//     for secure in consts.iter() {
//         for candidate in candidates.iter() {
//             if is_secure::<Seed, ConfigType>(secure, candidate, exceptionals) {
//                 // secured.push(*candidate);
//                 info!("candidate->{:?} is secure", candidate);
//             } else {
//                 warn!("candidate->{:?} is NOT secure", candidate);
//             }
//         }
//     }
//     Ok(secured)
// }

// pub fn set_level_ok<Secure>(
//     candidate_config: &mut Value,
//     exceptionals: &Vec<Secure>,
//     stomp: bool,
// ) -> Result<(), DatabaseError>
// where
//     for<'a> Secure: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
// {
//     let candidate_level = candidate_config.get("level");
//     if candidate_level.is_none() {
//         candidate_config["level"] = json!(0);
//         return Ok(());
//     } else {
//         if !candidate_level.unwrap().is_u64() {
//             candidate_config["level"] = json!(0);
//             return Ok(());
//         }

//         let any_bigger_candidate_level = exceptionals.iter().any(|exc| {
//             let exc_config = exc.get_config().as_ref().unwrap();
//             let exc_level = exc_config.get("level").unwrap();
//             candidate_level.unwrap().as_u64().unwrap() >= exc_level.as_u64().unwrap()
//         });
//         if any_bigger_candidate_level {
//             if stomp {
//                 candidate_config["level"] = json!(0);
//             } else {
//                 return Err(DatabaseError::DataCorruptionAttempt);
//             }
//         }
//         Ok(())
//     }
// }

pub fn get_max_level() -> u32 {
    u32::MAX
}
