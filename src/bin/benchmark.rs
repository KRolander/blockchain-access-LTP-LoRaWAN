extern crate iotb_gateway;
extern crate paho_mqtt as mqtt;

extern crate bitcoin_hashes;
extern crate secp256k1; 

use std::collections::HashSet;
// use std::io::Read;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use clap::Parser;
use iotb_gateway::blockchains::Blockchain as OtherBlockchain;
use json::object;
use k256::ecdsa::recoverable::Signature as RecoverableSignature;
use k256::ecdsa::signature::{Signature, Signer};
use k256::ecdsa::{Signature as Sign, SigningKey};
use protobuf::Message;
use sha2::{digest::Update, Digest, Sha256};
use sp_keyring::AccountKeyring;
// use sp_runtime::Serialize;
use substrate_api_client::rpc::WsRpcClient;
use substrate_api_client::{Api, PlainTipExtrinsicParams};

use iotb_gateway::blockchains::substrate::Substrate;
use iotb_gateway::payload::payload::Message as Msg;
use iotb_gateway::payload::payload::{Action, Blockchain, Payload};

use rand::distributions::{Distribution, Uniform};

use bitcoin_hashes::{sha256, Hash};

use secp256k1::{Error, Message as MsgEcdsa, PublicKey, Secp256k1, SecretKey, Signing, Verification, ecdsa};

// use sp_core::{ecdsa as spEcdsa, Pair};
// use sp_io::crypto::secp256k1_ecdsa_recover_compressed;



#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Private key for signing
    #[clap(
        long,
        value_parser,
        default_value = "ec3c161597b119998ad8822acb2b1123405e857577e5e8abd678ad8c2383f20c"
    )]
    private_key: String,

    /// Corresponding public key
    #[clap(
        long,
        value_parser,
        default_value = "02991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524"
    )]
    public_key: String,

    /// Blockchain
    #[clap(long, value_parser, default_value = "substrate")]
    blockchain: String,

    /// Smart contract name
    #[clap(long, value_parser, default_value = "Iotb")]
    smart_contract: String,

    /// MQTT url
    #[clap(long, value_parser, default_value = "tcp://localhost:1883")]
    mqtt_url: String,

    /// MQTT topic
    #[clap(long, value_parser, default_value = "gateway/gatewayid/event/uplink")]
    mqtt_topic: String,

    // -------------------------
    /// Number of parallel messages (ordering not guaranteed)
    #[clap(long, value_parser, default_value_t = 1)]
    parallel_messages: u16,

    /// Delay between messages in ms (between groups of parallel messages)
    #[clap(long, value_parser, default_value_t = 100)]
    delay: u16,

    /// Total number of messages to send
    #[clap(long, value_parser, default_value_t = 1)]
    total_messages: u16,

    // -------------------------
    /// Number of messages to verify
    #[clap(long, value_parser, default_value_t = 0)]
    verify: u16,

    /// Substrate URL
    #[clap(long, value_parser, default_value = "")]
    substrate_url: String,
}

