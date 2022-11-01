use std::env;

use anyhow::Result;
use celestus::database::Database;
use celestus::utils::environment::Environment;
use log::info;

fn main() -> Result<()> {
    env_logger::init();

    dotenvy::from_path(".env").expect("No .env file found!");

    let env = env::var("HOST_ENVIRONMENT")
        .expect("Unknown environment! Environment variable HOST_ENVIRONMENT must be set!");
    let environment: Environment = match env.as_str() {
        "dev" => Environment::Dev,
        "prod" => Environment::Prod,
        val => panic!(
            "Unknown value \"{}\" for environment variable HOST_ENVIRONMENT!",
            val
        ),
    };

    info!("Running on {}", env);

    let mut db = Database::new(&environment);
    db.connect_and_init()?;

    Ok(())
}
