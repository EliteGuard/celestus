use log::info;
use lru::LruCache;

pub type SettingsCache<'a> = LruCache<&'a str, &'a str>;

pub fn load_settings<'a>(cache: &mut SettingsCache) {
    import_settings(vec![("asd", "qwe")], cache);
}

fn import_settings<'a>(settings: Vec<(&'a str, &'a str)>, cache: &mut SettingsCache<'a>) {
    for setting in settings.iter() {
        cache.push(setting.to_owned().0, setting.to_owned().1);
    }
    info!("{:?}", cache.get(settings[0].0))
}
