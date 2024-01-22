use log::info;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
use vaultrs::kv2;
use vaultrs::sys::wrapping::unwrap;
use vaultrs_login::engines::approle::AppRoleLogin;
use vaultrs_login::LoginClient;

use crate::providers::data::business::postgres::PostgresData;
use crate::providers::FetchProviderData;

use super::SecretsProviderData;

// Create a client
// let mut client = VaultClient::new(
//     VaultClientSettingsBuilder::default()
//         .address("https://127.0.0.1:8200")
//         .token("TOKEN")
//         .build()
//         .unwrap()
// ).unwrap();

// Create and read secrets
// #[derive(Debug, Deserialize, Serialize)]
// struct MySecret {
//     key: String,
//     password: String,
// }

// let secret = MySecret {
//     key: "super".to_string(),
//     password: "secret".to_string(),
// };
// kv2::set(
//     &client,
//     "secret",
//     "mysecret",
//     &secret,
// ).await;

// let secret: MySecret = kv2::read(&client, "secret", "mysecret").await.unwrap();
// println!("{}", secret.password) // "secret"

pub struct Vault {
    client: VaultClient,
}

impl Vault {
    pub async fn new(provider_info: &SecretsProviderData) -> Self {
        let mut client = VaultClient::new(
            VaultClientSettingsBuilder::default()
                .address(provider_info.url.clone())
                .build()
                .unwrap(),
        )
        .unwrap();

        // let unwrap_resp = unwrap::<String>(&client, Some(&provider_info.login_pass)).await;

        let login = AppRoleLogin {
            role_id: provider_info.login_id.clone(),
            secret_id: provider_info.login_pass.clone(),
        };

        let _ = client.login("approle", &login).await;

        let asd = kv2::read::<PostgresData>(&client, "kv", "dev/celestus/database/pg").await;
        info!("{:#?}", asd);

        Self { client }
    }
}

impl FetchProviderData for Vault {
    fn fetch_data(&self) {
        // info!("Fetching.... {:#?}", self.client.settings)
    }
}
