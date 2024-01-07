use crate::utils::environment::get_env_var;
use anyhow::{Ok, Result};
use lru::LruCache;

use super::consts::{SettingsTypes, APP_SETTINGS};

pub type LruSettingsCache<'a, Value> = LruCache<&'a str, Value>;

pub struct SettingsCache<'a> {
    bools: LruSettingsCache<'a, bool>,
    ints: LruSettingsCache<'a, i32>,
}

impl<'a> SettingsCache<'a> {
    pub fn new() -> Self {
        let mut lru_bools: LruSettingsCache<bool> = LruCache::unbounded();
        let mut lru_ints: LruSettingsCache<i32> = LruCache::unbounded();

        let _ = load_settings(&mut lru_bools, &mut lru_ints);

        Self {
            bools: lru_bools,
            ints: lru_ints,
        }
    }

    pub fn get_bool(&mut self, key: &str) -> Option<&bool> {
        self.bools.get(key)
    }

    pub fn get_int(&mut self, key: &str) -> Option<&i32> {
        self.ints.get(key)
    }
}

fn load_settings<'a>(
    bools: &mut LruSettingsCache<'a, bool>,
    ints: &mut LruSettingsCache<'a, i32>,
) -> Result<()> {
    for setting in APP_SETTINGS.iter() {
        match setting {
            SettingsTypes::Bool(name, var, value) => {
                bools.push(name, get_env_var(var, *value)?);
            }
            SettingsTypes::Int32(name, var, value) => {
                ints.push(name, get_env_var(var, *value)?);
            }
        }
    }

    Ok(())
}
