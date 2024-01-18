pub mod data;
pub mod secrets;

use getset::Getters;

#[derive(PartialEq)]
pub enum DataProvision {
    OneTime,
    OnDemand,
}

#[derive(PartialEq)]
pub enum DataProviderConnectivity {
    SingleConnection,
    ConnectionPool,
}

#[derive(Getters)]
#[getset(get = "pub with_prefix")]
pub struct DataProvider<BasicInfo, Implementation> {
    name: String,
    prefix: String,
    #[getset(skip)]
    #[allow(dead_code)]
    basic_info: BasicInfo,
    provision_type: DataProvision,
    connectivity: DataProviderConnectivity,
    implementation: Implementation,
}

pub trait FetchProviderData {
    fn fetch_data(&self);
}