fn main() {
    // let pubKeyArray = ["02991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524", "02991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89525"];

    let mut privKeyArray = [
"ec3c161597b119998ad8822acb2b1123405e857577e5e8abd678ad8c2383f20c",
"bea2d079dae4d3d0a8c5b0414ab1ffe9a537691f53c90050a1875e209e262f94",
"1cee99ad7d2885dda73d1deb7dee710fe2af3f440c3e852dd7b242302efc8b2c",
"6319f9847e783c25be8e870062200fa8d73426a7f40456f3204bae52e2c41888",
"1a0cc2d832a51c155f013f007e142e73c46841826c97d76eb507ec76d02bd207",
"be257c352719161ed17dc3135fa427e95d94f7cf986c5497740faf450a5415c7",
"a720b775459c20ecd59f582b0b514df511602d358e1a339dcf98daabb83013d4",
"09395ae81952e11f741075cf7e2b5a32ff577b8acdef5a46f89a5da29476a45f",
"b8f74bc19ab8955915ef3ed43da44eb00eaaf2e70b396d76dd86c9dbdac77759",
"96e781f95d47f695a652316d3de43044e79366e10b82e3141817f7630dfef6c6",
"58a2cb387e4dc50ab6c3872c93aff4c973695d32c63fe4a7aef3799a61b81526",
"c39490cc6a7f675f618bead28b6d5bf4ef1f97422bc27d8e775b306fdfe3c56d",
"a301f120c505560cfa9aa6e53f7cefcf9ca260554d2ad739e8d065b120d71d52",
"cc17e1c2c88068d7d282b0cbca36a36ab9445e2114aa92886625407d6ee7c7a0",
"ef59d549843f9cb52bb608ce4b05d2d285b634063b4a7cfd02fcf3ddf87fced1",
"1dbb0c5f548d73c8a811dc61efd54977e7aa69b5cf8cf752de038f3742e6dfed",
"8303b4909e15f519c14d4cf8686f241b8ac52bcf3f174b3e5d10349e69eec4bc",
"fc501bc756619548f4442eb050315a92a3c0d843183ef2fb59bcf6544016e3f4",
"26f5ffc0486e5d964e5fd8ef3ad2c8a3693ac9963fc1f59ef191afe7849faefe",
"1c307c3d59e5b991f0a1d559074e2412159feada89ed94736fd49b9fe7db9df1",
"909c6aac319cb0d4cc0eb5abf81df1718897d201091cd79fd88840a10c98bdf1",
"fb4a83000772d23cf92523c6ef8770d6fab9371c6486c91935a9acfca0c0fba8",
"532450acc41bdec66b4076d054db50059e3ebeceea55441425ccbf1e4b117d38",
"ba85012043690072dbd27bc4372e9ca3f800a3d601392351f8b5b7e5dd5d4ac7",
"55546c88be97c479baec96d64ca060ce659246a040444e8ae10a857dd01dbf57",
"5e29a89f311640b7a3258462ef5ff61744c16b099927e13d277f420511f2e911",
"0ea4a5340b65aa35e85b72854285506d45b9d33a9b60fea9e4dd3dcc761128bb",
"2327ab58f4e54ee470e83b25bc3513e27a0c8fe2aabd81a02a91ace455b00daa",
"94f4e5f35480ba8058b37fc085868064ad01aff3fdd4258991998fd9cc3447f6",
"0301225b31e4aafd1f46372d7d84be72fbda2022818c5714d3e88ee8cce1f1bd",
"dda69db4661dbccd299b03ca71fb9976dc6b68916abfa589c0556bc38b6c68f3",
"46778cccc9293e368167f62ea27fcac7157230e1c303cb977be94f98aa950517",
"c54c7a5d5821a4cef030b83a887e775f071571e5f645bfef1f8155b3b458af16",
"fe55b083da8988817267e8be3c550b6847741e264d232c2308e7a2e3f72252fc",
"8a0af2aac3120150a24aa77905e4022692a20ed07353877d3d49b5435242ffbb",
"abf5c9937dd7eadc982ec57b80fbc9211844a797a3c386b7a593ffc607cec541",
"9ad06c3833ff89f4d2b7539ae3c830835b12f5ce290085390045a5838cfc4bcc",
"57823a338cd699e11326002d8cabd06d88394371ffdd7d9d2301a3d3ba56fc13",
"60c32043daa363a4739a14aae027b67c5d42370df775ddf54d482f5162120335",
"6e5cee4431c80e605d7adb6c26b47b89da1e4add6aaba6bb3c03a934eae2caac",
"d5132461d4a988400d4c5682d435cbee97ee03afbf9bf20f503bd98a3608c112",
"452eed9e1a3dd25c9c01e473d9a518b3a332cfe295f1d2da9a80842c957deda6",
"48aeb79d2639eb61568ac44f58d48d1edea8debb0ed759c5ff1e58766a429d4c",
"93aeb6bdf64b9314a2b9fd5814593cc66f146a94834d8513918ce7ebe2a38365",
"c65c26a0ba08443e88a2920bfab064cdf28f51c42490ae59632a8801006001e6",
"5189f8bf19cb481dcbbd7c00303c24806a694a04adf7da18fe3d4cd9954bf302",
"81502d80b05ecafd661f78b0cca4f775e45650cdbeabf26b40de3c50d286836b",
"b2975893e3df168b012af147998ebd78bc265141063184069ed559c2d52400af",
"273b6bc9f6ab8f4feefa1bce0ddca684ca34b32203592e1735b4020ad752dbfe",
"2aaa736398291b358ea0e6ea779eb56ca80eb28e5f8cd1e33187d22457170d3d",
"84a61a26e01a409d588794e1ecdf9e512448fb499019c5ead433eb67f341e7d1",
"f51892b4a5662707e892dadc0a1c38be338926ac394e5ae7c11f4f2c9153b20a",
"29997fbeb62e42fc0f19d537bd2b20b9adf4dab214c140ec99ed299fbdbe41dc",
"a99c4132268cffc2f9b2446ca7c738cf60ce12233da7eb3ea61b9af2abf056a4",
"7bbcd6e927c0b7b776e6a5a84d0218dc938b6422179d5ef2f5c17b91f3905be2",
"b753d62881fba9168fd16e1a9ce6b9c5b8563031a176490ec813c637c8397577",
"828b2858bf81094f2ac573f5f626ed3dad08fadaa2d98470e485c50840c5cef2",
"70847bd961fb304304a161e161144d5d16d62156916d9d4cc817508f75aa14be",
"19a87aa30461fd0cfe779379beab00f5e510997fb048935581a7e6f4fa5717b3",
"7f0362006e80de843eabcd3839d0035632b4627b782fea67bcdb8ceba5ac9051",
"e8e72959adf57a177ae4ae9d3d833c90e860dccce1158b1a612bf931babf7925",
"840cd4708b426c04b9b31692615a03e032672f3af409a5f9e17e8380cacab6b9",
"c347217da42817128f776a20ab77236157dc9d4fb9f507d31cbcc5a352d9b6b8",
"65152bb2ed849d312ca40900bbd03231c23ad7c57be6c3840da9e85deed974c0",
"2ac1bc16ebf4b7b51e3e4120d9268770cbc5650cef78428ddb3071dcaf2e82af",
    ];

 
    let mut pubKeyArray = [
"02991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524",
"0271c19d0e931b3798451a898273242152d0577d90df424f3a1b618e94337cc338",
"035b659ad3047becfdcdf453c9b1bda7865e6b1ecbc71528d77151b9f184d0217a",
"0266995b22143c71f538eaf202afa355be8e4571f15352624756a8f5e69b2d237f",
"03d456dec60b3c512b3a6945944c1f036b1166650ac01a26d93e2db7aa044331d4",
"03c3ebce94e6708edd3db351170ae11e147c5e6f7531541064c86b0b06e5a5670b",
"0332bb31ad4817afcfb0be6be98958f9ce162a2426749b5324a52131ccdb71b84a",
"024938f58f7909d6a860766b6ef37f7342060604fe9c69e11a8a433b665c08cba7",
"029ae57fab30354c5626bbf48a5f9073d160050b957ddacd42acea5580f8d1ace2",
"034ef3ddb92d08979837691680e733a021440fa3173c8d5e62eda3fe0221d4aa6b",
"02e0791ab65108d4110f71f3992a30a18e4ce50e9ed24aeade5e9021b048beb083",
"022ab35436af119aac8e3c64ec3f573d8404a764bc866d7fecc5d90ffbfe092652",
"0298bcb7d9ede6d0458e4a926a4ebd49281a8e5909f328d63653a05198b6194740",
"03f53ce4a30d5163df40b4c95a44ece1646a72fd68f477e81d03842feaad5ba168",
"0393c2e6e88499acd89217a9e00cce93856ac3b1cee8c4892550dfbcb348717319",
"031172b930b53ec635b66e5e757af6eca7f2f923aadd50a911d3e8dae1da7d8a2d",
"03d36efcbebaa571858ef0ca255bbf28aff5edb0c11791ea48758a5e7fa8079812",
"0349fc81f3b29661f445bf2517f8cb4eb125efb8c621f4fd1f42cb7db349445c7f",
"0205ee7e84e4cbb618944bbce8d2957117b280f68043c957173966b433407eecac",
"0244781d4b8287c0a11daaf94dcecf11d4b9ab9b7a41f904a03f049bfdbbc5ea66",
"03c445efabf8edd512a2c9b8a731c980d8a3845eaf8de2e4c2765597fd051897b4",
"03ec1149d2713ad66baf150d4647aeb1571d96afb9a46f68e81329071d6f9c8e76",
"036233cf0604a9b0b5a418bf528066bd219320b44e729af7a943a4c923c5cd516e",
"026d8236414b72631c481214810235d7b0be1a44892ee2fb592d1a8d0510230083",
"0215016c213839df971094557ae9bacec677da55425cd130582c203d2b8da6e7c3",
"023bbdbc19096cd54e52d7e4c52f1a42dd2de0fe2d3ffa3b9c469d33cb8322d9e4",
"03159d79daf83f6f44e10635ed8e988a10a971fb66471359d7d21fc211dfed83e6",
"02f228e52801dcad8bfd21d918ccdbd8864ec26ee76f6b7f082938cb1d898dffb0",
"028a437f0c11aa4c05b9754aca7676052c4f768f8d0e2c61271d99d2c4ebdfe557",
"03f3867cd69dda9cdff3628b97a2d7d28038da6d701758c882a816743234dc1a60",
"03b89b85f66dade6774e5406e514d26828875375c5e385c30261ec200bc6a5e351",
"03eb1178395c6a14ef94f9b680f7756895b5ce7af8b91a12dcce97da8f67e2b5e7",
"0393b14c6b010a9a780814190345cd6eb5dafa1811e1aabb2567d689013c66ee44",
"033597f1e0cd54d99df7a1ef9189cc401cb304ea5729a7c1fe7246ebcecabac8ee",
"036e190ce6244d89773ffe6eff223c8f7eb97b29794324b94f1857d0df585ee27d",
"0240667e25a522c3694aa5f135195f538ae79392c93fc7efd70a4b033481f7c2e6",
"021463ff3ccc9c31780b035e49d43cd38030aa6b1aa030d61093b92eef419a07be",
"026749143e5a372de6b5f0b9b40725551e834474b110f8946ad54df04253243c72",
"038dd064a797e8adab1475274d06b8ebc2405af6184187262bf07077ca235bebc3",
"027391a65fa712422e5ed7f502b9f731b434c12f9144422433ed4979afa1b3c852",
"03848dcfe31243606d13914209f9c0ce354a0e216ec6e73aa5b38e2e22401361a2",
"0343036b114c1d72a6e5e58ed92417e7ad75080e1852a3973d6298c201ed38a2d1",
"02a0dc9c4a587b8f891a2f7ae84db5ff702e8dca3cfc1969846d2405f80d3c99d1",
"02907c595ef5c95bc0ea357a4eeb0d5b5a3f0aefae0e4f09f29baaafd84f6a8767",
"03aa2090c7492de0051337c79b3dce489aa0ab15a56d8b452218dba51e9c1455b4",
"031501acba8eef45d03c10daef47ee82a12071d878a727f0add683cc71715ccba2",
"0238ba852a2f881a40ae5fb203593ebabc032a44f32cf3af11ca926d2622d39058",
"0342ff0ae3d093101831a340f4c0025a20a6cbb6ef115b9096a9fcbcd63d25653c",
"021b893deeade96e37f9eb9c74483bc282e38620560064a0fba44016ebfcbb6ad8",
"034761343418bab4d358e073a76e7ce57ed66e880f6973b0b82bb535a51c63e612",
"038210d427767fb4d914419e54322ae9b3782e1e8fe1d39dae29a984ad3d3e0434",
"026bd435cac45594bc7160ee173f3335407c4122e756092108d527690f95e90c3e",
"02e5391000f75188324edffe72d07e48b6d88f7132ee0c5b9978a8b133dc38ca7f",
"03151b15446bd67375539e9c620f42bb9947069b00a1b013dc10654e72f4b62994",
"03da0f6bc16a0cf1350e230c9d6ec5664e395d518474b77e2926e25958de80130d",
"02efb3fb089647f27813d1f6db5efe6887f9f787186e874aff152a9ef9b5b62e10",
"0302563f099a5aabd6b12d4a6fb074798f03679de9c6db6e8bbaaeaf994a5b5b64",
"021334854c944015d458e25f076ba177496119075522b617b4f681f2f25b3a33a0",
"03982d78ed21277a394a2624a23c0588bd7540487092953184dee4e9d33c0e2be1",
"023fa63b66568b4fd5c52e94c09e859891ed102877a3085b788103b807673ce77f",
"02f037a7170f00c295a47684f8ef77f5fb2ede593b5076521fc3ff04b25a4de65b",
"020fd236a5ff33459b1b0e24007e53c877e3f590c217e435b74b295858c442c620",
"03e33fd567f6e8592ced901d3d76f834ce255b04db8e1382332be74276691ca507",
"02a8474a54ad7fdb4ae04655577e18c749edd11aaf3abcb8d14a7f2b01db9e05dd",
"037b64bd9fbc499600ac0fc4ff0e1ff0a5e09dd07d04c3a2e672031d0287b71c3b",
    ];

    let args: Args = Args::parse();

    if args.verify > 0 {
        verify_messages(args.clone(), args.verify.clone());
        return;
    }

    let pool = threadpool::ThreadPool::new(args.parallel_messages as usize);

    for i in 0..args.total_messages {
        let args_clone = args.clone();

        pool.execute(move || {
            let mut rng = rand::thread_rng();
            let inrange = Uniform::from(0..63);
            let randInex = inrange.sample(&mut rng);


            // let randInex = 3;
            // println!("here: {}", pubKeyArray[randInex]);

            let msg = generate_message(args_clone.clone(), i.clone(), pubKeyArray[randInex], privKeyArray[randInex]);

            let cli = mqtt::Client::new(args_clone.mqtt_url.clone()).unwrap_or_else(|err| {
                println!("Error creating the client: {:?}", err);
                std::process::exit(1);
            });

            let conn_opts = mqtt::ConnectOptionsBuilder::new()
                .keep_alive_interval(Duration::from_secs(20))
                .clean_session(true)
                .finalize();

            match cli.connect(conn_opts) {
                Err(e) => {
                    println!("Unable to connect:\n\t{:?}", e);
                }
                _ => {}
            }

            match cli.publish(msg) {
                Err(e) => println!("Error sending message: {:?}", e),
                Ok(_) => {
                    println!("{}", i);
                } // Ok(_) => {}
            }

            cli.disconnect(mqtt::DisconnectOptionsBuilder::new().finalize())
                .unwrap();
        });

        thread::sleep(Duration::from_millis(args.delay.clone() as u64));
    }

    pool.join();
}

