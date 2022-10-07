use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Timeout,
    SubstrateSmartContract(substrate_api_client::std::error::Error),
    API,
    Network,
    NoData,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}