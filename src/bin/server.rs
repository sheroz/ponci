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

    log::info!("{} server v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let config = poncu::utils::config::read_config();
    println!("{:#?}", config);

    let node_socket_addresses = poncu::utils::config::get_node_socket_addresses(&config);
    let node_socket = node_socket_addresses[0];
    let server = PoncuTcpServer::with_socket(&node_socket);

    let server_ready = Arc::new(AtomicBool::new(false));
    let server_shutdown = Arc::new(AtomicBool::new(false));
    let server_signal_shutdown = server_shutdown.clone();
    let server_get_ready = server_ready.clone();
    let _server_handle = server.start(server_signal_shutdown, server_get_ready);

    while !server_ready.load(Ordering::SeqCst) {
        if log_enabled!(Level::Trace) {
            log::trace!("server not ready yet, wait...");
        }
        thread::sleep(time::Duration::from_millis(20));
    }

    log::trace!("server ready.");

}
