mod vault;

use std::collections::HashMap;

use getset::Getters;
use log::warn;
use serde::Deserialize;

use crate::utils::web::{URLData, URLInfo};

use self::vault::{Vault, VaultEnvData, VaultSecretsEngine, VAULT_SECRETS_PROVIDER_NAME};

use super::{DataProvider, DataProviderConnectivity, DataProvision};

pub const SETTING_USE_SECRETS_PROVIDER: &str = "use_secrets_provider";
pub const ENV_USE_SECRETS_PROVIDER: &str = "USE_SECRETS_PROVIDER";

pub const SETTING_SECRETS_PROVIDERS: &str = "secrets_providers";
pub const ENV_SECRETS_PROVIDERS: &str = "SECRETS_PROVIDERS";

pub type SecretsProvider = DataProvider<URLData, SecretsProviderImplementation>;

#[derive(Default, Getters)]
#[getset(get = "pub with_prefix")]
pub struct SecretsProviders {
    providers: HashMap<String, SecretsProvider>,
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
    pub fn new() -> Self {
        let secrets_providers_names = load_secrets_providers_names();

        let found_secrets_providers = load_secrets_providers(&secrets_providers_names);

        let mut providers = HashMap::<String, SecretsProvider>::new();

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

fn load_secrets_providers(providers_names: &[String]) -> Vec<SecretsProvider> {
    let mut read: Vec<SecretsProvider> = Vec::new();

    for provider_name in providers_names.iter() {
        let uppercase_name = provider_name.to_uppercase();

        if uppercase_name.contains(VAULT_SECRETS_PROVIDER_NAME) {
            read.extend(load_vault_secrets_provider(uppercase_name));
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

fn load_vault_secrets_provider(provider_name: String) -> Vec<SecretsProvider> {
    let mut vault_providers: Vec<SecretsProvider> = Vec::new();

    let parsed_env_data: VaultEnvData = load_provider_from_env::<VaultEnvData>(&provider_name);

    // vault_providers.extend(load_vault_secrets_providers());

    let connection_info = URLData {
        host: parsed_env_data.get_host().to_string(),
        port: parsed_env_data.get_port(),
        url: parsed_env_data.get_url().to_string(),
    };

    let implementation = Some(SecretsProviderImplementation::Vault(Vault::new(
        parsed_env_data,
        VaultSecretsEngine::KV2,
    )));

    vault_providers.push(DataProvider {
        name: provider_name.to_string().to_lowercase(),
        prefix: format!("{}_", provider_name.to_string().to_lowercase()),
        connection_info,
        provision_type: DataProvision::OneTime,
        connectivity: DataProviderConnectivity::SingleConnection,
        implementation,
    });

    vault_providers
}

fn load_provider_from_env<ProviderType: for<'a> Deserialize<'a>>(
    provider_name: &str,
) -> ProviderType {
    return match envy::prefixed(format!("{}_", provider_name)).from_env::<ProviderType>() {
            Ok(sec_prov) => sec_prov,
            Err(_) => panic!("Encountered error during loading of Secrets Provider, the name \"{}\" might be misspelled or related variables are missing", provider_name),
        };
}
