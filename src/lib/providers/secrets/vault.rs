use getset::Getters;
use log::info;
use serde_derive::Deserialize;
use tokio::runtime::Runtime;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv2;
use vaultrs_login::engines::approle::AppRoleLogin;
use vaultrs_login::LoginClient;

use crate::providers::data::business::postgres::PostgresData;
use crate::providers::{DataProviderConnectivity, DataProvision, DataProvisionActions};
use crate::utils::environment::{is_dev_mode, is_docker_host};
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
    login_id: String,
    login_pass: String,
    single_use: Option<bool>,
}

impl DataProvisionActions for VaultEnvData {
    fn get_provision_type(&self) -> DataProvision {
        match self.get_single_use() {
            Some(bool) => {
                if *bool {
                    DataProvision::OneTime
                } else {
                    DataProvision::OnDemand
                }
            }
            None => DataProvision::OneTime,
        }
    }
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
    base_path: String,
    runtime: Runtime,
}

impl Vault {
    pub fn new(provider_info: VaultEnvData, secrets_engine: VaultSecretsEngine) -> Self {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let client = create_vault_client(&provider_info);

        let base_path = match is_dev_mode() {
            true => "dev/celestus/".to_owned(),
            false => "celestus/".to_owned(),
        };

        let mut created = Self {
            client,
            secrets_engine,
            connectivity: DataProviderConnectivity::SingleConnection,
            base_path,
            runtime,
        };

        created.login_with_approle(created.create_approle_login(&provider_info));

        let asd = created.get_kv_data::<PostgresData>("database/pg");
        info!("{:#?}", asd);

        created
    }

    fn create_approle_login(&self, provider_info: &VaultEnvData) -> AppRoleLogin {
        AppRoleLogin {
            role_id: provider_info.login_id.clone(),
            secret_id: provider_info.login_pass.clone(),
        }
    }

    fn login_with_approle(&mut self, login: AppRoleLogin) {
        self.runtime
            .block_on(self.client.login("approle", &login))
            .unwrap()
    }

    fn get_kv_data<DataStruct: for<'de> serde::Deserialize<'de>>(&self, path: &str) -> DataStruct {
        self.runtime
            .block_on(kv2::read::<DataStruct>(
                &self.client,
                "kv",
                &format!("{}{}", self.base_path, path),
            ))
            .unwrap()
    }
}

fn create_vault_client(provider_info: &VaultEnvData) -> VaultClient {
    let address: String = if is_dev_mode() && is_docker_host() {
        "http://vault-dev:8201".to_owned()
    } else {
        provider_info.url.clone()
    };

    let client_settings = VaultClientSettingsBuilder::default()
        .address(address)
        .build()
        .unwrap();

    VaultClient::new(client_settings).unwrap()
}