fn verify_messages(args: Args, last_digit: u16) {
    let mut set = HashSet::<u16>::new();
    let from = AccountKeyring::Alice.pair();
    let client = WsRpcClient::new(args.substrate_url.clone().as_str());
    let api = Api::<_, _, PlainTipExtrinsicParams>::new(client)
        .map(|api| api.set_signer(from))
        .unwrap();

    let api_arc = Arc::new(api);

    for i in 0..last_digit {
        match Substrate::new(
            args.substrate_url.clone(),
            Arc::new(Mutex::new(0)),
            10,
            api_arc.clone(),
        )
        .get_data(
            String::from("Iotb"),
            hex::decode("02991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524")
                .unwrap(),
            String::from("End"),
            i.to_string(),
        ) {
            Ok(data) => {
                let encoded_data = u16::from_str(
                    String::from_utf8(
                        Msg::parse_from_bytes(data.as_slice())
                            .unwrap()
                            .get_payload()
                            .to_vec(),
                    )
                    .unwrap()
                    .as_str(),
                )
                .unwrap();
                if set.contains(&encoded_data) {
                    println!("DUPLICATE: {}", encoded_data)
                }
                set.insert(encoded_data);
            }
            Err(e) => {
                println!("[SUBSTRATE] (GET DATA) Error: {:?}", e);
            }
        };
    }

    let mut vector = Vec::from_iter(set);
    vector.sort();
    println!("Total size: {}", vector.len());
    println!("{:?}", vector);
}

