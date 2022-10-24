use std::time::SystemTime;

use anyhow::Result;
use diesel::pg::sql_types::Jsonb;
use diesel::{prelude::*, AsExpression, FromSqlRow};
use log::{error, info};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::{
    errors::{DatabaseError, SeedDatabaseError},
    schema::role_groups,
};

#[derive(Identifiable, Insertable, Queryable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = role_groups)]
pub struct RoleGroup {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub config: Option<serde_json::Value>,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
    pub deleted_at: Option<SystemTime>,
    pub hidden_at: Option<SystemTime>,
}

const SYSTEM_ROLE_GROUP_NAME: &str = "SYSTEM";
const ADMIN_ROLE_GROUP_NAME: &str = "ADMIN";
const CLIENT_ROLE_GROUP_NAME: &str = "CLIENT";
const USER_ROLE_GROUP_NAME: &str = "USER";

#[derive(Serialize, Deserialize, Debug)]
pub struct RoleGroupConfig {
    level: Option<i32>,
}
impl RoleGroup {
    pub fn get_all_role_groups(
        connection: &mut PgConnection,
    ) -> Result<Vec<RoleGroup>, DatabaseError> {
        let seeded_role_groups = match role_groups::table.load::<RoleGroup>(connection) {
            Ok(res) => res,
            Err(err) => {
                error!("{}", err);
                return Err(DatabaseError::DataSelectFailed);
            }
        };

        Ok(seeded_role_groups)
    }

    pub fn insert(
        connection: &mut PgConnection,
        role_groups: &Vec<RoleGroup>,
    ) -> Result<Vec<RoleGroup>, DatabaseError> {
        match diesel::insert_into(role_groups::table)
            .values(role_groups)
            .get_results(connection)
        {
            Ok(results) => Ok(results),
            Err(err) => {
                error!("{}", err);
                return Err(DatabaseError::DataCreateFailed);
            }
        }
    }

    pub fn update(
        connection: &mut PgConnection,
        candidates: &Vec<RoleGroup>,
    ) -> Result<Vec<RoleGroup>, DatabaseError> {
        Ok(vec![])
    }

    pub fn try_to_seed(connection: &mut PgConnection) -> Result<(), SeedDatabaseError> {
        let any_rows = match RoleGroup::get_all_role_groups(connection) {
            Ok(rows) => rows,
            Err(err) => {
                error!("{}", err);
                return Err(SeedDatabaseError::SeedRoleGroupsFailed);
            }
        };

        if any_rows.len() == 0 {
            RoleGroup::insert(connection, &any_rows);
        } else {
            RoleGroup::update(connection, &any_rows);
        }

        Ok(())
    }

    pub fn is_data_secure(candidates: Vec<RoleGroup>) -> Result<bool, DatabaseError> {
        let untouchables: Vec<&str> = vec![
            SYSTEM_ROLE_GROUP_NAME,
            ADMIN_ROLE_GROUP_NAME,
            CLIENT_ROLE_GROUP_NAME,
            USER_ROLE_GROUP_NAME,
        ];

        for untouchable in untouchables.iter() {
            let found = candidates.iter().find(|&c| &&c.name[..] == untouchable);
            if let Some(rg) = found {
                info!("{:?}", rg);
                // let config: RoleGroupConfig = serde_json::from_value(rg.config.unwrap()).unwrap();
            }
        }
        Ok(true)
    }
}
