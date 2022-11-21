use std::cmp::Ordering;
use std::ops::Deref;
use std::time::SystemTime;

use anyhow::Result;
use diesel::prelude::*;
use log::error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::database::helpers::GetAll;
use crate::database::helpers::{security::get_max_level, HasConfig, HasName, Predefined};
use crate::database::{errors::DatabaseError, schema::role_groups};

#[derive(Identifiable, Insertable, Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
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

impl HasName for RoleGroup {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn set_name(&mut self, name: &String) {
        self.name = name.clone()
    }
}
impl HasConfig for RoleGroup {
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

impl Ord for RoleGroup {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.get_name(),
            &self
                .get_config()
                .as_ref()
                .unwrap()
                .get("level")
                .unwrap()
                .as_u64()
                .unwrap(),
        )
            .cmp(&(
                other.get_name(),
                &other
                    .get_config()
                    .as_ref()
                    .unwrap()
                    .get("level")
                    .unwrap()
                    .as_u64()
                    .unwrap(),
            ))
    }
}

impl PartialOrd for RoleGroup {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RoleGroup {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
    }
}

impl Eq for RoleGroup {}

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
pub struct RoleGroupInput {
    name: String,
    config: Option<serde_json::Value>,
}

impl HasName for RoleGroupInput {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn set_name<'a>(&'a mut self, name: &'a String) {
        self.name = name.clone()
    }
}
impl HasConfig for RoleGroupInput {
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

impl Deref for RoleGroupInput {
    type Target = Option<serde_json::Value>;
    fn deref(&self) -> &Option<serde_json::Value> {
        &self.config
    }
}

impl Ord for RoleGroupInput {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            self.get_name(),
            &self
                .get_config()
                .as_ref()
                .unwrap()
                .get("level")
                .unwrap()
                .as_u64()
                .unwrap(),
        )
            .cmp(&(
                other.get_name(),
                &other
                    .get_config()
                    .as_ref()
                    .unwrap()
                    .get("level")
                    .unwrap()
                    .as_u64()
                    .unwrap(),
            ))
    }
}

impl PartialOrd for RoleGroupInput {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RoleGroupInput {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
    }
}

impl Eq for RoleGroupInput {}

impl Predefined<RoleGroupInput> for RoleGroupInput {
    fn get_predefined() -> Vec<RoleGroupInput> {
        vec![
            RoleGroupInput {
                name: SYSTEM_ROLE_GROUP_NAME.to_string(),
                config: Some(json!({ "level": get_max_level() })),
            },
            RoleGroupInput {
                name: ADMIN_ROLE_GROUP_NAME.to_string(),
                config: Some(json!({ "level": ADMIN_ROLE_LEVEL })),
            },
            RoleGroupInput {
                name: CLIENT_ROLE_GROUP_NAME.to_string(),
                config: Some(json!({ "level": CLIENT_ROLE_LEVEL })),
            },
            RoleGroupInput {
                name: USER_ROLE_GROUP_NAME.to_string(),
                config: Some(json!({ "level": USER_ROLE_LEVEL })),
            },
        ]
    }

    fn get_exceptions() -> Vec<RoleGroupInput> {
        vec![
            RoleGroupInput {
                name: SYSTEM_ROLE_GROUP_NAME.to_string(),
                config: Some(json!({ "level": get_max_level() })),
            },
            RoleGroupInput {
                name: ADMIN_ROLE_GROUP_NAME.to_string(),
                config: Some(json!({ "level": ADMIN_ROLE_LEVEL })),
            },
        ]
    }
}

impl GetAll<RoleGroup> for RoleGroup {
    fn get_all(connection: &mut PgConnection) -> Result<Vec<RoleGroup>, DatabaseError> {
        let seeded_role_groups = match role_groups::table.load::<RoleGroup>(connection) {
            Ok(res) => res,
            Err(err) => {
                error!("{}", err);
                return Err(DatabaseError::DataSelectFailed);
            }
        };

        Ok(seeded_role_groups)
    }
}

impl RoleGroup {
    // fn get_all(connection: &mut PgConnection) -> Result<Vec<RoleGroup>, DatabaseError> {
    //     let seeded_role_groups = match role_groups::table.load::<RoleGroup>(connection) {
    //         Ok(res) => res,
    //         Err(err) => {
    //             error!("{}", err);
    //             return Err(DatabaseError::DataSelectFailed);
    //         }
    //     };

    //     Ok(seeded_role_groups)
    // }
    // pub fn insert(
    //     connection: &mut PgConnection,
    //     role_groups: &Vec<RoleGroup>,
    // ) -> Result<Vec<RoleGroup>, DatabaseError> {
    //     let untouchables: Vec<RoleGroupInput> = RoleGroupInput::get_predefined();
    //     if let Err(err) =
    //         is_data_secure::<RoleGroupInput, RoleGroup, RoleGroupConfig>(&untouchables, role_groups)
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

    // pub fn try_to_seed(
    //     connection: &mut PgConnection,
    //     seed_file_path: &String,
    // ) -> Result<(), SeedDatabaseError> {
    //     info!("Seeding role_groups...");
    //     let any_rows = match RoleGroup::get_all(connection) {
    //         Ok(rows) => rows,
    //         Err(err) => {
    //             error!("{}", err);
    //             return Err(SeedDatabaseError::SeedRoleGroupsFailed);
    //         }
    //     };

    //     let predefined = RoleGroupInput::get_predefined();
    //     let exceptions = RoleGroupInput::get_exceptions();

    //     if any_rows.len() == 0 {
    //         match seed_file_check::<RoleGroupInput>(seed_file_path, &predefined, &exceptions) {
    //             Ok(()) => (),
    //             Err(err) => {
    //                 error!("{}", err);
    //                 return Err(SeedDatabaseError::SeedRoleGroupsFailed);
    //             }
    //         }
    //         // RoleGroup::insert(connection, &any_rows);
    //     } else {
    //         //RoleGroup::update(connection, &any_rows);
    //     }

    //     Ok(())
    // }
}
