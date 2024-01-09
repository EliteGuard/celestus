use std::collections::HashMap;

pub const SETTING_USE_SECRETS_PROVIDER: &str = "use_secrets_provider";
pub const ENV_USE_SECRETS_PROVIDER: &str = "USE_SECRETS_PROVIDER";

pub const SETTING_SECRETS_PROVIDERS: &str = "secrets_providers";
pub const ENV_SECRETS_PROVIDERS: &str = "SECRETS_PROVIDERS";

#[derive(Default, serde_derive::Deserialize, derive_builder::Builder)]
#[builder(default)]
#[builder(setter(strip_option))]
pub struct SecretsProvider<'a>{
    name: Option<&'a str>,
    prefix: Option<&'a str>,
    host: Option<&'a str>,
    port: Option<i32>,
    url: Option<&'a str>,
    login_id: Option<&'a str>,
    login_pass: Option<&'a str>,
}

pub struct SecretsProviders<'a> {
    providers: HashMap<&'a str, &'a SecretsProvider<'a>>
}

// fn load_secrets_providers(
// ) -> Result<()> {

//     Ok(())
// }