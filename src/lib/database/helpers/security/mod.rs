use log::warn;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Debug;

use super::{
    json::{insert_obj_prop, upsert_obj_prop},
    HasConfig, HasName,
};

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

pub fn is_secure<Data>(candidate: &Data, _secure: &Data, exceptions: &Vec<Data>) -> bool
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
        .find(|&exc| exc.get_name() == candidate.get_name());

    if exception.is_some() {
        let except = exception.unwrap();
        if except.get_name() == candidate.get_name() {
            let exception_level = except.get_config().as_ref().unwrap().get("level").unwrap();
            return candidate_level.unwrap().as_u64().unwrap() == exception_level.as_u64().unwrap();
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

pub fn fix_unsecure_data<Data>(candidates: &mut Vec<Data>, _secure: &Data, exceptions: &Vec<Data>)
where
    for<'a> Data: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
{
    candidates.iter_mut().for_each(|candidate| {
        set_level_ok(candidate, exceptions);
    });
}

pub fn set_level_ok<Data>(candidate: &mut Data, exceptions: &Vec<Data>)
where
    for<'a> Data: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
{
    let default_level = json!({"level": 0});
    fix_config(candidate, default_level);

    if candidate
        .get_config()
        .as_ref()
        .unwrap()
        .get("level")
        .is_none()
    {
        insert_obj_prop(
            candidate.get_config_mut().as_mut().unwrap(),
            &"level".to_string(),
            json!(0),
        );
    }

    let candidate_config = candidate.get_config().as_ref();
    let candidate_level = candidate_config
        .unwrap()
        .get("level")
        .unwrap()
        .as_u64()
        .unwrap();

    let exception = exceptions
        .iter()
        .find(|&exc| exc.get_name() == candidate.get_name());

    if exception.is_some() {
        let except = exception.unwrap();
        if except.get_name() == candidate.get_name() {
            let exception_level = except
                .get_config()
                .as_ref()
                .unwrap()
                .get("level")
                .unwrap()
                .as_u64()
                .unwrap();
            if candidate_level != exception_level {
                protect_exception(candidate);
                return;
            }
        }
    }

    let max_allowed_level = get_max_allowed_level(exceptions);
    let cfg_to_override = candidate.get_config_mut().as_mut().unwrap();
    if candidate_level >= max_allowed_level {
        upsert_obj_prop(cfg_to_override, &"level".to_string(), json!(0), true);
    }
}

fn fix_config<Data>(candidate: &mut Data, default_value: serde_json::Value)
where
    for<'a> Data: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
{
    if candidate.get_config().is_none() {
        candidate.set_config(&default_value);
    }
}

fn protect_exception<Data>(candidate: &mut Data)
where
    for<'a> Data: Debug + HasName + HasConfig + Serialize + Deserialize<'a>,
{
    candidate.set_name(&String::from("TEST"));
    let candidate_config = candidate.get_config_mut().as_mut().unwrap();

    upsert_obj_prop(candidate_config, &"level".to_string(), json!(0), true);
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
