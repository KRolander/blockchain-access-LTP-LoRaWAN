use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use codec::{Decode, Encode};
use sp_core::sr25519::Pair as P;
use substrate_api_client::{Api, PlainTipExtrinsicParams, StorageKey, UncheckedExtrinsicV4, XtStatus};
use substrate_api_client::ExtrinsicParams;
use substrate_api_client::Pair;
use substrate_api_client::rpc::WsRpcClient;
use std::time::Instant;
use crate::blockchains::Blockchain;
use crate::blockchains::utils::substrate::{create_composite_key, ID};
use crate::std::error::Error;
use crate::std::error::Error::{NoData, SubstrateSmartContract};

macro_rules! compose_extrinsic {
($api: expr, $pallet: expr, $call_name: expr, $nonce: expr $(, $args: expr) *) => {
        {
            use regex::Regex;
            use substrate_api_client::{compose_extrinsic_offline, UncheckedExtrinsicV4};

            let regexp = format!("\"{}\": PalletMetadata \\{{ index: (\\d+).*\"{}\": (\\d+)", $pallet, $call_name);
            let re = Regex::new(regexp.as_str()).unwrap();

            let text = format!("{:?}", $api.metadata.clone());
            let caps = re.captures(&text).unwrap();

            let pallet = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
            let call_index = caps.get(2).unwrap().as_str().parse::<u8>().unwrap();

            let call = ([pallet, call_index] $(, ($args)) *);

            if let Some(signer) = $api.signer.clone() {
                compose_extrinsic_offline!(
                    signer,
                    call,
                    $api.extrinsic_params($nonce))
            } else {
                UncheckedExtrinsicV4 {
                    signature: None,
                    function: call.clone(),
                }
            }
        }
    };
}

#[derive(Clone, Encode, Decode, PartialEq)]
pub struct Data {
    pub content: Vec<u8>,
}

pub struct Substrate {
    url: String,
    nonce: Arc<Mutex<u32>>,
    timeout: u64,
    api: Arc<Api<P, WsRpcClient, PlainTipExtrinsicParams>>,
}

impl Substrate {
    pub fn new(url: String, nonce: Arc<Mutex<u32>>, timeout: u64, api: Arc<Api<P, WsRpcClient, PlainTipExtrinsicParams>>) -> Substrate {
        Substrate {
            url,
            nonce,
            timeout,
            api,
        }
    }

    pub fn init(&self, smart_contract: String) {
        let smart_contract_clone = smart_contract.clone();
        let nonce_clone = self.nonce.clone();
        let api = self.api.clone();
        let mut nonce = nonce_clone.lock().unwrap();
        let xt: UncheckedExtrinsicV4<_, _> = compose_extrinsic!(
            api.clone(),
            smart_contract_clone,
            "init",
            *nonce
        );
        *nonce += 1;
        drop(nonce);

        api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock).unwrap();
    }
}

impl Blockchain for Substrate {
    fn set_data(&self, smart_contract: String, signature: Vec<u8>, message: Vec<u8>, pubkey: Vec<u8>, device_type: String) -> Result<(), Error> {
        let mut nonce = self.nonce.lock().unwrap();
        // thread::sleep(Duration::from_millis(100));

        let api = self.api.clone();

        let xt: UncheckedExtrinsicV4<_, _> = compose_extrinsic!(
                api.clone(),
                smart_contract,
                "put_data",
                *nonce,
                signature,
                message,
                pubkey,
                ID::from_string(device_type.as_str()) as u32
            );

        *nonce += 1;
        thread::sleep(Duration::from_millis(100));
        drop(nonce);


        match api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock) {
            Ok(_) => Ok(()),
            Err(e) => Err(SubstrateSmartContract(e))
        }
    }

    fn get_data(&self, smart_contract: String, pubkey: Vec<u8>, device_type: String, data_index: String) -> Result<Vec<u8>, Error> {
        let api = self.api.clone();

        let mut key_parts = Vec::new();
        key_parts.push((ID::from_string(device_type.as_str()) as u32).to_ne_bytes().encode());
        key_parts.push(pubkey);
        key_parts.push(data_index.parse::<u128>().unwrap().to_ne_bytes().encode());

        let key = create_composite_key(key_parts.clone());
        let encoded_key = key.encode();
        let x: &[u8] = encoded_key.as_slice();
        let blake2_key = sp_core::blake2_128(x)
            .iter()
            .chain(x.iter())
            .cloned()
            .collect::<Vec<_>>();

        let mut bytes = sp_core::twox_128(smart_contract.as_bytes()).to_vec();
        bytes.extend(&sp_core::twox_128("DataMap".as_bytes()));
        bytes.extend(blake2_key);
        let storage_key = StorageKey(bytes);

        let data: Option<Data> = api.get_storage_by_key_hash(storage_key, None).unwrap();
        match data {
            Some(data) => Ok(data.content),
            None => Err(NoData),
        }
    }
}