pub mod consts;

use core::result::Result::Ok;
use std::{collections::HashMap};

use crate::{
    providers::{secrets::{SecretsProviders, SETTING_SECRETS_PROVIDERS, SETTING_USE_SECRETS_PROVIDER, SecretsProviderData, SecretsProviderImplementation}, DataProvider},
    utils::environment::{get_env_var, get_host_mode, SETTING_HOST_MODE},
};
use anyhow::Result;
use log::error;
use lru::LruCache;
use tracing::info;

use self::consts::{APP_SETTINGS};

pub type LruSettingsCache<'a, Value> = LruCache<&'a str, Value>;

pub struct SettingsCache<'a> {
    bools: LruSettingsCache<'a, bool>,
    ints: LruSettingsCache<'a, i32>,
    strings: LruSettingsCache<'a, String>,
    hashmaps: HashMap<&'a str, HashMapValueTypes>,
}

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
        let lru_bools: LruSettingsCache<bool> = LruCache::unbounded();

        let lru_ints: LruSettingsCache<i32> = LruCache::unbounded();

        let mut lru_strings: LruSettingsCache<String> = LruCache::unbounded();
        lru_strings.push(SETTING_HOST_MODE, get_host_mode().to_string());

        let hashmaps = HashMap::<&str, HashMapValueTypes>::new();

        let mut created = Self {
            bools: lru_bools,
            ints: lru_ints,
            strings: lru_strings,
            hashmaps: hashmaps,
        };

        match created.load_env_var_settings() {
            Ok(_) => (),
            Err(err) => error!("{}", err),
        }

        match created.load_structured_settings() {
            Ok(_) => (),
            Err(err) => error!("{}", err),
        }

        info!("All settings have bee loaded successfully!");

        return created;
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

    pub fn get_hashmap(&self, key: &'a str) -> Option<&HashMapValueTypes> {
        self.hashmaps.get(key)
    }

    pub fn get_secrets_provider(&self, key: &'a str) -> Option<&DataProvider<SecretsProviderData, SecretsProviderImplementation>> {
        
        match self.get_hashmap(SETTING_SECRETS_PROVIDERS).unwrap() {
            HashMapValueTypes::SecretsProviders(sp) => sp.providers.get(key),
            _ => None
        }

    }

    fn load_env_var_settings(&mut self) -> Result<()> {
        for setting_type in APP_SETTINGS.iter() {
            for setting in setting_type.iter() {
                match setting {
                    SettingsTypes::Bool(name, var, value) => {
                        self.bools.push(name, get_env_var(var, value.to_owned())?);
                    }
                    SettingsTypes::Int32(name, var, value) => {
                        self.ints.push(name, get_env_var(var, value.to_owned())?);
                    }
                    SettingsTypes::String(name, var, value) => {
                        self.strings.push(name, get_env_var(var, value.to_owned())?);
                    }
                    _ => (),
                }
            }
        }
    
        Ok(())
    }

    fn load_structured_settings(&mut self) -> Result<()> {
        self.load_hashmaps()?;

        Ok(())
    }

    fn load_hashmaps(&mut self) -> Result<()> {
        
        self.load_data_providers()?;

        self.fetch_from_data_providers()?;

        Ok(())
    }

    fn load_data_providers(&mut self) -> Result<()> {

        self.load_secrets_providers()?;

        Ok(())
    }

    fn load_secrets_providers(&mut self) -> Result<()> {

        if let Some(use_providers) = self.bools.get(SETTING_USE_SECRETS_PROVIDER) {
            if *use_providers {
                self.hashmaps.insert(
                    SETTING_SECRETS_PROVIDERS,
                    HashMapValueTypes::SecretsProviders(SecretsProviders::new()),
                );
            }
        }

        Ok(())
    }

    pub fn fetch_from_data_providers(&mut self) -> Result<()> {

        self.fetch_from_secrets_providers()?;

        Ok(())
    }

    pub fn fetch_from_secrets_providers(&mut self) -> Result<()> {
            
        // for secret_provider in self.get_hashmap(SETTING_SECRETS_PROVIDERS)

        Ok(())
    }
}
