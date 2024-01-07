use anyhow::Result;
use celestus::{utils::environment::init_environment, cache::{Cache, consts::SETTING_OVERRIDE_VAULT}};
use log::info;


fn main() -> Result<()> {
    env_logger::init();

    init_environment();
    let mut cache = Cache::new();


    info!("{:?}", cache.settings.get_bool(SETTING_OVERRIDE_VAULT));
    info!("{:?}", cache.settings.get_int("qwe"));

    Ok(())
}
