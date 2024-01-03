use std::time::SystemTime;

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::models::role::Role;
use crate::database::models::user_group::UserGroup;
use crate::database::schema::users;

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
#[diesel(belongs_to(UserGroup))]
#[diesel(belongs_to(Role))]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    pub phone: Option<String>,
    pub external_provider_config: Option<serde_json::Value>,
    pub config: Option<serde_json::Value>,
    pub user_group_id: Option<Uuid>,
    pub role_id: Option<Uuid>,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    pub deleted_at: Option<SystemTime>,
    pub hidden_at: Option<SystemTime>,
}
