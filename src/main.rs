use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::time::{Duration};
use chrono::{Utc};

use clap::Parser;
use protobuf::Message;
use rumqttc::{Client, Event, Incoming, MqttOptions, QoS};
use sp_keyring::AccountKeyring;
use substrate_api_client::{Api, PlainTipExtrinsicParams};
use substrate_api_client::rpc::WsRpcClient;
use threadpool::ThreadPool;

use iotb_gateway::blockchains::Blockchain as BC;
use iotb_gateway::blockchains::hyperledger_fabric::HyperLedgerFabric;
use iotb_gateway::blockchains::substrate::Substrate;
use iotb_gateway::blockchains::utils::substrate::add_new_listener;
use iotb_gateway::payload::payload::{Action, Blockchain, Payload};
use iotb_gateway::payload::payload::Message as Msg;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Hyperledger Fabric CCP path
    #[clap(long, value_parser, default_value = "")]
    fabric_ccp_path: String,

    /// Hyperledger Fabric CRED path
    #[clap(long, value_parser, default_value = "")]
    fabric_cred_path: String,

    /// Substrate URL
    #[clap(long, value_parser, default_value = "")]
    substrate_url: String,

    /// MQTT broker host
    #[clap(long, value_parser, default_value = "localhost")]
    mqtt_host: String,

    /// MQTT broker port
    #[clap(long, value_parser, default_value_t = 1883)]
    mqtt_port: u16,

    /// MQTT topic
    #[clap(long, value_parser, default_value = "#")]
    mqtt_topic: String,

    /// Flag for initializing Hyperledger Fabric blockchain
    #[clap(long, action, default_value_t = false)]
    fabric_init: bool,

    /// Flag for initializing Substrate blockchain
    #[clap(long, action, default_value_t = false)]
    substrate_init: bool,

    /// Number of workers
    #[clap(short, long, value_parser, default_value_t = 10)]
    number_of_workers: usize,

    /// Substrate timeout
    #[clap(long, value_parser, default_value_t = 7)]
    substrate_timeout: u64,
}

