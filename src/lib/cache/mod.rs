use self::settings::SettingsCache;

pub mod settings;
pub mod consts;

pub struct Cache<'a> {
    pub settings: SettingsCache<'a>
}

impl<'a> Cache<'a> {
    pub fn new() -> Self {
        let settings:SettingsCache<'a> = SettingsCache::new();
        Self { settings }
    }
}