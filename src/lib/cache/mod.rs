use self::settings::SettingsCache;

pub mod settings;

pub struct Cache<'a> {
    settings: SettingsCache<'a>
}

impl<'a> Cache<'a> {
    pub fn new() -> Self {
        let settings:SettingsCache<'a> = SettingsCache::new();
        Self { settings }
    }

    pub fn get_settings(&mut self) -> &mut SettingsCache<'a> {
        &mut self.settings
    }
}