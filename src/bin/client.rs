use log;
use log4rs;
use poncu::client::core::{PoncuTcpClient, TcpClient};
use poncu::utils::config;

fn main() {
    log4rs::init_file("log.yaml", Default::default()).unwrap();

    log::info!(
        "{} client v{} ",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let config = config::get_config();
    assert!(config.remote.is_some());
    let config_remote = config.remote.unwrap();
    assert!(!config_remote.nodes.is_empty());
    let remote_address = config_remote.nodes[0];

    let mut client = PoncuTcpClient::with_socket(&remote_address);
    client.connect().expect("client connection error");

    let msg = String::from("Hi there!");
    client.set_item(msg).expect("set item error");
}
