use self::settings::SettingsCache;

pub mod settings;

pub struct Cache<'a> {
    pub settings: SettingsCache<'a>,
}

impl Cache<'_> {
    pub fn new() -> Self {
        let settings: SettingsCache = SettingsCache::new();
        Self { settings }
    }
}
