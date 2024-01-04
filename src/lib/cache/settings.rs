use lru::LruCache;

pub type SettingsCache<'a> = LruCache<&'a str, &'a str>;

pub fn load_settings<'a>(cache: &mut SettingsCache) {
    import_settings(vec![("asd", "qwe")], cache);
}

fn import_settings<'a>(settings: Vec<(&'a str, &'a str)>, cache: &'a mut SettingsCache) {
    for setting in settings.iter() {
        // cache.push(setting.0, setting.1);
    }
}
