use std::{cmp::Ordering, ops::Deref, time::SystemTime};

use diesel::prelude::*;
use log::error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::{
    errors::DatabaseError,
    helpers::{seeds::Seedable, GetAll, HasConfig, HasName, Predefined},
    schema::system_configs,
};

#[derive(Identifiable, Insertable, Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = system_configs)]
pub struct SystemConfig {
    pub id: Uuid,
    pub name: String,
    pub config: Option<serde_json::Value>,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    pub deleted_at: Option<SystemTime>,
    pub hidden_at: Option<SystemTime>,
}

impl HasName for SystemConfig {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn set_name<'a>(&'a mut self, name: &'a String) {
        self.name = name.clone()
    }
}
impl HasConfig for SystemConfig {
    fn get_config<'a>(&'a self) -> &'a Option<serde_json::Value> {
        &self.config
    }

    fn get_config_mut<'a>(&'a mut self) -> &'a mut Option<serde_json::Value> {
        &mut self.config
    }

    fn set_config<'a>(&'a mut self, config: &'a serde_json::Value) {
        self.config = Some(config.clone());
    }
}
impl Ord for SystemConfig {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.get_name(),).cmp(&(other.get_name(),))
    }
}
impl PartialOrd for SystemConfig {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for SystemConfig {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
    }
}
impl Eq for SystemConfig {}
impl Seedable<SystemConfig, SystemConfigInput> for SystemConfig {}
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = system_configs)]
pub struct SystemConfigInput {
    name: String,
    config: Option<serde_json::Value>,
}

impl HasName for SystemConfigInput {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn set_name<'a>(&'a mut self, name: &'a String) {
        self.name = name.clone()
    }
}
impl HasConfig for SystemConfigInput {
    fn get_config<'a>(&'a self) -> &'a Option<serde_json::Value> {
        &self.config
    }

    fn get_config_mut<'a>(&'a mut self) -> &'a mut Option<serde_json::Value> {
        &mut self.config
    }

    fn set_config<'a>(&'a mut self, config: &'a serde_json::Value) {
        self.config = Some(config.clone());
    }
}
impl Deref for SystemConfigInput {
    type Target = Option<serde_json::Value>;
    fn deref(&self) -> &Option<serde_json::Value> {
        &self.config
    }
}
impl Ord for SystemConfigInput {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.get_name(),).cmp(&(other.get_name(),))
    }
}
impl PartialOrd for SystemConfigInput {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for SystemConfigInput {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
    }
}
impl Eq for SystemConfigInput {}
impl Seedable<SystemConfig, SystemConfigInput> for SystemConfigInput {}
impl Predefined<SystemConfigInput> for SystemConfigInput {
    fn get_predefined() -> Vec<SystemConfigInput> {
        vec![]
    }

    fn get_exceptions() -> Vec<SystemConfigInput> {
        vec![]
    }
}
impl GetAll<SystemConfig> for SystemConfig {
    fn get_all(connection: &mut PgConnection) -> Result<Vec<SystemConfig>, DatabaseError> {
        let seeded_system_configs = match system_configs::table.load::<SystemConfig>(connection) {
            Ok(res) => res,
            Err(err) => {
                error!("{}", err);
                return Err(DatabaseError::DataSelectFailed);
            }
        };

        Ok(seeded_system_configs)
    }
}
