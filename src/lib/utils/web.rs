use serde::Deserialize;

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct URLData {
    pub(crate) host: String,
    pub(crate) port: i32,
    pub(crate) url: String,
}

pub trait URLInfo {
    fn get_url(&self) -> &str;
    fn get_host(&self) -> &str;
    fn get_port(&self) -> i32;
}

impl URLInfo for URLData {
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
