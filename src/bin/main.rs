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
    
    let server_ready = Arc::new(AtomicBool::new(false));
    let server_shutdown = Arc::new(AtomicBool::new(false));

    let ip_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port = 9191_u16;
    let server_signal_shutdown = server_shutdown.clone();
    let server_get_ready = server_ready.clone();
   
    let server = PoncuTcpServer::new(ip_address, port);
    let server_handle = server.start(server_signal_shutdown, server_get_ready);

    while !server_ready.load(Ordering::SeqCst) {
        if log_enabled!(Level::Trace) {
            log::trace!("server not ready yet, wait...");
        }
        thread::sleep(time::Duration::from_millis(20));
    }
    
    // thread::sleep(time::Duration::from_secs(3));

    let mut client = PoncuTcpClient::new(ip_address, port);
    client.connect().expect("client connection error");

    let msg = String::from("Hi there!");
    client.set_item(msg).expect("set item error");


    // shutdown the server
    // server_shutdown.store(false, Ordering::SeqCst);
    let _ = server_handle.join();
    log::info!("server closed.");
}
