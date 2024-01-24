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
pub struct DataProvider<ConnectionInfo, Implementation> {
    name: String,
    prefix: String,
    #[getset(skip)]
    #[allow(dead_code)]
    connection_info: ConnectionInfo,
    provision_type: DataProvision,
    connectivity: DataProviderConnectivity,
    #[getset(skip)]
    implementation: Option<Implementation>,
}

impl<ConnectionInfo, Implementation> DataProvider<ConnectionInfo, Implementation> {
    pub fn get_implementation(&self) -> Option<&Implementation> {
        self.implementation.as_ref()
    }

    pub(super) fn delete_implementation(&mut self) {
        self.implementation = None
    }
}

pub trait FetchProviderData {
    fn fetch_data(&self);
}