fn generate_message(args: Args, i: u16, randPubkey: &str, randPrivkey: &str) -> mqtt::Message {
    // let mut public_key_tmp = hex::decode(args.public_key.clone()).unwrap();

    // let mut vecPubkeys = Vec::new();

    // let iu = i as u8;
    // public_key_tmp[32] = public_key_tmp[32] + iu;

    // vecPubkeys.push(public_key_tmp.clone());

    // let public_key_tmp_hex = hex::encode(public_key_tmp);
    // println!("{:}", public_key_tmp_hex);

    // let public_key = hex::decode(args.public_key.clone()).unwrap();

    let public_key = hex::decode(randPubkey.to_string().clone()).unwrap();

    let private_key = hex::decode(randPrivkey.to_string().clone()).unwrap();

    let mut message = Msg::new();
    message.set_action(Action::SET_DATA);
    message.set_blockchain_id(if args.blockchain == "fabric" {
        Blockchain::HYPERLEDGER_FABRIC
    } else {
        Blockchain::SUBSTRATE
    });
    message.set_smart_contract_name(args.smart_contract.clone());
    // message.set_payload(i.to_string().as_bytes().to_vec());

    message.set_payload("Message for ECDSA signing".as_bytes().to_vec());

    let mut payload = Payload::new();
    payload.set_public_key(public_key.clone());
    payload.set_message(message.clone());
    payload.set_signature(generate_signature(
        message.write_to_bytes().unwrap(),
        private_key,
        public_key,
    ));
    
    let out_bytes = payload.write_to_bytes().unwrap();

    let payload = object! {
        data: object! {
            uplink_message: object! {
                frm_payload: base64::encode(out_bytes),
            }
        }
    };

    mqtt::Message::new(args.mqtt_topic.clone(), payload.to_string(), 2)
}

