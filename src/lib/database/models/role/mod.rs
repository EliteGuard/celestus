use std::time::SystemTime;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::models::role_group::RoleGroup;
use crate::database::schema::roles;

#[derive(
    Identifiable,
    Associations,
    Queryable,
    AsChangeset,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    PartialEq,
)]
#[diesel(belongs_to(RoleGroup))]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub config: serde_json::Value,
    pub role_group_id: Option<Uuid>,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    pub deleted_at: Option<SystemTime>,
    pub hidden_at: Option<SystemTime>,
}
