pub mod consts;

use core::result::Result::Ok;
use std::{collections::HashMap, num::NonZeroUsize};

use crate::{
    providers::secrets::{
        SecretsProviders, SETTING_SECRETS_PROVIDERS, SETTING_USE_SECRETS_PROVIDER,
    },
    utils::environment::{get_env_var, get_host_mode, SETTING_HOST_MODE},
};
use anyhow::Result;
use log::error;
use lru::LruCache;
use tracing::info;

use self::consts::{APP_SETTINGS, BOOL_SETTINGS, INT32_SETTINGS};

pub type LruSettingsCache<'a, Value> = LruCache<&'a str, Value>;

pub struct SettingsCache<'a> {
    bools: LruSettingsCache<'a, bool>,
    ints: LruSettingsCache<'a, i32>,
    strings: LruSettingsCache<'a, String>,
    hashmaps: HashMap<&'a str, HashMapValueTypes>,
}

#[derive(Debug)]
pub enum HashMapValueTypes {
    SecretsProviders(SecretsProviders),
}

pub enum SettingsTypes<'a> {
    Bool(&'a str, &'a str, Option<bool>),
    Int32(&'a str, &'a str, Option<i32>),
    String(&'a str, &'a str, Option<String>),
    Hashmap(&'a str, HashMapValueTypes),
}

impl<'a> SettingsCache<'a> {
    pub fn new() -> Self {
        let mut lru_bools: LruSettingsCache<bool> =
            LruCache::new(NonZeroUsize::new(BOOL_SETTINGS.len()).unwrap());

        let mut lru_ints: LruSettingsCache<i32> =
            LruCache::new(NonZeroUsize::new(INT32_SETTINGS.len()).unwrap());

        let mut lru_strings: LruSettingsCache<String> = LruCache::unbounded();
        lru_strings.push(SETTING_HOST_MODE, get_host_mode().to_string());

        let mut hashmaps = HashMap::<&str, HashMapValueTypes>::new();

        match load_settings(
            &mut lru_bools,
            &mut lru_ints,
            &mut lru_strings,
            &mut hashmaps,
        ) {
            Ok(_) => info!("All settings have bee loaded successfully!"),
            Err(err) => error!("{}", err),
        }

        Self {
            bools: lru_bools,
            ints: lru_ints,
            strings: lru_strings,
            hashmaps: hashmaps,
        }
    }

    pub fn get_bool(&mut self, key: &str) -> Option<&bool> {
        self.bools.get(key)
    }

    pub fn get_int(&mut self, key: &str) -> Option<&i32> {
        self.ints.get(key)
    }

    pub fn get_string(&mut self, key: &str) -> Option<&str> {
        self.strings.get(key).map(|x| x.as_str())
    }

    pub fn get_hashmap(&mut self, key: &'a str) -> Option<&HashMapValueTypes> {
        self.hashmaps.get(key)
    }

    // pub fn set_bool(&mut self, key: &'a str, val: bool) -> Option<bool> {
    //     self.bools.put(key, val)
    // }

    // pub fn set_int(&mut self, key: &'a str, val: i32) -> Option<i32> {
    //     self.ints.put(key, val)
    // }

    // pub fn set_string(&mut self, key: &'a str, val: String) -> Option<String> {
    //     self.strings.put(key, val)
    // }

    // pub fn set_hashmap(
    //     &mut self,
    //     key: &'a str,
    //     val: HashMapValueTypes,
    // ) -> Option<HashMapValueTypes> {
    //     self.hashmaps.insert(key, val)
    // }
}

fn load_settings<'a>(
    bools: &mut LruSettingsCache<'a, bool>,
    ints: &mut LruSettingsCache<'a, i32>,
    strings: &mut LruSettingsCache<'a, String>,
    hashmaps: &mut HashMap<&'a str, HashMapValueTypes>,
) -> Result<()> {
    for setting_type in APP_SETTINGS.iter() {
        for setting in setting_type.iter() {
            match setting {
                SettingsTypes::Bool(name, var, value) => {
                    bools.push(name, get_env_var(var, value.to_owned())?);
                }
                SettingsTypes::Int32(name, var, value) => {
                    ints.push(name, get_env_var(var, value.to_owned())?);
                }
                SettingsTypes::String(name, var, value) => {
                    strings.push(name, get_env_var(var, value.to_owned())?);
                }
                _ => (),
            }
        }
    }

    if let Some(use_providers) = bools.get(SETTING_USE_SECRETS_PROVIDER) {
        if *use_providers {
            hashmaps.insert(
                SETTING_SECRETS_PROVIDERS,
                HashMapValueTypes::SecretsProviders(SecretsProviders::new()),
            );
        }
    }

    Ok(())
}
