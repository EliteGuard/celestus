use vaultrs::client::{Client, VaultClient, VaultClientSettingsBuilder};

// Create a client
// let mut client = VaultClient::new(
//     VaultClientSettingsBuilder::default()
//         .address("https://127.0.0.1:8200")
//         .token("TOKEN")
//         .build()
//         .unwrap()
// ).unwrap();

pub struct Vault {
    client: Optoin<VaultClient>
}