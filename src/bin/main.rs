use std::env;

use anyhow::Result;
use celestus::database::Database;
use celestus::utils::environment::Environment;
use dotenvy::dotenv;

fn main() -> Result<()> {
    dotenv().expect("No .env file found!");

    let env = env::var("HOST_ENVIRONMENT").expect("Unknown environment! Environment variable HOST_ENVIRONMENT must be set!");
    let environment: Environment = match env.as_str() {
        "dev" => Environment::Dev,
        "prod" => Environment::Prod,
        val => panic!("Unknown value \"{}\" for environment variable HOST_ENVIRONMENT!", val),
    };

    println!("Running on {}", env);

    let mut db = Database::new(environment);
    db.connect_and_init()?;

    Ok(())
}
