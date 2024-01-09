use crate::{
    providers::secrets::{ENV_USE_SECRETS_PROVIDER, SETTING_USE_SECRETS_PROVIDER},
    utils::environment::{ENV_HOST_ENVIRONMENT, SETTING_HOST_MODE},
};

use super::SettingsTypes;

pub type AppSettings = &'static [&'static [&'static SettingsTypes<'static>]];

pub const APP_SETTINGS: AppSettings = &[BOOL_SETTINGS, INT32_SETTINGS];

pub const BOOL_SETTINGS: &'static [&'static SettingsTypes] = &[&SettingsTypes::Bool(
    SETTING_USE_SECRETS_PROVIDER,
    ENV_USE_SECRETS_PROVIDER,
    Some(false),
)];

pub const INT32_SETTINGS: &'static [&'static SettingsTypes] =
    &[&SettingsTypes::Int32("some_int", "some_int", Some(123))];

pub const STRING_SETTINGS: &'static [&'static SettingsTypes] = &[&SettingsTypes::String(
    SETTING_HOST_MODE,
    ENV_HOST_ENVIRONMENT,
    None,
)];
