use crate::utils::environment::get_env_var;
use anyhow::{Ok, Result};
use lru::LruCache;

use super::consts::{ENV_VAR_OVERRIDE_VAULT, SETTING_OVERRIDE_VAULT};

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
    let _ = import_settings(
        bools,
        vec![(
            SETTING_OVERRIDE_VAULT,
            get_env_var(ENV_VAR_OVERRIDE_VAULT, Some(false))?,
        )],
    );

    let _ = import_settings(ints, vec![("qwe", 123)]);

    Ok(())
}

fn import_settings<'a, Value>(
    lru: &mut LruSettingsCache<'a, Value>,
    settings: Vec<(&'a str, Value)>,
) -> Result<()>
where
    Value: std::fmt::Debug + Copy,
{
    for setting in settings.iter() {
        lru.push(setting.0, setting.1);
    }
    Ok(())
}
