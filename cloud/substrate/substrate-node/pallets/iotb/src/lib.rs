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
            // test_ids.push("02e607a962411371b76ffc2f9f4b5d0fb78992c0f7daf853f7b660533a14f96fe4");
            // test_ids.push("0366d86f5d1da88a3df9f465a33281e2f3818117c803174f5aa7367accdcc516a6");
            // test_ids.push("02e01d8fcf7996776ebec5c23e5dae66580d49ae9981b5aa4fee324d680ad15b5f");

            test_ids.push("0271c19d0e931b3798451a898273242152d0577d90df424f3a1b618e94337cc338");
            test_ids.push("035b659ad3047becfdcdf453c9b1bda7865e6b1ecbc71528d77151b9f184d0217a");
            test_ids.push("0266995b22143c71f538eaf202afa355be8e4571f15352624756a8f5e69b2d237f");
            test_ids.push("03d456dec60b3c512b3a6945944c1f036b1166650ac01a26d93e2db7aa044331d4");
            test_ids.push("03c3ebce94e6708edd3db351170ae11e147c5e6f7531541064c86b0b06e5a5670b");
            test_ids.push("0332bb31ad4817afcfb0be6be98958f9ce162a2426749b5324a52131ccdb71b84a");
            test_ids.push("024938f58f7909d6a860766b6ef37f7342060604fe9c69e11a8a433b665c08cba7");
            test_ids.push("029ae57fab30354c5626bbf48a5f9073d160050b957ddacd42acea5580f8d1ace2");
            test_ids.push("034ef3ddb92d08979837691680e733a021440fa3173c8d5e62eda3fe0221d4aa6b");
            test_ids.push("02e0791ab65108d4110f71f3992a30a18e4ce50e9ed24aeade5e9021b048beb083");
            test_ids.push("022ab35436af119aac8e3c64ec3f573d8404a764bc866d7fecc5d90ffbfe092652");
            test_ids.push("0298bcb7d9ede6d0458e4a926a4ebd49281a8e5909f328d63653a05198b6194740");
            test_ids.push("03f53ce4a30d5163df40b4c95a44ece1646a72fd68f477e81d03842feaad5ba168");
            test_ids.push("0393c2e6e88499acd89217a9e00cce93856ac3b1cee8c4892550dfbcb348717319");
            test_ids.push("031172b930b53ec635b66e5e757af6eca7f2f923aadd50a911d3e8dae1da7d8a2d");
            test_ids.push("03d36efcbebaa571858ef0ca255bbf28aff5edb0c11791ea48758a5e7fa8079812");
            test_ids.push("0349fc81f3b29661f445bf2517f8cb4eb125efb8c621f4fd1f42cb7db349445c7f");
            test_ids.push("0205ee7e84e4cbb618944bbce8d2957117b280f68043c957173966b433407eecac");
            test_ids.push("0244781d4b8287c0a11daaf94dcecf11d4b9ab9b7a41f904a03f049bfdbbc5ea66");
            test_ids.push("03c445efabf8edd512a2c9b8a731c980d8a3845eaf8de2e4c2765597fd051897b4");
            test_ids.push("03ec1149d2713ad66baf150d4647aeb1571d96afb9a46f68e81329071d6f9c8e76");
            test_ids.push("036233cf0604a9b0b5a418bf528066bd219320b44e729af7a943a4c923c5cd516e");
            test_ids.push("026d8236414b72631c481214810235d7b0be1a44892ee2fb592d1a8d0510230083");
            test_ids.push("0215016c213839df971094557ae9bacec677da55425cd130582c203d2b8da6e7c3");
            test_ids.push("023bbdbc19096cd54e52d7e4c52f1a42dd2de0fe2d3ffa3b9c469d33cb8322d9e4");
            test_ids.push("03159d79daf83f6f44e10635ed8e988a10a971fb66471359d7d21fc211dfed83e6");
            test_ids.push("02f228e52801dcad8bfd21d918ccdbd8864ec26ee76f6b7f082938cb1d898dffb0");
            test_ids.push("028a437f0c11aa4c05b9754aca7676052c4f768f8d0e2c61271d99d2c4ebdfe557");
            test_ids.push("03f3867cd69dda9cdff3628b97a2d7d28038da6d701758c882a816743234dc1a60");
            test_ids.push("03b89b85f66dade6774e5406e514d26828875375c5e385c30261ec200bc6a5e351");
            test_ids.push("03eb1178395c6a14ef94f9b680f7756895b5ce7af8b91a12dcce97da8f67e2b5e7");
            test_ids.push("0393b14c6b010a9a780814190345cd6eb5dafa1811e1aabb2567d689013c66ee44");
            test_ids.push("033597f1e0cd54d99df7a1ef9189cc401cb304ea5729a7c1fe7246ebcecabac8ee");
            test_ids.push("036e190ce6244d89773ffe6eff223c8f7eb97b29794324b94f1857d0df585ee27d");
            test_ids.push("0240667e25a522c3694aa5f135195f538ae79392c93fc7efd70a4b033481f7c2e6");
            test_ids.push("021463ff3ccc9c31780b035e49d43cd38030aa6b1aa030d61093b92eef419a07be");
            test_ids.push("026749143e5a372de6b5f0b9b40725551e834474b110f8946ad54df04253243c72");
            test_ids.push("038dd064a797e8adab1475274d06b8ebc2405af6184187262bf07077ca235bebc3");
            test_ids.push("027391a65fa712422e5ed7f502b9f731b434c12f9144422433ed4979afa1b3c852");
            test_ids.push("03848dcfe31243606d13914209f9c0ce354a0e216ec6e73aa5b38e2e22401361a2");
            test_ids.push("0343036b114c1d72a6e5e58ed92417e7ad75080e1852a3973d6298c201ed38a2d1");
            test_ids.push("02a0dc9c4a587b8f891a2f7ae84db5ff702e8dca3cfc1969846d2405f80d3c99d1");
            test_ids.push("02907c595ef5c95bc0ea357a4eeb0d5b5a3f0aefae0e4f09f29baaafd84f6a8767");
            test_ids.push("03aa2090c7492de0051337c79b3dce489aa0ab15a56d8b452218dba51e9c1455b4");
            test_ids.push("031501acba8eef45d03c10daef47ee82a12071d878a727f0add683cc71715ccba2");
            test_ids.push("0238ba852a2f881a40ae5fb203593ebabc032a44f32cf3af11ca926d2622d39058");
            test_ids.push("0342ff0ae3d093101831a340f4c0025a20a6cbb6ef115b9096a9fcbcd63d25653c");
            test_ids.push("021b893deeade96e37f9eb9c74483bc282e38620560064a0fba44016ebfcbb6ad8");
            test_ids.push("034761343418bab4d358e073a76e7ce57ed66e880f6973b0b82bb535a51c63e612");
            test_ids.push("038210d427767fb4d914419e54322ae9b3782e1e8fe1d39dae29a984ad3d3e0434");
            test_ids.push("026bd435cac45594bc7160ee173f3335407c4122e756092108d527690f95e90c3e");
            test_ids.push("02e5391000f75188324edffe72d07e48b6d88f7132ee0c5b9978a8b133dc38ca7f");
            test_ids.push("03151b15446bd67375539e9c620f42bb9947069b00a1b013dc10654e72f4b62994");
            test_ids.push("03da0f6bc16a0cf1350e230c9d6ec5664e395d518474b77e2926e25958de80130d");
            test_ids.push("02efb3fb089647f27813d1f6db5efe6887f9f787186e874aff152a9ef9b5b62e10");
            test_ids.push("0302563f099a5aabd6b12d4a6fb074798f03679de9c6db6e8bbaaeaf994a5b5b64");
            test_ids.push("021334854c944015d458e25f076ba177496119075522b617b4f681f2f25b3a33a0");
            test_ids.push("03982d78ed21277a394a2624a23c0588bd7540487092953184dee4e9d33c0e2be1");
            test_ids.push("023fa63b66568b4fd5c52e94c09e859891ed102877a3085b788103b807673ce77f");
            test_ids.push("02f037a7170f00c295a47684f8ef77f5fb2ede593b5076521fc3ff04b25a4de65b");
            test_ids.push("020fd236a5ff33459b1b0e24007e53c877e3f590c217e435b74b295858c442c620");
            test_ids.push("03e33fd567f6e8592ced901d3d76f834ce255b04db8e1382332be74276691ca507");
            test_ids.push("02a8474a54ad7fdb4ae04655577e18c749edd11aaf3abcb8d14a7f2b01db9e05dd");
            test_ids.push("037b64bd9fbc499600ac0fc4ff0e1ff0a5e09dd07d04c3a2e672031d0287b71c3b");
            

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

            match Self::verify_ecdsa_signature(signature, message.clone(), pubkey.clone()) {
                true => {}
                false => {
                    Self::deposit_event(Event::DataStored(Result::Err(Error::InvalidSignature), Some(who.clone())));
                    return Err(Other("Invalid signature"));
                }
            };

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
