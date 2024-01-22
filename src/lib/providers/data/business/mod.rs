pub mod postgres;

#[allow(dead_code)]
pub struct BusinessDataProviderData {
    database_name: String,
    host: String,
    password: String,
    port: i32,
    url: String,
    url_prefix: Option<String>,
    user: String,
}
