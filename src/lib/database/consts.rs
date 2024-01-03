const C_SYSTEM_CONFIG_SEED_FILE_PATH: &str = "./data/seed/system_configs.json";
const C_SYSTEM_CONFIG_SEED_FILE_PATH_DEV: &str =
    "./src/lib/database/models/system_config/data/seed.json";

const C_ROLE_GROUP_SEED_FILE_PATH: &str = "./data/seed/role_groups.json";
const C_ROLE_GROUP_SEED_FILE_PATH_DEV: &str = "./src/lib/database/models/role_group/data/seed.json";

pub struct Consts {
    pub environment: Environment,
    pub seed_consts: Vec<SeedProps>,
}

use crate::utils::environment::Environment;

use super::helpers::seeds::{SeedModels, SeedProps};

impl Consts {
    pub fn new(environment: &Environment) -> Self {
        let environment = *environment;
        let system_config_seed_file_path = match environment {
            Environment::Prod => C_SYSTEM_CONFIG_SEED_FILE_PATH.to_string(),
            Environment::Dev => C_SYSTEM_CONFIG_SEED_FILE_PATH_DEV.to_string(),
        };
        let role_group_seed_file_path = match environment {
            Environment::Prod => C_ROLE_GROUP_SEED_FILE_PATH.to_string(),
            Environment::Dev => C_ROLE_GROUP_SEED_FILE_PATH_DEV.to_string(),
        };

        Self {
            environment,
            seed_consts: vec![
                SeedProps {
                    model: SeedModels::SystemConfig,
                    name: "system_configs".to_string(),
                    file_path: system_config_seed_file_path,
                    minimum_required: 1,
                },
                SeedProps {
                    model: SeedModels::RoleGroup,
                    name: "role_groups".to_string(),
                    file_path: role_group_seed_file_path,
                    minimum_required: 4,
                },
            ],
        }
    }
}