fn generate_signature(message: Vec<u8>, private_key: Vec<u8>, public_key: Vec<u8>) -> Vec<u8> {
    let key = SigningKey::from_bytes(private_key.as_slice()).unwrap();
    let signature: Sign = key.sign(message.as_slice());

   


    let mut signature_vec_zero = signature.clone().as_bytes().to_vec();
    signature_vec_zero.push(0);

    let mut signature_vec_one = signature.clone().as_bytes().to_vec();
    signature_vec_one.push(1);

    let rs_zero = RecoverableSignature::from_bytes(signature_vec_zero.as_slice()).unwrap();

    let messageStr = "Message for ECDSA signing";


    // let mut buf;
    // let secp = Secp256k1::preallocated_new(buf).unwrap();
     
    let secp = Secp256k1::new();

    let mut privkeyBytes = [0x0,0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0, 0x0,0x0,0x0,0x0,0x0,0x0];
   
   
    privkeyBytes[0] = private_key[0];
    privkeyBytes[1] = private_key[1];
    privkeyBytes[2] = private_key[2];
    privkeyBytes[3] = private_key[3];
    privkeyBytes[4] = private_key[4];
    privkeyBytes[5] = private_key[5];
    privkeyBytes[6] = private_key[6];
    privkeyBytes[7] = private_key[7];
    privkeyBytes[8] = private_key[8];
    privkeyBytes[9] = private_key[9];
    privkeyBytes[10] = private_key[10];
    privkeyBytes[11] = private_key[11];
    privkeyBytes[12] = private_key[12];
    privkeyBytes[13] = private_key[13];
    privkeyBytes[14] = private_key[14];
    privkeyBytes[15] = private_key[15];
    privkeyBytes[16] = private_key[16];
    privkeyBytes[17] = private_key[17];
    privkeyBytes[18] = private_key[18];
    privkeyBytes[19] = private_key[19];
    privkeyBytes[20] = private_key[20];
    privkeyBytes[21] = private_key[21];
    privkeyBytes[22] = private_key[22];
    privkeyBytes[23] = private_key[23];
    privkeyBytes[24] = private_key[24];
    privkeyBytes[25] = private_key[25];
    privkeyBytes[26] = private_key[26];
    privkeyBytes[27] = private_key[27];
    privkeyBytes[28] = private_key[28];
    privkeyBytes[29] = private_key[29];
    privkeyBytes[30] = private_key[30];
    privkeyBytes[31] = private_key[31];

    // println!("Message {}", hex::encode(message.clone()));

    // println!("private key btc {}", hex::encode(privkeyBytes.clone()));
    // println!("public key {}", hex::encode(public_key.clone()));

    // let message_u8 = messageStr.as_bytes();
    let message_u8 = &message[..];

   
    // let signature = sign_recovery(&secp, messageStr.as_bytes(), privkeyBytes).unwrap();
    let signature = sign_recovery(&secp, message_u8 , privkeyBytes).unwrap();


    let (recovery_id, serialize_sig) = signature.serialize_compact();
    
    
    // println!("Signature Serialized  {}",  hex::encode(serialize_sig.clone()));
    
    let recovery_id32 = recovery_id.to_i32();
    let mut bs = recovery_id32.to_ne_bytes();
    
    // let res = recover(&secp, messageStr.as_bytes(), serialize_sig.clone(), bs[3]).unwrap();

    // let res = recover(&secp, messageStr.as_bytes(), serialize_sig.clone(), bs[0]);
    let res = recover(&secp, message_u8, serialize_sig.clone(), bs[0]);

    match res {
        Ok(res) => {
            // println!("recovered Public key  {}",  hex::encode(res.serialize()));
        
        },
        Err(e) => {
            println!("Error while recovery {:?}\n", e);
        }
        
    }
   
    // println!("recovered Public key  {}",  hex::encode(res.serialize()));

    

    let mut final_signature = serialize_sig.to_vec();
    final_signature.push(bs[0]);

    // println!("Final Signature Serialized  {}",  hex::encode(final_signature.clone()));



    // let signaturetmp = sign_recovery(&secp, msg, private_key.as_slice()).unwrap();
   
    let hasher = Sha256::new().chain(message).finalize();
    
    ////////////////////////////////////////////////////////////////////////////
    // OLD Signature process
    ////////////////////////////////////////////////////////////////////////////
    // let hasher = Sha256::new().chain(messageStr.as_bytes()).finalize();

    // println!("Hash old {}", hex::encode(hasher.clone()));
    
    // println!("private key {}", hex::encode(private_key.clone()));
    // println!("public key {}", hex::encode(public_key.clone()));
    // println!("Signature old {}",  hex::encode(signature_vec_zero.clone()));

    match rs_zero.recover_verify_key_from_digest_bytes(&hasher.clone()) {
        Ok(v) => {
            if v.to_bytes().to_vec() == public_key {
                // signature_vec_zero
                // println!("Signature old pubkey {}", hex::encode(v.to_bytes().to_vec()));

            } else {
                // println!("Signature old pubkey not equal {}", hex::encode(v.to_bytes().to_vec()));

                // signature_vec_one
            }
        },
        Err(_) => {
                 println!("Error");
        
        }
        // Err(_) => signature_vec_one,
    }
    return final_signature;
    
}


