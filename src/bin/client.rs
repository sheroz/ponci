use log;
use log4rs;
use poncu::client::core::{PoncuTcpClient, TcpClient};
use std::net::{IpAddr, Ipv4Addr};

fn main() {
    
    log4rs::init_file("log.yaml", Default::default()).unwrap();

    log::info!("{} client v{} ", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        
    let ip_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port = 9191_u16;

    let mut client = PoncuTcpClient::new(ip_address, port);
    client.connect().expect("client connection error");

    let msg = String::from("Hi there!");
    client.set_item(msg).expect("set item error");
}
