use anyhow::Result;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt::Debug;

use super::{HasConfig, HasName};

pub fn is_data_secure<Data>(
    candidates: &Vec<Data>,
    consts: &Vec<Data>,
    exceptions: &Vec<Data>,
) -> bool
where
    for<'a> Data: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
{
    for secure in consts.iter() {
        for candidate in candidates.iter() {
            if !is_secure(candidate, secure, exceptions) {
                warn!("candidate->{:?} is NOT secure", candidate);
                return false;
            }
        }
    }
    true
}

pub fn is_secure<Data>(candidate: &Data, secure: &Data, exceptions: &Vec<Data>) -> bool
where
    for<'a> Data: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
{
    is_level_ok(candidate, exceptions)
}

fn is_level_ok<Data>(candidate: &Data, exceptions: &Vec<Data>) -> bool
where
    for<'a> Data: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
{
    let candidate_config = candidate.get_config().as_ref();
    if candidate_config.is_none() {
        return false;
    }

    let candidate_level = candidate_config.unwrap().get("level");
    if candidate_level.is_none() {
        return false;
    }

    if !candidate_level.unwrap().is_u64() {
        return false;
    }

    let exception = exceptions
        .iter()
        .find(|&exc| *exc.get_name() == *candidate.get_name());

    if exception.is_some() {
        let except = exception.unwrap();
        if *except.get_name() == *candidate.get_name() {
            let exception_level = except.get_config().as_ref().unwrap().get("level").unwrap();
            if candidate_level.unwrap().as_u64().unwrap() != exception_level.as_u64().unwrap() {
                return false;
            }
        }
    }

    let max_allowed_level = get_max_allowed_level(exceptions);
    if candidate_level.unwrap().as_u64().unwrap() >= max_allowed_level {
        return false;
    }

    return true;
}

pub fn set_data_secure<Data>(
    candidates: &mut Vec<Data>,
    consts: &Vec<Data>,
    exceptions: &Vec<Data>,
    filter: bool,
) where
    for<'a> Data: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
{
    for secure in consts.iter() {
        if filter {
            filter_secure_data(candidates, secure, exceptions);
        } else {
            fix_unsecure_data(candidates, secure, exceptions);
        }
    }
}

pub fn filter_secure_data<Data>(candidates: &mut Vec<Data>, secure: &Data, exceptions: &Vec<Data>)
where
    for<'a> Data: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
{
    candidates.retain(|candidate| is_secure(candidate, secure, exceptions));
}

pub fn fix_unsecure_data<Data>(candidates: &mut Vec<Data>, secure: &Data, exceptions: &Vec<Data>)
where
    for<'a> Data: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
{
    candidates.iter_mut().for_each(|candidate| {
        // let exceptional = exceptions
        //     .iter()
        //     .find(|&exc| exc.get_name() == candidate.get_name());
        // let is_exception = exceptional.is_some();

        // let mut no_config = json!("{}");
        // let candidate_config = candidate
        //     .get_config_mut()
        //     .as_mut()
        //     .unwrap_or(&mut no_config);
        // set_level_ok(candidate_config, secure, is_exception, max_allowed_level);
        set_level_ok(candidate, exceptions);
    });
}

pub fn set_level_ok<Data>(candidate: &mut Data, exceptions: &Vec<Data>)
where
    for<'a> Data: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
{
    let mut candidate_config = candidate.get_config().as_mut();

    if candidate_config.is_none() {
        candidate_config = Some(&mut json!({"level": 0}));
        return;
    }

    let candidate_level = candidate_config.unwrap().get("level");
    if candidate_level.is_none() {
        candidate_config.unwrap()["level"] = json!(0);
        return;
    }

    if !candidate_level.unwrap().is_u64() {
        candidate_config.unwrap()["level"] = json!(0);
        return;
    }

    let exception = exceptions
        .iter()
        .find(|&exc| *exc.get_name() == *candidate.get_name());

    if exception.is_some() {
        let except = exception.unwrap();
        if *except.get_name() == *candidate.get_name() {
            let exception_level = except.get_config().as_ref().unwrap().get("level").unwrap();
            if candidate_level.unwrap().as_u64().unwrap() != exception_level.as_u64().unwrap() {
                candidate.set_name(String::from("TEST"));
                candidate_config.unwrap()["level"] = json!(0);
                return;
            }
        }
    }

    let max_allowed_level = get_max_allowed_level(exceptions);
    if candidate_level.unwrap().as_u64().unwrap() >= max_allowed_level {
        candidate_config.unwrap()["level"] = json!(0);
    }
    // let candidate_level = candidate_config.get("level");
    // if candidate_level.is_none() {
    //     candidate_config["level"] = json!(0);
    //     return;
    // } else {
    //     if !candidate_level.unwrap().is_u64() {
    //         candidate_config["level"] = json!(0);
    //         return;
    //     }

    //     let secure_level = secure.get_config().as_ref().unwrap().get("level").unwrap();

    //     if is_exception {
    //         if candidate_level.unwrap().as_u64().unwrap() != secure_level.as_u64().unwrap() {
    //             candidate_config["level"] = json!(0);
    //             return;
    //         }
    //     } else {
    //         if candidate_level.unwrap().as_u64().unwrap() >= max_allowed_level {
    //             candidate_config["level"] = json!(0);
    //             return;
    //         }
    //     }
    // }
}

pub fn get_max_level() -> u32 {
    u32::MAX
}

fn get_max_allowed_level<Data>(group: &Vec<Data>) -> u64
where
    for<'a> Data: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
{
    group.iter().fold(u64::MAX, |min_val, member| {
        let val = member
            .get_config()
            .as_ref()
            .unwrap()
            .get("level")
            .unwrap()
            .as_u64()
            .unwrap();
        val.min(min_val)
    })
}
