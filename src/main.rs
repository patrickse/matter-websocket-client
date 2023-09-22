use clap::{Parser, Subcommand};

mod matter_api_client;
mod messages;

use matter_api_client::MatterApiClient;

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Set's the given wifi credentials passed as parameter.", long_about = None)]
    SetWifiCreds {
        #[arg(short, help = "Pass the ssid")]
        ssid: String,

        #[arg(short, help = "Pass the PSK")]
        psk: String,
    },

    #[command(about = "Set's the given thread credentials passed as parameter.", long_about = None)]
    SetThreadCreds {
        #[arg(short, help = "Pass the Thread Credentials TLV")]
        tlv: String,
    },

    #[command(about = "Commission a device with code", long_about = None)]
    CommissionWithCode {
        #[arg(short, help = "Matter Commission Code")]
        commission_code: String,
    },

    #[command(about = "Open a commissioning window", long_about = None)]
    OpenCommissionWindow {
        #[arg(short, help = "Open Commissioning Window for Node")]
        node_id: String,
    },

    #[command(about = "Get all nodes", long_about = None)]
    GetAllNodes {},

    #[command(about = "Get Node Info", long_about = None)]
    GetNode {
        #[arg(short, help = "Get Node Infos")]
        node_id: String,
    },

    #[command()]
    Test {},
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CommandLineArguments {
    #[arg(short, long, required = true)]
    websocket_url: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    env_logger::init();

    println!("Home-Assistant Matter Add-On CLI");
    println!("================================\n\n");

    // Parse command line arguments and evaluate if all required Args are set
    let cli = CommandLineArguments::parse();
    let websocket_url: String = cli.websocket_url;

    match &cli.command {
        Some(Commands::SetWifiCreds { ssid, psk }) => {
            let mut client = MatterApiClient::new(websocket_url);
            client.send_wifi_creds(ssid.to_string(), psk.to_string());
            client.close();
        }
        Some(Commands::SetThreadCreds { tlv }) => {
            let mut client = MatterApiClient::new(websocket_url);
            client.send_thread_tlv(tlv.to_string());
            client.close();
        }
        Some(Commands::CommissionWithCode { commission_code }) => {
            let mut client = MatterApiClient::new(websocket_url);
            client.send_commission_with_code(commission_code.to_string());
            client.close();
        }
        Some(Commands::OpenCommissionWindow { node_id }) => {
            let mut client = MatterApiClient::new(websocket_url);
            client.send_open_commission_window(node_id.to_string());
            client.close();
        }
        Some(Commands::GetAllNodes {}) => {
            let mut client = MatterApiClient::new(websocket_url);
            client.send_get_nodes();
            client.close();
        }
        Some(Commands::GetNode { node_id }) => {
            let mut client = MatterApiClient::new(websocket_url);
            client.send_get_node(node_id.to_string());
            client.close();
        }
        Some(Commands::Test {}) => {
            let mut client = MatterApiClient::new(websocket_url);
            client.test();
            client.close();
        }
        None => {}
    }
}