fn main() {
    let args: Args = Args::parse();

    let mut mqtt_options = MqttOptions::new("rumqtt-sync", args.mqtt_host.clone(), args.mqtt_port.clone());
    mqtt_options.set_keep_alive(Duration::MAX);

    let (mut client, mut connection) = Client::new(mqtt_options, 10);
    client.subscribe(args.mqtt_topic.clone(), QoS::AtMostOnce).unwrap();

    let pool = ThreadPool::new(args.number_of_workers);

    let from = AccountKeyring::Alice.pair();
    let client = WsRpcClient::new(args.substrate_url.clone().as_str());
    let api = Api::<_, _, PlainTipExtrinsicParams>::new(client)
        .map(|api| api.set_signer(from))
        .unwrap();
    let nonce = Arc::new(Mutex::new(api.get_nonce().unwrap()));

    let events = Arc::new(Mutex::new(HashSet::<(String, String)>::new()));

    let api_arc = Arc::new(api);

    if args.substrate_init {
        add_new_listener("Iotb".into(), String::from("Initialized"), args.substrate_url.clone(), events.clone());
        Substrate::new(args.substrate_url.clone(), nonce.clone(), args.substrate_timeout, api_arc.clone()).init("Iotb".into());
    }

    if args.fabric_init {
        HyperLedgerFabric::new(args.fabric_ccp_path.clone(), args.fabric_cred_path.clone()).init("Iotb".into());
    }

    let (sender, receiver) = channel::<Payload>();

    let receiver = Arc::new(Mutex::new(receiver));

    // let counter_general = Mutex::new(0); //is mutex counter
    let mut counter_general: usize = 0; //is mutex counter
    let mut table_start_timers:[i64; 100000] = [0; 100000];
    let mut table_stop_timers:[i64; 100000] = [0; 100000];

    for (_, notification) in connection.iter().enumerate() {
        match notification.unwrap() {
            Event::Incoming(Incoming::Publish(p)) => {
                //lock mutex
                // let counter_general_m = counter_general.lock().unwrap();
                // let current_counter = counter_general_m.;
                // *counter_general_m+=1;
                let current_counter = counter_general;
                counter_general = counter_general + 1;
                //free mutex
                table_start_timers[current_counter.clone()] = Utc::now().timestamp_millis();
                
                let parsed = json::parse(String::from_utf8(p.payload.clone().to_vec()).unwrap().as_str()).unwrap();
                let decoded = base64::decode(&parsed["data"]["uplink_message"]["frm_payload"].as_str().unwrap()).unwrap();
                let payload: Payload = Message::parse_from_bytes(decoded.as_slice()).unwrap();

                let nonce_clone = nonce.clone();
                let args_clone = args.clone();
                let api_clone = api_arc.clone();
                sender.send(payload.clone()).unwrap();
                let receiver_clone = receiver.clone();

                pool.execute(move || {
                    let payload = receiver_clone.lock().unwrap().recv().unwrap();
                    match payload.get_message().get_blockchain_id() {
                        Blockchain::HYPERLEDGER_FABRIC => match payload.get_message().get_action() {
                            Action::SET_DATA => {
                                match HyperLedgerFabric::new(
                                    args_clone.fabric_ccp_path.clone(),
                                    args_clone.fabric_cred_path.clone())
                                    .set_data(
                                        payload.get_message().get_smart_contract_name().into(),
                                        payload.get_signature().into(),
                                        payload.get_message().write_to_bytes().unwrap(),
                                        payload.get_public_key().into(),
                                        "End".into()) {
                                    Ok(_) => { 
                                        table_stop_timers[current_counter.clone()] = Utc::now().timestamp_millis();
                                        println!("[FABRIC] (SET DATA) Success:{}:{:?}", current_counter.clone(), table_stop_timers[current_counter.clone()]-table_start_timers[current_counter.clone()]);
                                    }
                                    Err(e) => { 
                                        table_stop_timers[current_counter.clone()] = Utc::now().timestamp_millis();
                                        println!("[FABRIC] (SET DATA) Error: {:?}, \n Millis:{}:{:?}", e, current_counter.clone(), table_stop_timers[current_counter.clone()]-table_start_timers[current_counter.clone()]);
                                    }
                                }
                            }
                            Action::GET_DATA => {
                                match HyperLedgerFabric::new(
                                    args_clone.fabric_ccp_path.clone(),
                                    args_clone.fabric_cred_path.clone())
                                    .get_data(
                                        payload.get_message().get_smart_contract_name().into(),
                                        payload.get_public_key().into(),
                                        String::from("End"),
                                        String::from_utf8(Vec::from(payload.get_message().get_payload())).unwrap()) {
                                    Ok(data) => { 
                                        println!("[FABRIC] (GET DATA) Index: {:?}", Msg::parse_from_bytes(data.as_slice()).unwrap().get_payload());
                                    }
                                    Err(e) => { 
                                        println!("[FABRIC] (GET DATA) Error: {:?}", e);
                                    }
                                }
                            }
                        },
                        Blockchain::SUBSTRATE => match payload.get_message().get_action() {
                            Action::SET_DATA => {
                                match Substrate::new(args_clone.substrate_url.clone(), nonce_clone.clone(), args_clone.substrate_timeout.clone(), api_clone.clone())
                                    .set_data(
                                        payload.get_message().get_smart_contract_name().into(),
                                        payload.get_signature().into(),
                                        payload.get_message().write_to_bytes().unwrap(),
                                        payload.get_public_key().into(),
                                        "End".into()) {
                                    Ok(_) => { 
                                        table_stop_timers[current_counter.clone()] = Utc::now().timestamp_millis();
                                        println!("[SUBSTRATE] (SET DATA) Success:{}:{:?}", current_counter.clone(), table_stop_timers[current_counter.clone()]-table_start_timers[current_counter.clone()]);
                                    }
                                    Err(e) => { 
                                        table_stop_timers[current_counter.clone()] = Utc::now().timestamp_millis();
                                        println!("[SUBSTRATE] (SET DATA) Error: {:?}, \n Millis:{}:{:?}", e, current_counter.clone(), table_stop_timers[current_counter.clone()]-table_start_timers[current_counter.clone()]);
                                    }
                                }
                            }
                            Action::GET_DATA => {
                                match Substrate::new(args_clone.substrate_url.clone(), nonce_clone.clone(), args_clone.substrate_timeout.clone(), api_clone.clone())
                                    .get_data(
                                        payload.get_message().get_smart_contract_name().into(),
                                        payload.get_public_key().into(),
                                        String::from("End"),
                                        String::from_utf8(Vec::from(payload.get_message().get_payload())).unwrap()) {
                                    Ok(data) => { 
                                        println!("[SUBSTRATE] (GET DATA) Success: {:?}", Msg::parse_from_bytes(data.as_slice()).unwrap().get_payload().to_vec());
                                    }
                                    Err(e) => { 
                                        println!("[SUBSTRATE] (GET DATA) Error: {:?}", e);
                                    }
                                }
                            }
                        },
                        Blockchain::HYPERLEDGER_SAWTOOTH => todo!(),
                        Blockchain::ETHEREUM => todo!()
                    }
                });
            }
            _ => {}
        }
    }
}