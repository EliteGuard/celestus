use std::time::SystemTime;

use anyhow::Result;
use diesel::prelude::*;
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::database::helpers::{
    security::{get_max_level, is_data_secure},
    seed_file_check, HasConfig, HasName, Predefined,
};
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

const ADMIN_ROLE_LEVEL: u32 = 100_000;
const CLIENT_ROLE_LEVEL: u32 = 10_000;
const USER_ROLE_LEVEL: u32 = 1_000;

#[derive(Serialize, Deserialize, Debug)]
pub struct RoleGroupConfig {
    level: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = role_groups)]
pub struct RoleGroupForm {
    name: String,
    config: Option<serde_json::Value>,
}

impl HasName for RoleGroup {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
impl HasConfig for RoleGroup {
    fn get_config<'a>(&'a self) -> &'a Option<serde_json::Value> {
        &self.config
    }
}

impl HasName for RoleGroupForm {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
impl HasConfig for RoleGroupForm {
    fn get_config<'a>(&'a self) -> &'a Option<serde_json::Value> {
        &self.config
    }
}

impl Predefined<'_, RoleGroupForm> for RoleGroupForm {
    fn get_predefined() -> Vec<RoleGroupForm> {
        vec![
            RoleGroupForm {
                name: SYSTEM_ROLE_GROUP_NAME.to_string(),
                config: Some(json!({ "level": get_max_level() })),
            },
            RoleGroupForm {
                name: ADMIN_ROLE_GROUP_NAME.to_string(),
                config: Some(json!({ "level": ADMIN_ROLE_LEVEL })),
            },
            RoleGroupForm {
                name: CLIENT_ROLE_GROUP_NAME.to_string(),
                config: Some(json!({ "level": CLIENT_ROLE_LEVEL })),
            },
            RoleGroupForm {
                name: USER_ROLE_GROUP_NAME.to_string(),
                config: Some(json!({ "level": USER_ROLE_LEVEL })),
            },
        ]
    }

    fn get_exceptionals() -> Vec<RoleGroupForm> {
        vec![
            RoleGroupForm {
                name: SYSTEM_ROLE_GROUP_NAME.to_string(),
                config: Some(json!({ "level": get_max_level() })),
            },
            RoleGroupForm {
                name: ADMIN_ROLE_GROUP_NAME.to_string(),
                config: Some(json!({ "level": ADMIN_ROLE_LEVEL })),
            },
        ]
    }
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

    // pub fn insert(
    //     connection: &mut PgConnection,
    //     role_groups: &Vec<RoleGroup>,
    // ) -> Result<Vec<RoleGroup>, DatabaseError> {
    //     let untouchables: Vec<RoleGroupForm> = RoleGroupForm::get_predefined();
    //     if let Err(err) =
    //         is_data_secure::<RoleGroupForm, RoleGroup, RoleGroupConfig>(&untouchables, role_groups)
    //     {
    //         error!("{}", err);
    //         return Err(DatabaseError::DataCorruptionAttempt);
    //     }

    //     match diesel::insert_into(role_groups::table)
    //         .values(role_groups)
    //         .get_results(connection)
    //     {
    //         Ok(results) => Ok(results),
    //         Err(err) => {
    //             error!("{}", err);
    //             return Err(DatabaseError::DataCreateFailed);
    //         }
    //     }
    // }

    // pub fn update(
    //     connection: &mut PgConnection,
    //     candidates: &Vec<RoleGroup>,
    // ) -> Result<Vec<RoleGroup>, DatabaseError> {
    //     Ok(vec![])
    // }

    pub fn try_to_seed(
        connection: &mut PgConnection,
        seed_file_path: &String,
    ) -> Result<(), SeedDatabaseError> {
        let any_rows = match RoleGroup::get_all_role_groups(connection) {
            Ok(rows) => rows,
            Err(err) => {
                error!("{}", err);
                return Err(SeedDatabaseError::SeedRoleGroupsFailed);
            }
        };

        let predefined = RoleGroupForm::get_predefined();
        let exceptionals = RoleGroupForm::get_exceptionals();

        if any_rows.len() == 0 {
            seed_file_check::<RoleGroupForm, RoleGroupConfig>(
                seed_file_path,
                &predefined,
                &exceptionals,
            );
            // RoleGroup::insert(connection, &any_rows);
        } else {
            //RoleGroup::update(connection, &any_rows);
        }

        Ok(())
    }
}
