use log::{log_enabled, Level};
use log4rs;
use poncu::server::core::{PoncuTcpServer, TcpServer};
use poncu::client::core::{PoncuTcpClient, TcpClient};
use core::time;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
fn main() {
    
    log4rs::init_file("log.yaml", Default::default()).unwrap();

    log::info!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let config = poncu::utils::config::read_config();
    log::debug!("{:#?}", config);

    let node_socket_addresses = poncu::utils::config::get_node_socket_addresses(&config);
    let node_socket = node_socket_addresses[0];
    let server = PoncuTcpServer::with_socket(&node_socket);

    let server_ready = Arc::new(AtomicBool::new(false));
    let server_shutdown = Arc::new(AtomicBool::new(false));
    let server_signal_shutdown = server_shutdown.clone();
    let server_get_ready = server_ready.clone();
    let server_handle = server.start(server_signal_shutdown, server_get_ready);


    while !server_ready.load(Ordering::SeqCst) {
        if log_enabled!(Level::Trace) {
            log::trace!("server not ready yet, wait...");
        }
        thread::sleep(time::Duration::from_millis(20));
    }
    
    // thread::sleep(time::Duration::from_secs(3));
    let remote_nodes = poncu::utils::config::get_remote_nodes(&config);
    let remote_address = remote_nodes[0];
    let mut client = PoncuTcpClient::with_socket(&remote_address);
    client.connect().expect("client connection error");

    let msg = String::from("Hi there!");
    client.set_item(msg).expect("set item error");

    // shutdown the server
    // server_shutdown.store(false, Ordering::SeqCst);
    let _ = server_handle.join();
    log::info!("server closed.");

}
