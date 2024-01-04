use anyhow::Result;
use celestus::utils::environment::{ init_environment};


fn main() -> Result<()> {
    env_logger::init();

    init_environment();

    Ok(())
}
