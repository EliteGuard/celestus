pub const SETTING_OVERRIDE_VAULT: &str = "override_vault";
pub const ENV_VAR_OVERRIDE_VAULT: &str = "OVERRIDE_VAULT";

pub enum SettingsTypes<'a> {
    Bool(&'a str, &'a str, Option<bool>),
    Int32(&'a str, &'a str, Option<i32>),
}

pub type AppSettings = &'static [&'static [&'static SettingsTypes<'static>]];

pub const APP_SETTINGS: AppSettings = &[BOOL_SETTINGS, INT32_SETTINGS];

const BOOL_SETTINGS: &'static [&'static SettingsTypes] = &[&SettingsTypes::Bool(
    SETTING_OVERRIDE_VAULT,
    ENV_VAR_OVERRIDE_VAULT,
    Some(false),
)];

const INT32_SETTINGS: &'static [&'static SettingsTypes] =
    &[&SettingsTypes::Int32("some_int", "some_int", Some(123))];
