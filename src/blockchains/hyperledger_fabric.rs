use std::ffi::{CStr, CString};

use crate::blockchains::Blockchain;
use crate::blockchains::utils::fabric::{_GoString_, CreateAsset, GetAsset, GoString, InitLedger};
use crate::std::error::Error;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct HyperLedgerFabric {
    ccp_path: String,
    cred_path: String,
}

impl HyperLedgerFabric {
    pub fn new(ccp_path: String, cred_path: String) -> HyperLedgerFabric {
        HyperLedgerFabric {
            ccp_path,
            cred_path,
        }
    }

    pub fn init(&self, smart_contract: String) {
        let c_path = CString::new(self.ccp_path.clone()).expect("CString::new failed");
        let ptr = c_path.as_ptr();
        let go_string = GoString {
            p: ptr,
            n: c_path.as_bytes().len() as isize,
        };

        let c_path = CString::new(self.cred_path.clone()).expect("CString::new failed");
        let ptr = c_path.as_ptr();
        let go_string2 = GoString {
            p: ptr,
            n: c_path.as_bytes().len() as isize,
        };

        let c_path = CString::new(smart_contract).expect("CString::new failed");
        let ptr = c_path.as_ptr();
        let go_smart_contract = GoString {
            p: ptr,
            n: c_path.as_bytes().len() as isize,
        };

        unsafe { InitLedger(go_string, go_string2, go_smart_contract); }
    }
}

impl Blockchain for HyperLedgerFabric {
    fn set_data(&self, smart_contract: String, signature: Vec<u8>, message: Vec<u8>, pubkey: Vec<u8>, device_type: String) -> Result<(), Error> {

        println!("Message {}", hex::encode(message.clone()));

        let c_path = CString::new(self.ccp_path.clone()).expect("CString::new failed");
        let ptr = c_path.as_ptr();
        let go_ccp_path = GoString {
            p: ptr,
            n: c_path.as_bytes().len() as isize,
        };

        let c_path = CString::new(self.cred_path.clone()).expect("CString::new failed");
        let ptr = c_path.as_ptr();
        let go_string2 = GoString {
            p: ptr,
            n: c_path.as_bytes().len() as isize,
        };

        let c_path = CString::new(smart_contract).expect("CString::new failed");
        let ptr = c_path.as_ptr();
        let go_smart_contract = GoString {
            p: ptr,
            n: c_path.as_bytes().len() as isize,
        };

        let c_path = CString::new(hex::encode(signature)).expect("CString::new failed");
        let ptr = c_path.as_ptr();
        let go_string3 = GoString {
            p: ptr,
            n: c_path.as_bytes().len() as isize,
        };

        let c_path = CString::new(hex::encode(message)).expect("CString::new failed");
        let ptr = c_path.as_ptr();
        let go_string4 = GoString {
            p: ptr,
            n: c_path.as_bytes().len() as isize,
        };

        let c_path = CString::new(hex::encode(pubkey)).expect("CString::new failed");
        let ptr = c_path.as_ptr();
        let go_string5 = GoString {
            p: ptr,
            n: c_path.as_bytes().len() as isize,
        };

        let c_path = CString::new(device_type).expect("CString::new failed");
        let ptr = c_path.as_ptr();
        let go_string6 = GoString {
            p: ptr,
            n: c_path.as_bytes().len() as isize,
        };

        let ret = unsafe {
            CreateAsset(
                go_ccp_path,
                go_string2,
                go_smart_contract,
                go_string3,
                go_string4,
                go_string5,
                go_string6)
        };
        thread::sleep(Duration::from_millis(1000));

        let c_str = unsafe { CStr::from_ptr(ret) };
        let data = c_str.to_str().expect("Error parsing CString");
        println!("{}", data);
        // thread::sleep(Duration::from_millis(1000));
        Ok(())
    }

    fn get_data(&self, smart_contract: String, pubkey: Vec<u8>, device_type: String, data_index: String) -> Result<Vec<u8>, Error> {
        let c_path = CString::new(self.ccp_path.clone()).expect("CString::new failed");
        let ptr = c_path.as_ptr();
        let go_string = GoString {
            p: ptr,
            n: c_path.as_bytes().len() as isize,
        };

        let c_path = CString::new(self.cred_path.clone()).expect("CString::new failed");
        let ptr = c_path.as_ptr();
        let go_string2 = GoString {
            p: ptr,
            n: c_path.as_bytes().len() as isize,
        };

        let c_path = CString::new(smart_contract).expect("CString::new failed");
        let ptr = c_path.as_ptr();
        let go_smart_contract = GoString {
            p: ptr,
            n: c_path.as_bytes().len() as isize,
        };

        let c_path = CString::new(hex::encode(pubkey)).expect("CString::new failed");
        let ptr = c_path.as_ptr();
        let go_string3 = GoString {
            p: ptr,
            n: c_path.as_bytes().len() as isize,
        };

        let c_path = CString::new(device_type).expect("CString::new failed");
        let ptr = c_path.as_ptr();
        let go_string4 = GoString {
            p: ptr,
            n: c_path.as_bytes().len() as isize,
        };

        let c_path = CString::new(data_index).expect("CString::new failed");
        let ptr = c_path.as_ptr();
        let go_string5 = GoString {
            p: ptr,
            n: c_path.as_bytes().len() as isize,
        };

        let ret = unsafe {
            GetAsset(
                go_string,
                go_string2,
                go_smart_contract,
                go_string3,
                go_string4,
                go_string5)
        };

        let c_str = unsafe { CStr::from_ptr(ret) };
        let data = c_str.to_str().expect("Error parsing CString");
        println!("{}", data);
        Ok(vec![])
    }
}

#[allow(dead_code)]
fn get_go_string(data: String) -> _GoString_ {
    let c_path = CString::new(data.clone()).expect("CString::new failed");
    let ptr = c_path.as_ptr();
    let go_string = GoString {
        p: ptr,
        n: c_path.as_bytes().len() as isize,
    };
    return go_string
}