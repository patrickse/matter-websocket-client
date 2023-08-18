use serde::{Deserialize, Serialize};
//use serde_json::Result;
use tungstenite::{connect, Message};
use clap::Parser;
//use serde_json::json;

#[derive(Parser, Debug)]
#[command()]
struct Args {

	#[arg(short, long, required=true)]
	thread_data_tlv: String,

	#[arg(short, long, required=true)]
	matter_commission_code: String,

	#[arg(short, long, required=true)]
	websocket_url: String
	
}

#[derive(Serialize, Deserialize)]
struct Dataset {
	dataset: String
}

#[derive(Serialize, Deserialize)]
struct Code {
	code: String
}

#[derive(Serialize, Deserialize)]
struct MessageTlv {
	message_id: String,
	command: String,
	args: Dataset
}

#[derive(Serialize, Deserialize)]
struct MessageCommissionCode {
	message_id: String,
	command: String,
	args: Code
}

fn get_msg_thread_credentials(thread_data_tlv: String) -> String {
	let msg_dataset = Dataset {
		dataset: thread_data_tlv.to_owned()
	};

	let msg_thread_credentials = MessageTlv {
		message_id: "1".to_owned(),
		command: "set_thread_dataset".to_owned(),
		args: msg_dataset
	};

	let msg_thread_credentials = serde_json::to_string(&msg_thread_credentials).unwrap();

	return msg_thread_credentials;
}

fn get_msg_commission_with_code(matter_commission_code: String) -> String {
	let msg_code = Code {
		code: matter_commission_code.to_owned()
	};

	let msg_message_commission_code = MessageCommissionCode {
		message_id: "2".to_owned(),
		command: "commission_with_code".to_owned(),
		args: msg_code
	};

	let msg_commission_with_code = serde_json::to_string(&msg_message_commission_code).unwrap();

	return msg_commission_with_code;
}

fn main() {
    env_logger::init();

	let args = Args::parse();

	let thread_data_tlv: String = args.thread_data_tlv;
	let matter_commission_code: String = args.matter_commission_code;
	let websocket_url: String = args.websocket_url;

	let msg_commission_with_code = get_msg_commission_with_code(matter_commission_code);
	let msg_thread_credentials = get_msg_thread_credentials(thread_data_tlv);

	println!("Starting up");
	println!("Thread TLV: {}", thread_data_tlv);
	println!("Matter Commission Code: {}", matter_commission_code);
	println!("Home Assistant Matter WebSocket URL: {}", websocket_url);

    let (mut socket, response) =
        connect(websocket_url).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());

    let msg = socket.read().expect("Error reading message");
    println!("Received: {}", msg);

	println!();

	println!("Sending Thread Credentials to the WebSocket Endpoint of the Matter Server.");
    socket.send(Message::Text(msg_thread_credentials.to_string().into())).unwrap();
    let msg = socket.read().expect("Error reading message");
    println!("Received: {}", msg);

	println!();

    println!("Sending Commission with Code command to the WebSocket Endpoint of the Matter Server");
    socket.send(Message::Text(msg_commission_with_code.to_string().into())).unwrap();
    let msg = socket.read().expect("Error reading message");
    println!("Received: {}", msg);
    socket.close(None).ok();
}
