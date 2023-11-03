use serde::{Deserialize, Serialize};

/// All available commands taken from https://github.com/home-assistant-libs/python-matter-server.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Commands {
    SetWifiCredentials,
    SetThreadDataset,
    CommissionWithCode,
    CommissionOnNetwork,
    OpenCommissioningWindow,
    GetNodes,
    GetNode,
    StartListening,
}

#[derive(Serialize, Deserialize)]
pub enum MessageID {
    #[serde(rename = "1")]
    One = 1,

    #[serde(rename = "2")]
    Two = 2,
}

/// Message Object including Arguments like DatasetTLV or DatasetWifiCreds
#[derive(Serialize, Deserialize)]
pub struct MessageWithArgs<T> {
    pub message_id: MessageID,
    pub command: Commands,
    pub args: T,
}

/// Message Object without Additional Arguments
#[derive(Serialize, Deserialize)]
pub struct MessageWithoutArgs {
    pub message_id: MessageID,
    pub command: Commands,
}

/// Message with Optional args
#[derive(Serialize, Deserialize)]
pub struct MatterMessage<T> {
    pub message_id: MessageID,
    pub command: Commands,
    pub args: T,
}

/// Message Object without Additional Arguments
#[derive(Serialize, Deserialize)]
pub struct MessageWithoutArgsTest<T> {
    pub message_id: MessageID,
    pub command: Commands,
    pub args: Option<T>,
}

/// Contains Thread TLV Dataset
#[derive(Serialize, Deserialize)]
pub struct DatasetTLV {
    pub dataset: String,
}

/// Contains Wifi Credentials
#[derive(Serialize, Deserialize)]
pub struct DatasetWifiCreds {
    pub ssid: String,
    pub credentials: String,
}

/// Contains Commissioning Code
#[derive(Serialize, Deserialize)]
pub struct Code {
    pub code: String,
}

/// Contains NodeID
#[derive(Serialize, Deserialize)]
pub struct NodeId {
    pub node_id: String,
}

/// Contains Commissioning Code
#[derive(Serialize, Deserialize)]
pub struct SetupPinCode {
    pub setup_pin_code: String,
}
