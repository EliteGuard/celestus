use std::time::SystemTime;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::schema::system_configs;

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = system_configs)]
pub struct SystemConfig {
    pub id: Uuid,
    pub name: String,
    pub config: serde_json::Value,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    pub deleted_at: Option<SystemTime>,
    pub hidden_at: Option<SystemTime>,
}

// #[derive(Insertable, Deserialize)]
// #[diesel(table_name = system_configs)]
// pub struct SystemConfigSeed<'a> {
//     name: &'a str,
//     config: &'a serde_json::Value,
// }
