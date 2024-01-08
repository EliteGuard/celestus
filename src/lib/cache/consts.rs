pub const SETTING_USE_SECRETS_PROVIDER: &str = "use_secrets_provider";
const ENV_USE_USE_SECRETS_PROVIDER: &str = "USE_SECRETS_PROVIDER";

pub enum SettingsTypes<'a> {
    Bool(&'a str, &'a str, Option<bool>),
    Int32(&'a str, &'a str, Option<i32>),
}

pub type AppSettings = &'static [&'static [&'static SettingsTypes<'static>]];

pub const APP_SETTINGS: AppSettings = &[BOOL_SETTINGS, INT32_SETTINGS];

pub const BOOL_SETTINGS: &'static [&'static SettingsTypes] = &[&SettingsTypes::Bool(
    SETTING_USE_SECRETS_PROVIDER,
    ENV_USE_USE_SECRETS_PROVIDER,
    Some(false),
)];

pub const INT32_SETTINGS: &'static [&'static SettingsTypes] =
    &[&SettingsTypes::Int32("some_int", "some_int", Some(123))];
