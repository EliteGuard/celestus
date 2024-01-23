mod vault;

use std::collections::HashMap;

use getset::Getters;
use log::warn;
use serde::Deserialize;

use self::vault::{Vault, VaultEnvData, VaultSecretsEngine, VAULT_SECRETS_PROVIDER_NAME};

use super::{DataProvider, DataProviderConnectivity, DataProvision};

pub const SETTING_USE_SECRETS_PROVIDER: &str = "use_secrets_provider";
pub const ENV_USE_SECRETS_PROVIDER: &str = "USE_SECRETS_PROVIDER";

pub const SETTING_SECRETS_PROVIDERS: &str = "secrets_providers";
pub const ENV_SECRETS_PROVIDERS: &str = "SECRETS_PROVIDERS";

#[derive(Clone, Deserialize)]
#[allow(dead_code)]
pub struct SecretsProviderInfo {
    host: String,
    port: i32,
    url: String,
}

#[derive(Getters)]
#[getset(get = "pub with_prefix")]
pub struct SecretsProviders {
    providers: HashMap<String, DataProvider<SecretsProviderInfo, SecretsProviderImplementation>>,
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
        let secrets_providers_names = load_secrets_providers_names();

        let found_secrets_providers = load_secrets_providers(&secrets_providers_names).await;

        let mut providers = HashMap::<
            String,
            DataProvider<SecretsProviderInfo, SecretsProviderImplementation>,
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
) -> Vec<DataProvider<SecretsProviderInfo, SecretsProviderImplementation>> {
    let mut read: Vec<DataProvider<SecretsProviderInfo, SecretsProviderImplementation>> =
        Vec::new();

    for provider_name in providers_names.iter() {
        let uppercase_name = provider_name.to_uppercase();

        if uppercase_name.contains(VAULT_SECRETS_PROVIDER_NAME) {
            read.extend(load_vault_secrets_provider(uppercase_name).await);
        } else {
            warn!(
                "{} is not referencing any currently supported Secrets Providers.\n
            Name must contain currently supported Secrets Providers \n
            Currently supported: Vault",
                provider_name
            )
        }
    }

    read
}

async fn load_vault_secrets_provider(
    provider_name: String,
) -> Vec<DataProvider<SecretsProviderInfo, SecretsProviderImplementation>> {
    let mut vault_providers: Vec<DataProvider<SecretsProviderInfo, SecretsProviderImplementation>> =
        Vec::new();

    let parsed_env_data =
            match envy::prefixed(format!("{}_", provider_name)).from_env::<VaultEnvData>() {
                Ok(sec_prov) => sec_prov,
                Err(error) => panic!("Encountered error during loading of Secrets Provider, the name \"{}\": {:#?} might be misspelled or related variables are missing", provider_name, error),
            };

    vault_providers.push(DataProvider {
        name: provider_name.to_string().to_lowercase(),
        prefix: format!("{}_", provider_name.to_string().to_lowercase()),
        basic_info: SecretsProviderInfo {
            host: parsed_env_data.get_host().to_string(),
            port: *parsed_env_data.get_port(),
            url: parsed_env_data.get_url().to_string(),
        },
        provision_type: DataProvision::OneTime,
        connectivity: DataProviderConnectivity::SingleConnection,
        implementation: SecretsProviderImplementation::Vault(
            Vault::new(parsed_env_data, VaultSecretsEngine::KV2).await,
        ),
    });

    vault_providers
}
