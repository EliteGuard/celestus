use std::collections::HashMap;

use log::warn;
use serde::Deserialize;

use crate::utils::strings::vec_to_uppercase;

pub const SETTING_USE_SECRETS_PROVIDER: &str = "use_secrets_provider";
pub const ENV_USE_SECRETS_PROVIDER: &str = "USE_SECRETS_PROVIDER";

pub const SETTING_SECRETS_PROVIDERS: &str = "secrets_providers";
pub const ENV_SECRETS_PROVIDERS: &str = "SECRETS_PROVIDERS";

#[derive(Debug, Deserialize)]
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
    providers: HashMap<&'a str, SecretsProvider<'a>>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
struct SecretsProvidersEnvVar {
    secrets_providers: Option<Vec<String>>,
}

impl<'a> SecretsProviders<'a> {
    pub fn new() -> Self {
        
        let secrets_providers = match envy::from_env::<SecretsProvidersEnvVar>() {
            Ok(config) => config.secrets_providers,
            Err(_) => {
                warn!("{} is set to TRUE, but environment variable {} is not found", ENV_USE_SECRETS_PROVIDER, ENV_SECRETS_PROVIDERS);
                None
            }
        };

        let found_secrets_providers = vec_to_uppercase(&mut secrets_providers.unwrap_or(vec![]));
        
        for provider in found_secrets_providers.iter() {
            let prefixed = format!("{}_", provider);
            match envy::prefixed(prefixed).from_env::<SecretsProvider>() {
                Ok(sec_prov) => println!("{:#?}", sec_prov),
                Err(error) => panic!("{:#?}", error)
            }
        }
        

        let providers = HashMap::<&str, SecretsProvider>::new();

        Self { providers }
    }
}
