use crate::std::error::Error;

pub mod hyperledger_fabric;
pub mod substrate;

pub mod utils;


pub trait Blockchain {
    fn set_data(&self, smart_contract: String, signature: Vec<u8>, message: Vec<u8>, pubkey: Vec<u8>, device_type: String) -> Result<(), Error>;
    fn get_data(&self, smart_contract: String, pubkey: Vec<u8>, device_type: String, data_index: String) -> Result<Vec<u8>, Error>;
}
