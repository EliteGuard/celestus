use std::collections::HashMap;

use serde::Deserialize;

use crate::utils::strings::vec_to_uppercase;

use super::{DataProvider, DataProviderConnectivity, DataProvision};

pub const SETTING_USE_SECRETS_PROVIDER: &str = "use_secrets_provider";
pub const ENV_USE_SECRETS_PROVIDER: &str = "USE_SECRETS_PROVIDER";

pub const SETTING_SECRETS_PROVIDERS: &str = "secrets_providers";
pub const ENV_SECRETS_PROVIDERS: &str = "SECRETS_PROVIDERS";

#[derive(Clone, Deserialize)]
#[allow(dead_code)]
pub struct SecretsProviderData {
    host: String,
    port: i32,
    url: String,
    login_id: String,
    login_pass: String,
}

pub struct SecretsProviders {
    pub providers: HashMap<String, DataProvider<SecretsProviderData, SecretsProviderImplementation>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
struct SecretsProvidersEnvVar {
    secrets_providers: Option<Vec<String>>,
}

#[derive(Clone, Copy)]
pub struct Vault;

#[derive(Clone, Copy)]
pub enum SecretsProviderImplementation{
    Vault(Vault)
}

impl SecretsProviders {
    pub fn new() -> Self {
        let secrets_providers = match envy::from_env::<SecretsProvidersEnvVar>() {
            Ok(config) => config.secrets_providers,
            Err(_) => {
                panic!(
                    "{} is set to TRUE, but environment variable {} is not found",
                    ENV_USE_SECRETS_PROVIDER, ENV_SECRETS_PROVIDERS
                )
            }
        };

        let found_secrets_providers = vec_to_uppercase(&mut secrets_providers.unwrap_or(vec![]));

        let read_providers = load_secrets_providers(&found_secrets_providers);

        let mut providers = HashMap::<String, DataProvider<SecretsProviderData, SecretsProviderImplementation>>::new();

        for provider in read_providers.into_iter() {
            providers.insert(provider.get_name().to_string(), provider);
        }

        Self { providers }
    }
}

fn load_secrets_providers<'a>(providers_names: &'a Vec<String>) -> Vec<DataProvider<SecretsProviderData, SecretsProviderImplementation>> {
    let mut read: Vec<DataProvider<SecretsProviderData, SecretsProviderImplementation>> = [].to_vec();

    for provider in providers_names.iter() {
        let result =
            match envy::prefixed(format!("{}_", provider)).from_env::<SecretsProviderData>() {
                Ok(sec_prov) => sec_prov,
                Err(error) => panic!("Encountered error during loading of Secrets Provider, the name \"{}\": {:#?} might be misspelled or related variables are missing", provider, error),
            };

        read.push(
            DataProvider
            { 
                name: provider.to_string().to_lowercase(), 
                prefix: format!("{}_", provider.to_string().to_lowercase()), 
                basic_info: result.to_owned(),
                provision_type: DataProvision::OneTime,
                connectivity: DataProviderConnectivity::SingleConnection,
                implementation: None
            },
        );
    }

    return read;
}
