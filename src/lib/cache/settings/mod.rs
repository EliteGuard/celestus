pub mod consts;

use std::{collections::HashMap, num::NonZeroUsize};

use crate::{providers::secrets::SecretsProviders, utils::environment::get_env_var};
use anyhow::{Ok, Result};
use lru::LruCache;

use self::consts::{APP_SETTINGS, BOOL_SETTINGS, INT32_SETTINGS};

pub type LruSettingsCache<'a, Value> = LruCache<&'a str, Value>;

pub struct SettingsCache<'a> {
    bools: LruSettingsCache<'a, bool>,
    ints: LruSettingsCache<'a, i32>,
    hashmaps: HashMap<&'a str, HashMapValueTypes<'a>>,
}

pub enum HashMapValueTypes<'a> {
    SecretsProvider(SecretsProviders<'a>),
}

pub enum SettingsTypes<'a> {
    Bool(&'a str, &'a str, Option<bool>),
    Int32(&'a str, &'a str, Option<i32>),
}

impl<'a> SettingsCache<'a> {
    pub fn new() -> Self {
        let mut lru_bools: LruSettingsCache<bool> =
            LruCache::new(NonZeroUsize::new(BOOL_SETTINGS.len()).unwrap());
        let mut lru_ints: LruSettingsCache<i32> =
            LruCache::new(NonZeroUsize::new(INT32_SETTINGS.len()).unwrap());
        let mut hashmaps = HashMap::<&str, HashMapValueTypes>::new();

        let _ = load_settings(&mut lru_bools, &mut lru_ints);

        Self {
            bools: lru_bools,
            ints: lru_ints,
            hashmaps: hashmaps,
        }
    }

    pub fn get_bool(&mut self, key: &str) -> Option<&bool> {
        self.bools.get(key)
    }

    pub fn get_int(&mut self, key: &str) -> Option<&i32> {
        self.ints.get(key)
    }

    pub fn get_hashmap(&mut self, key: &'a str) -> Option<&HashMapValueTypes> {
        self.hashmaps.get(key)
    }

    pub fn set_bool(&mut self, key: &'a str, val: bool) -> Option<bool> {
        self.bools.put(key, val)
    }

    pub fn set_int(&mut self, key: &'a str, val: i32) -> Option<i32> {
        self.ints.put(key, val)
    }

    pub fn set_hashmap(
        &mut self,
        key: &'a str,
        val: HashMapValueTypes<'a>,
    ) -> Option<HashMapValueTypes> {
        self.hashmaps.insert(key, val)
    }
}

fn load_settings<'a>(
    bools: &mut LruSettingsCache<'a, bool>,
    ints: &mut LruSettingsCache<'a, i32>,
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
            }
        }
    }

    Ok(())
}
