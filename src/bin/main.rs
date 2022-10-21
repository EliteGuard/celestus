use std::env;

use anyhow::Result;
use celestus::database::Database;
use celestus::utils::environment::Environment;
use dotenvy::dotenv;

fn main() -> Result<()> {
    dotenv().ok();

    let env = env::var("HOST_ENVIRONMENT").unwrap();
    let environment: Environment = match env.as_str() {
        "dev" => Environment::Dev,
        "prod" => Environment::Prod,
        _ => panic!("Unknown environment! Environment variable HOST_ENVIRONMENT must be set!"),
    };

    println!("Running on {}", env);

    let mut db = Database::new(environment);
    db.connect_and_init()?;

    Ok(())
}
