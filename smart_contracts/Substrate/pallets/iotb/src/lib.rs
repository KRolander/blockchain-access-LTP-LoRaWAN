#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::inherent::Vec;
use frame_support::pallet_prelude::TypeInfo;
use frame_support::RuntimeDebug;

pub use pallet::*;

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum IdentityStatus {
    Known,
    Unknown,
}

pub enum ID {
    EndDevice,
    EdgeDevice,
}

impl ID {
    fn from_u32(value: u32) -> ID {
        match value {
            0 => ID::EndDevice,
            1 => ID::EdgeDevice,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Data {
    pub content: Vec<u8>,
}

impl MaxEncodedLen for Data where Data: Encode {
    fn max_encoded_len() -> usize {
        200
    }
}

#[frame_support::pallet]
pub mod pallet {
    use frame_support::inherent::Vec;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use k256::ecdsa::recoverable::{Signature as RecoverableSignature};
    use k256::ecdsa::signature::Signature;
    use sha2::{Digest, Sha256, digest::Update};
    use sp_runtime::DispatchError::{BadOrigin, Other};

    use crate::{Data, IdentityStatus, Result, ID};
    use crate::ID::EndDevice;
    use crate::IdentityStatus::Unknown;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn indices)]
    pub type Indices<T> = StorageMap<_, Blake2_128Concat, [u8; 32], u128>;

    #[pallet::storage]
    #[pallet::getter(fn data_map)]
    pub type DataMap<T> = StorageMap<_, Blake2_128Concat, [u8; 32], Data>;

    #[pallet::storage]
    #[pallet::getter(fn identities)]
    pub type Identities<T> = StorageMap<_, Blake2_128Concat, [u8; 32], IdentityStatus>;

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        DataStored(Result<u128, Error<T>>, Option<T::AccountId>),
        GetData(Result<Vec<u8>, Error<T>>, Option<T::AccountId>),
        Initialized(Result<(), Error<T>>, Option<T::AccountId>),
    }

