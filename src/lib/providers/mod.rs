pub mod secrets;

use derive_builder::Builder;
use getset::Getters;

#[derive(Clone, Copy)]
pub enum DataProvision {
    OneTime,
    OnDemand
}

#[derive(Clone, Copy)]
pub enum DataProviderConnectivity {
    SingleConnection,
    ConnectionPool
}

#[derive(Clone, Builder, Getters)]
#[getset(get = "pub with_prefix")]
pub struct DataProvider<BasicInfo, Implementation> {
    name: String,
    prefix: String,
    #[getset(skip)]
    #[allow(dead_code)]
    basic_info: BasicInfo,
    implementation: Option<Implementation>
}
