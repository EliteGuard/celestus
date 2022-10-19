use std::time::SystemTime;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::schema::role_groups;

#[derive(Identifiable, Queryable, AsChangeset, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = role_groups)]
pub struct RoleGroup {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub config: serde_json::Value,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    pub deleted_at: Option<SystemTime>,
    pub hidden_at: Option<SystemTime>,
}
