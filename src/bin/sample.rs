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
    let server_running = Arc::new(AtomicBool::new(true));

    let ip_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port = 9191_u16;
    let server = PoncuTcpServer::new(ip_address, port);
    let server_running_thread = server_running.clone();
    let server_ready_thread = server_ready.clone();
   
    let server_handle = thread::spawn(move || {
        server.start(server_running_thread, server_ready_thread);
    });

    while !server_ready.load(Ordering::SeqCst) {
        if log_enabled!(Level::Trace) {
            log::trace!("server not ready yet, wait...");
        }
        thread::sleep(time::Duration::from_millis(20));
    }
    
    let mut client = PoncuTcpClient::new(ip_address, port);
    let _ = client.connect();

    thread::sleep(time::Duration::from_secs(3));

    // shutdown the server
    server_running.store(false, Ordering::SeqCst);
    let _ = server_handle.join();
}
