pub mod json;
pub mod security;
pub mod seeds;
use anyhow::Result;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

use super::errors::DatabaseError;

pub trait HasName {
    fn get_name(&self) -> &String;
    fn set_name<'a>(&'a mut self, name: &'a String);
}

pub trait HasConfig {
    fn get_config(&self) -> &Option<serde_json::Value>;
    fn get_config_mut<'a>(&'a mut self) -> &'a mut Option<serde_json::Value>;
    fn set_config<'a>(&'a mut self, config: &'a serde_json::Value);
}

pub trait Predefined<Model>
where
    for<'a> Model: HasName + HasConfig + Serialize + Deserialize<'a>,
{
    fn get_predefined() -> Vec<Model> {
        vec![]
    }

    fn get_exceptions() -> Vec<Model> {
        vec![]
    }
}

pub trait GetAll<Model> {
    fn get_all(connection: &mut PgConnection) -> Result<Vec<Model>, DatabaseError>;
}
