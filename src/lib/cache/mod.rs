pub mod settings;

use self::settings::SettingsCache;

pub struct Cache<'a> {
    pub settings: SettingsCache<'a>,
}

impl Cache<'_> {
    pub async fn new() -> Self {
        let settings: SettingsCache = SettingsCache::new().await;
        Self { settings }
    }
}
