use getset::Getters;
use log::info;
use serde_derive::Deserialize;
use tokio::runtime::Runtime;
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
    runtime: Runtime,
}

impl Vault {
    pub fn new(provider_info: VaultEnvData, secrets_engine: VaultSecretsEngine) -> Self {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let client = create_vault_client(&provider_info);

        let mut created = Self {
            client,
            secrets_engine,
            connectivity: DataProviderConnectivity::SingleConnection,
            runtime,
        };

        created.approle_login(created.create_approle_login(&provider_info));

        let asd = created.get_kv_data::<PostgresData>();
        info!("{:#?}", asd);

        created
    }

    fn get_login_secret(&self, provider_info: &VaultEnvData) -> String {
        if is_dev_mode() {
            provider_info.login_pass.clone()
        } else {
            self.runtime
                .block_on(unwrap::<VaultWrappedSecret>(
                    &self.client,
                    Some(&provider_info.login_pass),
                ))
                .unwrap()
                .secret_id
        }
    }

    fn create_approle_login(&self, provider_info: &VaultEnvData) -> AppRoleLogin {
        AppRoleLogin {
            role_id: provider_info.login_id.clone(),
            secret_id: self.get_login_secret(provider_info),
        }
    }

    fn approle_login(&mut self, login: AppRoleLogin) {
        self.runtime
            .block_on(self.client.login("approle", &login))
            .unwrap()
    }

    fn get_kv_data<DataStruct: for<'de> serde::Deserialize<'de>>(&self) -> DataStruct {
        self.runtime
            .block_on(kv2::read::<DataStruct>(
                &self.client,
                "kv",
                "dev/celestus/database/pg",
            ))
            .unwrap()
    }
}

fn create_vault_client(provider_info: &VaultEnvData) -> VaultClient {
    let client_settings = VaultClientSettingsBuilder::default()
        .address(provider_info.url.clone())
        .token(provider_info.token.clone())
        .build()
        .unwrap();

    VaultClient::new(client_settings).unwrap()
}

impl FetchProviderData for Vault {
    fn fetch_data(&self) {
        // info!("Fetching.... {:#?}", self.client.settings)
    }
}
