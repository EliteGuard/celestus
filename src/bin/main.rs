use anyhow::Result;
use celestus::{
    cache::{settings::HashMapValueTypes, Cache},
    providers::secrets::{SETTING_SECRETS_PROVIDERS, SETTING_USE_SECRETS_PROVIDER},
    utils::environment::init_environment,
};
use log::info;

// #[tokio::main(flavor = "multi_thread", worker_threads = 1)]
// #[tokio::main(flavor = "current_thread")]
fn main() -> Result<()> {
    env_logger::init();

    init_environment();
    let mut cache = Cache::new();

    info!(
        "{:?}",
        cache.settings.get_bool(SETTING_USE_SECRETS_PROVIDER)
    );

    let providers = cache
        .settings
        .get_hashmap(SETTING_SECRETS_PROVIDERS)
        .unwrap();
    match providers {
        HashMapValueTypes::SecretsProviders(sp) => {
            info!(
                "{:#?}",
                sp.get_providers().get(&"vault".into()).unwrap().get_name()
            );
        }
    }

    Ok(())
}
