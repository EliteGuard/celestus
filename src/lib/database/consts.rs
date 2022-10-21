const C_SYSTEM_CONFIG_SEED_FILE_PATH: &str = "./data/seed/system_configs.json";
const C_SYSTEM_CONFIG_SEED_FILE_PATH_DEV: &str =
    "./src/lib/database/models/system_config/data/seed.json";

pub struct Consts {
    pub ENVIRONMENT: Environment,
    pub SYSTEM_CONFIG_SEED_FILE_PATH: String,
}

use crate::utils::environment::Environment;

impl Consts {
    pub fn new(environment: Environment) -> Self {
        let ENVIRONMENT = environment;
        let SYSTEM_CONFIG_SEED_FILE_PATH = match environment {
            Environment::Prod => C_SYSTEM_CONFIG_SEED_FILE_PATH.to_string(),
            Environment::Dev => C_SYSTEM_CONFIG_SEED_FILE_PATH_DEV.to_string(),
        };

        Self {
            ENVIRONMENT,
            SYSTEM_CONFIG_SEED_FILE_PATH,
        }
    }
}
