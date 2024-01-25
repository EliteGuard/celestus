pub mod settings;

use self::settings::SettingsCache;

pub struct Cache<'a> {
    pub settings: SettingsCache<'a>,
}

impl Default for Cache<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Cache<'_> {
    pub fn new() -> Self {
        let settings: SettingsCache = SettingsCache::new();
        Self { settings }
    }
}
