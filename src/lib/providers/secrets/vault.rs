use anyhow::Error;
use getset::Getters;
use log::info;
use serde_derive::Deserialize;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv2;
use vaultrs::sys::wrapping::unwrap;
use vaultrs_login::engines::approle::AppRoleLogin;
use vaultrs_login::LoginClient;

use crate::providers::data::business::postgres::PostgresData;
use crate::providers::{DataProviderConnectivity, FetchProviderData};
use crate::utils::environment::is_dev_mode;
use crate::utils::web::URLInfo;

pub const VAULT_SECRETS_PROVIDER_NAME: &str = "VAULT";

#[derive(Deserialize, Getters)]
#[getset(get = "pub with_prefix")]
pub struct VaultEnvData {
    #[getset(skip)]
    host: String,
    #[getset(skip)]
    port: i32,
    #[getset(skip)]
    url: String,
    engine: String,
    token: String,
    login_id: String,
    login_pass: String,
    single_use: Option<String>,
}

impl URLInfo for VaultEnvData {
    fn get_url(&self) -> &str {
        self.url.as_str()
    }

    fn get_host(&self) -> &str {
        self.host.as_str()
    }

    fn get_port(&self) -> i32 {
        self.port
    }
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct VaultWrappedSecret {
    secret_id: String,
    secret_id_accessor: String,
    secret_id_ttl: i32,
}

pub enum VaultSecretsEngine {
    KV2,
}

#[derive(Getters)]
#[getset(get = "pub with_prefix")]
pub struct Vault {
    client: VaultClient,
    connectivity: DataProviderConnectivity,
    secrets_engine: VaultSecretsEngine,
}

impl Vault {
    pub fn new(provider_info: VaultEnvData, secrets_engine: VaultSecretsEngine) -> Self {
        let client_settings = VaultClientSettingsBuilder::default()
            .address(provider_info.url.clone())
            .token(provider_info.token.clone())
            .build()
            .unwrap();

        let mut client = VaultClient::new(client_settings).unwrap();

        let secret_id: String = if is_dev_mode() {
            provider_info.login_pass.clone()
        } else {
            smol::block_on(unwrap::<VaultWrappedSecret>(
                &client,
                Some(&provider_info.login_pass),
            ))
            .unwrap()
            .secret_id

            // unwrap::<VaultWrappedSecret>(&client, Some(&provider_info.login_pass))
            //     .await
            //     .unwrap()
            //     .secret_id
        };

        let login = AppRoleLogin {
            role_id: provider_info.login_id.clone(),
            secret_id,
        };

        smol::block_on(async {
            let _ = client.login("approle", &login).await;

            let asd = kv2::read::<PostgresData>(&client, "kv", "dev/celestus/database/pg").await;
            info!("{:#?}", asd);
        });

        Self {
            client,
            secrets_engine,
            connectivity: DataProviderConnectivity::SingleConnection,
        }
    }
}

impl FetchProviderData for Vault {
    fn fetch_data(&self) {
        // info!("Fetching.... {:#?}", self.client.settings)
    }
}
