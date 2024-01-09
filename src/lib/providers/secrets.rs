use std::collections::HashMap;

use getset::Getters;
use itertools::Itertools;
use log::warn;
use serde::Deserialize;

use crate::utils::strings::vec_to_uppercase;

pub const SETTING_USE_SECRETS_PROVIDER: &str = "use_secrets_provider";
pub const ENV_USE_SECRETS_PROVIDER: &str = "USE_SECRETS_PROVIDER";

pub const SETTING_SECRETS_PROVIDERS: &str = "secrets_providers";
pub const ENV_SECRETS_PROVIDERS: &str = "SECRETS_PROVIDERS";

#[derive(Debug, Clone, Deserialize, Getters)]
#[get = "pub with_prefix"]
pub struct SecretsProvider {
    name: Option<String>,
    prefix: Option<String>,
    host: Option<String>,
    port: Option<i32>,
    url: Option<String>,
    login_id: Option<String>,
    login_pass: Option<String>,
}

#[derive(Debug)]
pub struct SecretsProviders {
    pub providers: HashMap<String, SecretsProvider>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
struct SecretsProvidersEnvVar {
    secrets_providers: Option<Vec<String>>,
}

impl SecretsProviders {
    pub fn new() -> Self {
        let secrets_providers = match envy::from_env::<SecretsProvidersEnvVar>() {
            Ok(config) => config.secrets_providers,
            Err(_) => {
                warn!(
                    "{} is set to TRUE, but environment variable {} is not found",
                    ENV_USE_SECRETS_PROVIDER, ENV_SECRETS_PROVIDERS
                );
                None
            }
        };
        //info!("{:?}", secrets_providers);

        let found_secrets_providers = vec_to_uppercase(&mut secrets_providers.unwrap_or(vec![]));

        let references = found_secrets_providers
            .iter()
            .map(|s| s.as_str())
            .collect_vec();

        let mut read_providers = load_secrets_providers(&references);

        let valid_providers: Vec<SecretsProvider> = read_providers
            .iter_mut()
            .filter_map(|p| {
                if let Some(pn) = &p.name {
                    let new_name = pn;
                    p.name = Some(new_name.to_string().to_lowercase());
                    return Some(p.to_owned());
                } else {
                    None
                }
            })
            .collect();

        let mut providers = HashMap::<String, SecretsProvider>::new();

        for mut provider in valid_providers.into_iter() {
            providers.insert(provider.name.as_mut().unwrap().to_string(), provider);
        }

        Self { providers }
    }
}

fn load_secrets_providers<'a>(providers_names: &'a Vec<&'a str>) -> Vec<SecretsProvider> {
    let mut read: Vec<SecretsProvider> = [].to_vec();

    for provider in providers_names.iter() {
        let mut result =
            match envy::prefixed(format!("{}_", provider)).from_env::<SecretsProvider>() {
                Ok(sec_prov) => sec_prov,
                Err(error) => panic!("{:#?}", error),
            };
        result.name = Some(provider.to_string());
        result.prefix = Some(format!("{}_", provider));
        read.push(result.to_owned());
    }

    return read;
}
