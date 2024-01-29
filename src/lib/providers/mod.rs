pub mod data;
pub mod secrets;

use std::sync::Arc;

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

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DataProviderName(pub Arc<str>);

impl DataProviderName {
    // fn as_str(&self) -> &str {
    //     &self.0
    // }
}

impl From<&str> for DataProviderName {
    fn from(value: &str) -> Self {
        DataProviderName(value.into())
    }
}

impl From<String> for DataProviderName {
    fn from(value: String) -> Self {
        DataProviderName(value.into())
    }
}

impl From<&String> for DataProviderName {
    fn from(value: &String) -> Self {
        DataProviderName(value.as_str().into())
    }
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

    // pub(super) fn delete_implementation(&mut self) {
    //     self.implementation = None
    // }
}

pub trait DataProvisionActions {
    fn get_provision_type(&self) -> DataProvision;
    // fn fetch_data(&self);
}