fn sign_recovery<C: Signing>(secp: &Secp256k1<C>, msg: &[u8], seckey: [u8; 32]) -> Result<ecdsa::RecoverableSignature, Error> {
    // println!("Message in sign_recovery{}", hex::encode(msg.clone()));
    
    let msg = sha256::Hash::hash(msg);

    // println!("Hash {}", hex::encode(msg.clone()));


    let msg = MsgEcdsa::from_slice(&msg)?;


    // let msg = MsgEcdsa::from_slice(&msg)?;

    let seckey = SecretKey::from_slice(&seckey)?;
    Ok(secp.sign_ecdsa_recoverable(&msg, &seckey))
}

fn recover<C: Verification>(secp: &Secp256k1<C>,msg: &[u8],sig: [u8; 64],recovery_id: u8) -> Result<PublicKey, Error> {
    let msg = sha256::Hash::hash(msg);


    let msg = MsgEcdsa::from_slice(&msg)?;
    let id = ecdsa::RecoveryId::from_i32(recovery_id as i32)?;
    let sig = ecdsa::RecoverableSignature::from_compact(&sig, id)?;

    secp.recover_ecdsa(&msg, &sig)
}


// private key btc 6319f9847e783c25be8e870062200fa8d73426a7f40456f3204bae52e2c41888
// public key 0266995b22143c71f538eaf202afa355be8e4571f15352624756a8f5e69b2d237f
// Signature Serialized  5a1373978e7eb7449f0d25a90e6a3794700af4e03a7b24a3a2c7b2107923ca977a4a353d6a61eca934a2bfa9c03c8cdb58ed3119cc0c14781e038295135bad66
// recovered Public key  0266995b22143c71f538eaf202afa355be8e4571f15352624756a8f5e69b2d237f
// Final Signature Serialized  5a1373978e7eb7449f0d25a90e6a3794700af4e03a7b24a3a2c7b2107923ca977a4a353d6a61eca934a2bfa9c03c8cdb58ed3119cc0c14781e038295135bad6600





