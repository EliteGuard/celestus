use log::info;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

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

// impl Default for Vault {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl Vault {
    pub fn new(provider_info: &SecretsProviderData) -> Self {
        let client = VaultClient::new(
            VaultClientSettingsBuilder::default()
                .address(provider_info.url.clone())
                .token("TOKEN")
                .build()
                .unwrap(),
        )
        .unwrap();

        Self { client }
    }
}

impl FetchProviderData for Vault {
    fn fetch_data(&self) {
        info!("Fetching.... {:#?}", self.client.settings)
    }
}
