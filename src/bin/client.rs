use log;
use log4rs;
use poncu::client::raw_core::{PoncuTcpClient, TcpClient};
use poncu::utils::config;

fn main() {
    log4rs::init_file("log.yaml", Default::default()).unwrap();

    log::info!(
        "{} client v{} ",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let config = config::get_config();
    let mut client = PoncuTcpClient::with_config(&config);
    client.connect().expect("client connection error");

    let msg = String::from("Hi there!");
    client.set_item(msg).expect("set item error");
}