// recovered 020ec782ee853b0b151b3eeba353edb485bcb0ddd8f4463ce75d35636d45fc75d4 
// expected: 0266995b22143c71f538eaf202afa355be8e4571f15352624756a8f5e69b2d237f


// 5a1373978e7eb7449f0d25a90e6a3794700af4e03a7b24a3a2c7b2107923ca977a4a353d6a61eca934a2bfa9c03c8cdb58ed3119cc0c14781e038295135bad66
// 5a1373978e7eb7449f0d25a90e6a3794700af4e03a7b24a3a2c7b2107923ca977a4a353d6a61eca934a2bfa9c03c8cdb58ed3119cc0c14781e038295135bad66
// 5a1373978e7eb7449f0d25a90e6a3794700af4e03a7b24a3a2c7b2107923ca977a4a353d6a61eca934a2bfa9c03c8cdb58ed3119cc0c14781e038295135bad66
// 5a1373978e7eb7449f0d25a90e6a3794700af4e03a7b24a3a2c7b2107923ca977a4a353d6a61eca934a2bfa9c03c8cdb58ed3119cc0c14781e038295135bad66
//   5a1373978e7eb7449f0d25a90e6a3794700af4e03a7b24a3a2c7b2107923ca977a4a353d6a61eca934a2bfa9c03c8cdb58ed3119cc0c14781e038295135bad66

