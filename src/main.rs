use tungstenite::{connect, Message};
use clap::Parser;
use serde_json::json;

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

fn main() {
    env_logger::init();

	let args = Args::parse();

	let thread_data_tlv: String = args.thread_data_tlv;
	let matter_commission_code: String = args.matter_commission_code;
	let websocket_url: String = args.websocket_url;

	println!("Starting up");
	println!("Thread TLV: {}", thread_data_tlv);
	println!("Matter Commission Code: {}", matter_commission_code);
	println!("Home Assistant Matter WebSocket URL: {}", websocket_url);

    let (mut socket, response) =
        connect(websocket_url).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());

    let msg_thread_credentials = json!({
	    "message_id": "1",
	    "command": "set_thread_dataset",
	    "args": { 
	      "dataset": thread_data_tlv
	    }
	});

   let msg_commission_with_code = json!({
	    "message_id": "2",
	    "command": "commission_with_code",
	    "args": {
	      "code": matter_commission_code
	    }
	});

    let msg = socket.read().expect("Error reading message");
    println!("Received: {}", msg);

	println!();

	println!("Sending Thread Credentials to the WebSocket Endpoint of the Matter Server. {}", msg_thread_credentials);
    socket.send(Message::Text(msg_thread_credentials.to_string().into())).unwrap();
    let msg = socket.read().expect("Error reading message");
    println!("Received: {}", msg);

	println!();

    println!("Sending Commission with Code command to the WebSocket Endpoint of the Matter Server. {}", msg_commission_with_code);
    socket.send(Message::Text(msg_commission_with_code.to_string().into())).unwrap();
    let msg = socket.read().expect("Error reading message");
    println!("Received: {}", msg);
    socket.close(None).ok();
}
