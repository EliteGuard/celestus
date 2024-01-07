use anyhow::Result;
use celestus::{utils::environment::init_environment, cache::Cache};
use log::info;


fn main() -> Result<()> {
    env_logger::init();

    init_environment();
    let mut cache = Cache::new();


    info!("{:?}", cache.get_settings().get_bool("asd"));
    info!("{:?}", cache.get_settings().get_int("qwe"));

    Ok(())
}
