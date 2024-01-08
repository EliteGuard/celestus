use anyhow::Result;
use celestus::{
    cache::Cache,
    utils::environment::init_environment, providers::secrets::SETTING_USE_SECRETS_PROVIDER,
};
use log::info;

fn main() -> Result<()> {
    env_logger::init();

    init_environment();
    let mut cache = Cache::new();

    info!("{:?}", cache.settings.get_bool(SETTING_USE_SECRETS_PROVIDER));


    
    info!("{:?}", cache.settings.get_int("some_int"));
    cache.settings.set_int("some_int", &456);
    info!("{:?}", cache.settings.get_int("some_int"));

    Ok(())
}
