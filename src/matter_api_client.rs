use std::net::TcpStream;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};

use crate::messages::{
    Code, Commands, DatasetTLV, DatasetWifiCreds, MessageID, MessageWithArgs, MessageWithoutArgs,
    NodeId,
};

pub struct MatterApiClient {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl MatterApiClient {
    pub fn new(websocket_url: String) -> MatterApiClient {
        let (mut socket, _response) = connect(websocket_url).expect("Can't connect");
        let msg = &socket.read().expect("Error reading response");
        println!("Connected To Service \n Server-Details: {} \n", msg);
        MatterApiClient { socket }
    }

    pub fn send_wifi_creds(&mut self, ssid: String, credentials: String) {
        let msg_dataset = DatasetWifiCreds {
            ssid: ssid.to_owned(),
            credentials: credentials.to_owned(),
        };

        let msg_wifi_credentials = MessageWithArgs {
            message_id: MessageID::One,
            command: Commands::SetWifiCredentials,
            args: msg_dataset,
        };

        let msg_wifi_credentials = serde_json::to_string(&msg_wifi_credentials).unwrap();

        println!("Sending Wifi Credentials to the WebSocket Endpoint of the Matter Server.");
        self.socket
            .send(Message::Text(msg_wifi_credentials.to_string().into()))
            .unwrap();

        let msg = self.socket.read().expect("Error reading message");
        println!("Received: {}", msg);
    }

    pub fn send_thread_tlv(&mut self, thread_dataset_tlv: String) {
        let msg_dataset = DatasetTLV {
            dataset: thread_dataset_tlv.to_owned(),
        };

        let msg_thread_credentials = MessageWithArgs {
            message_id: MessageID::One,
            command: Commands::SetThreadDataset,
            args: msg_dataset,
        };

        let msg_thread_credentials = serde_json::to_string(&msg_thread_credentials).unwrap();

        println!("Sending Thread Credentials to the WebSocket Endpoint of the Matter Server.");
        self.socket
            .send(Message::Text(msg_thread_credentials.to_string().into()))
            .unwrap();
        let msg = self.socket.read().expect("Error reading message");
        println!("Received: {}", msg);
    }

    pub fn send_commission_with_code(&mut self, code: String) {
        let msg_message_commission_code = MessageWithArgs {
            message_id: MessageID::Two,
            command: Commands::CommissionWithCode,
            args: Code { code },
        };

        let msg_commission_with_code = serde_json::to_string(&msg_message_commission_code).unwrap();

        println!("Sending Commission with Code command");
        self.socket
            .send(Message::Text(msg_commission_with_code.to_string().into()))
            .unwrap();
        let msg = self.socket.read().expect("Error reading message");
        println!("Received: {}", msg);
    }

    pub fn send_open_commission_window(&mut self, node_id: String) {
        let msg_open_commission_window = MessageWithArgs {
            message_id: MessageID::Two,
            command: Commands::OpenCommissioningWindow,
            args: NodeId { node_id },
        };

        let msg_open_commission_window_object =
            serde_json::to_string(&msg_open_commission_window).unwrap();

        println!("Sending Open Commission Window");
        self.socket
            .send(Message::Text(
                msg_open_commission_window_object.to_string().into(),
            ))
            .unwrap();
        let msg = self.socket.read().expect("Error reading message");
        println!("Received: {}", msg);
    }

    pub fn send_get_nodes(&mut self) {
        let msg_without_args = MessageWithoutArgs {
            message_id: MessageID::Two,
            command: Commands::GetNodes,
        };

        self.send_and_read(msg_without_args);
    }

    pub fn send_get_node(&mut self, node_id: String) {
        let node_id = NodeId { node_id };

        let msg_get_node = MessageWithArgs {
            message_id: MessageID::Two,
            command: Commands::GetNode,
            args: node_id,
        };

        let msg_get_node_object = serde_json::to_string(&msg_get_node).unwrap();

        println!("Sending get Node Object");
        self.socket
            .send(Message::Text(msg_get_node_object.to_string().into()))
            .unwrap();
        let msg = self.socket.read().expect("Error reading message");
        println!("Received: {}", msg);
    }

    pub fn test(&mut self) {
        let test_message = MessageWithoutArgs {
            message_id: MessageID::One, //MessageID::One,
            command: Commands::SetWifiCredentials,
        };

        self.send_and_read(test_message);
    }

    pub fn close(&mut self) {
        self.socket.close(None).ok();
    }

    pub fn send_and_read(&mut self, message: MessageWithoutArgs) {
        let msg = serde_json::to_string(&message).unwrap();
        println!("Outgoing: \n{}", msg);
        let _ = self.socket.send(Message::text(msg));
        let response = self.socket.read().expect("Could not read response");
        println!("Incoming: \n{}", response);
    }
}
