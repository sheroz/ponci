use log;
use log4rs;
use poncu::client::core::{PoncuTcpClient, TcpClient};
use std::net::{IpAddr, Ipv4Addr};

fn main() {
    
    log4rs::init_file("log.yaml", Default::default()).unwrap();

    log::info!("{} client v{} ", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let config = poncu::utils::config::read_config();
    println!("{:#?}", config);

    let remote_nodes = poncu::utils::config::get_remote_nodes(&config);
    let remote_address = remote_nodes[0];
    let mut client = PoncuTcpClient::with_socket(&remote_address);
    client.connect().expect("client connection error");

    let msg = String::from("Hi there!");
    client.set_item(msg).expect("set item error");
}
