extern crate iotb_gateway;
extern crate paho_mqtt as mqtt;

use clap::Parser;
use json::object;
use k256::ecdsa::{Signature as Sign, SigningKey};
use k256::ecdsa::recoverable::Signature as RecoverableSignature;
use k256::ecdsa::signature::{Signature, Signer};
use protobuf::Message;
use sha2::{Digest, digest::Update, Sha256};

use iotb_gateway::payload::payload::{Action, Blockchain, Payload};
use iotb_gateway::payload::payload::Message as Msg;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Private key for signing
    #[clap(long, value_parser, default_value = "ec3c161597b119998ad8822acb2b1123405e857577e5e8abd678ad8c2383f20c")]
    private_key: String,

    /// Corresponding public key
    #[clap(long, value_parser, default_value = "02991d1d22f24d0044adea280cfbc8c8c24998c4b4074078409e33491530a89524")]
    public_key: String,

    /// Flag for getting data from blockchain. Pass index as --payload
    #[clap(long, action, default_value_t = false)]
    get_data: bool,

    /// Payload to send
    #[clap(long, value_parser, default_value = "0")]
    payload: String,

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
}

fn main() {
    let args: Args = Args::parse();

    let public_key = hex::decode(args.public_key.clone()).unwrap();
    let private_key = hex::decode(args.private_key.clone()).unwrap();

    let mut message = Msg::new();
    message.set_action(if args.get_data { Action::GET_DATA } else { Action::SET_DATA });
    message.set_blockchain_id(if args.blockchain == "fabric" { Blockchain::HYPERLEDGER_FABRIC } else { Blockchain::SUBSTRATE });
    message.set_smart_contract_name(args.smart_contract.clone());
    message.set_payload(args.payload.clone().into_bytes());

    let mut payload = Payload::new();
    payload.set_public_key(public_key.clone());
    payload.set_message(message.clone());
    payload.set_signature(generate_signature(message.write_to_bytes().unwrap(), private_key, public_key));

    let out_bytes = payload.write_to_bytes().unwrap();

    let cli = mqtt::Client::new(args.mqtt_url.clone()).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        std::process::exit(1);
    });

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(std::time::Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    match cli.connect(conn_opts) {
        Err(e) => {
            println!("Unable to connect:\n\t{:?}", e);
            std::process::exit(1);
        }
        _ => {}
    }

    let payload = object! {
        data: object! {
            uplink_message: object! {
                frm_payload: base64::encode(out_bytes),
            }
        }
    };

    let msg = mqtt::Message::new(args.mqtt_topic.clone(), payload.to_string(), 2);
    match cli.publish(msg) {
        Err(e) => println!("Error sending message: {:?}", e),
        _ => {}
    }

    cli.disconnect(mqtt::DisconnectOptionsBuilder::new().finalize()).unwrap();
}

fn generate_signature(message: Vec<u8>, private_key: Vec<u8>, public_key: Vec<u8>) -> Vec<u8> {
    let key = SigningKey::from_bytes(private_key.as_slice()).unwrap();
    let signature: Sign = key.sign(message.as_slice());

    let mut signature_vec_zero = signature.clone().as_bytes().to_vec();
    signature_vec_zero.push(0);

    let mut signature_vec_one = signature.clone().as_bytes().to_vec();
    signature_vec_one.push(1);

    let rs_zero = RecoverableSignature::from_bytes(signature_vec_zero.as_slice()).unwrap();
    let hasher = Sha256::new().chain(message).finalize();

    match rs_zero.recover_verify_key_from_digest_bytes(&hasher.clone()) {
        Ok(v) => { if v.to_bytes().to_vec() == public_key { signature_vec_zero } else { signature_vec_one } }
        Err(_) => { signature_vec_one }
    }
}