    #[pallet::error]
    #[derive(Clone, PartialEq)]
    pub enum Error<T> {
        NoneValue,
        InvalidSignature,
        BadOrigin,
        UnauthorizedAccess,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn init(origin: OriginFor<T>) -> DispatchResult {
            let who = match ensure_signed(origin) {
                Ok(who) => who,
                Err(_) => {
                    Self::deposit_event(Event::Initialized(Result::Err(Error::BadOrigin), None));
                    return Err(BadOrigin);
                }
            };

            let mut test_ids = Vec::new();

            test_ids.push("02991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524");
            test_ids.push("02e607a962411371b76ffc2f9f4b5d0fb78992c0f7daf853f7b660533a14f96fe4");
            test_ids.push("0366d86f5d1da88a3df9f465a33281e2f3818117c803174f5aa7367accdcc516a6");
            test_ids.push("02e01d8fcf7996776ebec5c23e5dae66580d49ae9981b5aa4fee324d680ad15b5f");

            for id in test_ids {
                Self::put_id_to_state(EndDevice, Self::hex_to_bytes(id), IdentityStatus::Known);
                Self::put_counter_to_state(EndDevice, Self::hex_to_bytes(id), 0);
            }

            Self::deposit_event(Event::Initialized(Result::Ok(()), Some(who)));
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn put_data(origin: OriginFor<T>, signature: Vec<u8>, message: Vec<u8>, pubkey: Vec<u8>, device_type: u32) -> DispatchResult {
            let who = match ensure_signed(origin) {
                Ok(who) => who,
                Err(_) => {
                    Self::deposit_event(Event::DataStored(Result::Err(Error::BadOrigin), None));
                    return Err(BadOrigin);
                }
            };

            match Self::get_id_from_state(ID::from_u32(device_type), pubkey.clone()) {
                IdentityStatus::Known => {}
                Unknown => {
                    Self::deposit_event(Event::DataStored(Result::Err(Error::UnauthorizedAccess), Some(who.clone())));
                    return Err(BadOrigin);
                }
            }

            let message_u8 = message.as_slice();
            match Self::verify_recoverable_signature(signature,  message_u8, pubkey.clone() ){
                true => {}
                false => {
                            Self::deposit_event(Event::DataStored(Result::Err(Error::InvalidSignature), Some(who.clone())));
                            return Err(Other("Invalid signature"));
                        }
            };


            // match Self::verify_ecdsa_signature(signature, message.clone(), pubkey.clone()) {
            //     true => {}
            //     false => {
            //         Self::deposit_event(Event::DataStored(Result::Err(Error::InvalidSignature), Some(who.clone())));
            //         return Err(Other("Invalid signature"));
            //     }
            // };

            let current_index = Self::get_counter_from_state(ID::from_u32(device_type), pubkey.clone());
            Self::put_data_to_state(ID::from_u32(device_type), pubkey.clone(), current_index, message);
            Self::put_counter_to_state(ID::from_u32(device_type), pubkey.clone(), current_index + 1);

            Self::deposit_event(Event::DataStored(Result::Ok(current_index), Some(who)));

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn get_data(origin: OriginFor<T>, pubkey: Vec<u8>, device_type: u32, data_index: u128) -> DispatchResult {
            let who = match ensure_signed(origin) {
                Ok(who) => who,
                Err(_) => {
                    Self::deposit_event(Event::GetData(Result::Err(Error::BadOrigin), None));
                    return Err(BadOrigin);
                }
            };

            match Self::get_id_from_state(ID::from_u32(device_type), pubkey.clone()) {
                IdentityStatus::Known => {}
                Unknown => {
                    Self::deposit_event(Event::GetData(Result::Err(Error::UnauthorizedAccess), Some(who.clone())));
                    return Err(BadOrigin);
                }
            }

            match Self::get_data_from_state(ID::from_u32(device_type), pubkey, data_index) {
                Some(data) => {
                    Self::deposit_event(Event::GetData(Result::Ok(data), Some(who)));
                }
                None => {
                    Self::deposit_event(Event::GetData(Result::Err(Error::NoneValue), Some(who)));
                    return Err(Error::<T>::NoneValue.into());
                }
            };

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn verify_recoverable_signature<C: Verification>(secp: &Secp256k1<C>, msg: &[u8],sig: [u8; 65], pubkey: Vec<u8>) -> bool {
            let msg = sha256::Hash::hash(msg);
        
            let msg = MsgEcdsa::from_slice(&msg)?;
            
            let recovery_id = sig[64];
            
            let id = ecdsa::RecoveryId::from_i32(recovery_id as i32)?;
            let sig = ecdsa::RecoverableSignature::from_compact(&sig, id)?;
        
            let recovered_pubkey = secp.recover_ecdsa(&msg, &sig);
            match recovered_pubkey {
                Ok(recovered_pubkey) => { 
                    if recovered_pubkey == pubkey {
                        return true;
                    }
                    else
                    {
                        return false;
                    }
                    // println!("recovered Public key  {}",  hex::encode(res.serialize()));
                
                },
                Err(e) => {
                    return false;
                    // println!("Error while recovery {:?}\n", e);
                }
            }
        }
        
        
        
        pub fn verify_ecdsa_signature(signature: Vec<u8>, message: Vec<u8>, pubkey: Vec<u8>) -> bool {
            let rs = match RecoverableSignature::from_bytes(signature.as_slice()) {
                Ok(signature) => { signature }
                Err(_) => return false,
            };

            let hashed_message = Sha256::new().chain(message);
            match rs.recover_verify_key_from_digest_bytes(&hashed_message.finalize()) {
                Ok(key) => {
                    if key.to_bytes().to_vec() == pubkey {
                        true
                    } else {
                        false
                    }
                }
                Err(_) => { false }
            }
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

        pub fn get_id_from_state(id: ID, pubkey: Vec<u8>) -> IdentityStatus {
            let mut key_parts = Vec::new();
            key_parts.push((id as u32).to_ne_bytes().encode());
            key_parts.push(pubkey);

            let c_key = Self::create_composite_key(key_parts);
            match <Identities<T>>::get(c_key) {
                Some(identity) => identity,
                None => Unknown,
            }
        }

        pub fn put_id_to_state(id: ID, pubkey: Vec<u8>, value: IdentityStatus) {
            let mut key_parts = Vec::new();
            key_parts.push((id as u32).to_ne_bytes().encode());
            key_parts.push(pubkey);

            let c_key = Self::create_composite_key(key_parts);
            <Identities<T>>::insert(c_key, value);
        }

        pub fn get_counter_from_state(id: ID, pubkey: Vec<u8>) -> u128 {
            let mut key_parts = Vec::new();
            key_parts.push((id as u32).to_ne_bytes().encode());
            key_parts.push(pubkey);

            let c_key = Self::create_composite_key(key_parts);
            match <Indices<T>>::get(c_key) {
                Some(counter) => counter,
                None => 0,
            }
        }

        pub fn put_counter_to_state(id: ID, pubkey: Vec<u8>, value: u128) {
            let mut key_parts = Vec::new();
            key_parts.push((id as u32).to_ne_bytes().encode());
            key_parts.push(pubkey);

            let c_key = Self::create_composite_key(key_parts);
            <Indices<T>>::insert(c_key, value);
        }

        pub fn get_data_from_state(id: ID, pubkey: Vec<u8>, counter: u128) -> Option<Vec<u8>> {
            let mut key_parts = Vec::new();
            key_parts.push((id as u32).to_ne_bytes().encode());
            key_parts.push(pubkey);
            key_parts.push(counter.to_ne_bytes().encode());

            let c_key = Self::create_composite_key(key_parts);
            match <DataMap<T>>::get(c_key) {
                Some(counter) => Some(counter.content),
                None => None,
            }
        }

        pub fn put_data_to_state(id: ID, pubkey: Vec<u8>, counter: u128, data: Vec<u8>) {
            let mut key_parts = Vec::new();
            key_parts.push((id as u32).to_ne_bytes().encode());
            key_parts.push(pubkey);
            key_parts.push(counter.to_ne_bytes().encode());

            let c_key = Self::create_composite_key(key_parts);
            <DataMap<T>>::insert(c_key, Data { content: data });
        }

        pub fn hex_to_bytes(s: &str) -> Vec<u8> {
            (0..s.len())
                .step_by(2)
                .map(|i| s.get(i..i + 2)
                    .and_then(|sub| u8::from_str_radix(sub, 16).ok()))
                .collect::<Option<Vec<u8>>>()
                .unwrap()
        }
    }
}
