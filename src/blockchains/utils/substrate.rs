use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use codec::Decode;
use protobuf::Message;
use sp_keyring::AccountKeyring;
use substrate_api_client::{AccountId, Api, PlainTipExtrinsicParams};
use substrate_api_client::rpc::WsRpcClient;
use crate::payload::payload::Message as Msg;
use sha2::{Digest, Sha256};

#[allow(unused)]
#[derive(Decode)]
pub struct GetData {
    pub data: SubstrateResult<Vec<u8>, SubstrateError>,
    account: Option<AccountId>,
}

impl Display for GetData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GetData")
    }
}

#[allow(unused)]
#[derive(Decode)]
pub struct DataStored {
    pub index: SubstrateResult<u128, SubstrateError>,
    account: Option<AccountId>,
}

impl Display for DataStored {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DataStored")
    }
}

#[allow(unused)]
#[derive(Decode)]
struct Initialized {
    result: SubstrateResult<(), SubstrateError>,
    account: Option<AccountId>,
}

impl Display for Initialized {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Initialized")
    }
}

#[derive(Clone, Decode)]
pub enum SubstrateResult<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Clone, PartialEq, Decode, Debug)]
pub enum SubstrateError {
    NoneValue,
    InvalidSignature,
    BadOrigin,
    UnauthorizedAccess,
}

impl Display for SubstrateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)
    }
}

impl std::error::Error for SubstrateError {}

pub enum ID {
    EndDevice,
    EdgeDevice,
}

impl ID {
    pub fn from_string(value: &str) -> ID {
        match value {
            "End" => ID::EndDevice,
            "Edge" => ID::EdgeDevice,
            _ => panic!("[SUBSTRATE] Unknown value: {}", value),
        }
    }
}

pub fn add_new_listener(smart_contract: String, event: String, substrate_url: String, events_queue: Arc<Mutex<HashSet<(String, String)>>>) {
    if events_queue.lock().unwrap().contains(&(smart_contract.clone(), event.clone())) {
        return;
    }

    events_queue.lock().unwrap().insert((smart_contract.clone(), event.clone()));
    thread::spawn(move || {
        let from = AccountKeyring::Alice.pair();
        let client = WsRpcClient::new(&substrate_url);
        let api = match Api::<_, _, PlainTipExtrinsicParams>::new(client)
            .map(|api| api.set_signer(from)) {
            Ok(api) => api,
            Err(_) => {
                events_queue.lock().unwrap().remove(&(smart_contract.clone(), event.clone()));
                panic!("[SUBSTRATE] Cannot connect to the api")
            }
        };

        let (events_in, events_out) = channel();
        match api.subscribe_events(events_in) {
            Ok(_) => {}
            Err(_) => {
                events_queue.lock().unwrap().remove(&(smart_contract.clone(), event.clone()));
                panic!("[SUBSTRATE] Cannot subscribe to events")
            }
        };

        loop {
            if event == "DataStored" {
                let data: DataStored = match api.wait_for_event_timeout(smart_contract.as_str(), event.as_str(), None, &events_out, Duration::from_secs(10)) {
                    Ok(data) => data,
                    Err(_) => break
                };

                match data.index {
                    SubstrateResult::Ok(i) => println!("[SUBSTRATE] ({}) Success: {}", &event, i),
                    SubstrateResult::Err(e) => println!("[SUBSTRATE] ({}) Error: {:?}", &event, e),
                };
            } else if event == "GetData" {
                let data: GetData = match api.wait_for_event_timeout(smart_contract.as_str(), event.as_str(), None, &events_out, Duration::from_secs(60)) {
                    Ok(data) => data,
                    Err(_) => break
                };

                match data.data {
                    SubstrateResult::Ok(data) => println!("[SUBSTRATE] ({}) Success: {:?}", &event, String::from_utf8(Msg::parse_from_bytes(data.as_slice()).unwrap().get_payload().to_vec()).unwrap()),
                    SubstrateResult::Err(e) => println!("[SUBSTRATE] ({}) Error: {:?}", &event, e),
                };
            } else {
                let data: Initialized = match api.wait_for_event_timeout(smart_contract.as_str(), event.as_str(), None, &events_out, Duration::from_secs(60)) {
                    Ok(data) => data,
                    Err(_) => break
                };

                match data.result {
                    SubstrateResult::Ok(()) => println!("[SUBSTRATE] ({}) Success", &event),
                    SubstrateResult::Err(e) => println!("[SUBSTRATE] ({}) Error: {:?}", &event, e),
                };
            }
        }

        events_queue.lock().unwrap().remove(&(smart_contract.clone(), event.clone()));
    });
}

pub fn create_composite_key(parts: Vec<Vec<u8>>) -> [u8; 32] {
    let concatenated = parts.iter()
        .fold(Vec::new(), |mut res: Vec<u8>, new| {
            res.extend(new.as_slice());
            res
        });
    let mut hasher = Sha256::new();
    sha2::Digest::update(&mut hasher, concatenated.as_slice());
    hasher.finalize().into()
}