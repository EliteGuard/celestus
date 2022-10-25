const C_SYSTEM_CONFIG_SEED_FILE_PATH: &str = "./data/seed/system_configs.json";
const C_SYSTEM_CONFIG_SEED_FILE_PATH_DEV: &str =
    "./src/lib/database/models/system_config/data/seed.json";

const C_ROLE_GROUP_SEED_FILE_PATH: &str = "./data/seed/role_groups.json";
const C_ROLE_GROUP_SEED_FILE_PATH_DEV: &str = "./src/lib/database/models/role_group/data/seed.json";

pub struct Consts {
    pub environment: Environment,
    pub system_config_seed_file_path: String,
    pub role_group_seed_file_path: String,
}

use crate::utils::environment::Environment;

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
            system_config_seed_file_path,
            role_group_seed_file_path,
        }
    }
}
