use anyhow::Result;
use celestus::{utils::environment::init_environment, cache::settings::{SettingsCache, load_settings}};
use lru::LruCache;


fn main() -> Result<()> {
    env_logger::init();

    init_environment();
    let mut settings_cache: SettingsCache = LruCache::unbounded();
    load_settings(&mut settings_cache);

    Ok(())
}
