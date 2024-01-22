mod vault;

use std::collections::HashMap;

use log::warn;
use serde::Deserialize;

use crate::utils::strings::vec_to_uppercase;

use self::vault::Vault;

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
    pub providers:
        HashMap<String, DataProvider<SecretsProviderData, SecretsProviderImplementation>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
struct SecretsProvidersEnvVar {
    secrets_providers: Option<Vec<String>>,
}

pub enum SecretsProviderImplementation {
    Vault(Vault),
}

impl SecretsProviders {
    pub async fn new() -> Self {
        let mut secrets_providers_names = load_secrets_providers_names();

        let renamed_secrets_providers = vec_to_uppercase(&mut secrets_providers_names);

        let found_secrets_providers = load_secrets_providers(&renamed_secrets_providers).await;

        let mut providers = HashMap::<
            String,
            DataProvider<SecretsProviderData, SecretsProviderImplementation>,
        >::new();

        for provider in found_secrets_providers.into_iter() {
            providers.insert(provider.get_name().to_string(), provider);
        }

        Self { providers }
    }
}

fn load_secrets_providers_names() -> Vec<String> {
    match envy::from_env::<SecretsProvidersEnvVar>() {
        Ok(config) => config.secrets_providers.unwrap(),
        Err(_) => {
            panic!(
                "{} is set to TRUE, but environment variable {} is not found",
                ENV_USE_SECRETS_PROVIDER, ENV_SECRETS_PROVIDERS
            )
        }
    }
}

async fn load_secrets_providers(
    providers_names: &[String],
) -> Vec<DataProvider<SecretsProviderData, SecretsProviderImplementation>> {
    let mut read: Vec<DataProvider<SecretsProviderData, SecretsProviderImplementation>> =
        Vec::new();

    for provider in providers_names.iter() {
        let result =
            match envy::prefixed(format!("{}_", provider)).from_env::<SecretsProviderData>() {
                Ok(sec_prov) => sec_prov,
                Err(error) => panic!("Encountered error during loading of Secrets Provider, the name \"{}\": {:#?} might be misspelled or related variables are missing", provider, error),
            };

        let provider_implementation =
            create_secrets_provider_implementation(provider.to_lowercase().as_str(), &result).await;

        if provider_implementation.is_some() {
            read.push(DataProvider {
                name: provider.to_string().to_lowercase(),
                prefix: format!("{}_", provider.to_string().to_lowercase()),
                basic_info: result.to_owned(),
                provision_type: DataProvision::OneTime,
                connectivity: DataProviderConnectivity::SingleConnection,
                implementation: provider_implementation.unwrap(),
            });
        } else {
            warn!("{} is not currently supported", provider)
        }
    }

    read
}

async fn create_secrets_provider_implementation(
    provider_name: &str,
    provider_info: &SecretsProviderData,
) -> Option<SecretsProviderImplementation> {
    if provider_name.contains("vault") {
        Some(SecretsProviderImplementation::Vault(
            Vault::new(provider_info).await,
        ))
    } else {
        None
    }
}
