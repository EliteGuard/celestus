use anyhow::Result;
use celestus::{
    cache::{consts::SETTING_OVERRIDE_VAULT, Cache},
    utils::environment::init_environment,
};
use log::info;

fn main() -> Result<()> {
    env_logger::init();

    init_environment();
    let mut cache = Cache::new();

    info!("{:?}", cache.settings.get_bool(SETTING_OVERRIDE_VAULT));
    info!("{:?}", cache.settings.get_int("some_int"));

    Ok(())
}
