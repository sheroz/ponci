use log::{trace, debug, info, warn, error, log_enabled, Level};
use log4rs;
use poncu::server::core::{PoncuTcpServer, TcpServer};
use poncu::client::core::{PoncuTcpClient, TcpClient};
use core::time;
use std::net::{IpAddr, Ipv4Addr};
use std::thread;

fn main() {
    println!("Poncu!");
    
    log4rs::init_file("log.yaml", Default::default()).unwrap();

    if log_enabled!(Level::Trace) {
        trace!("log message in {} level", "TRACE");
    }

    if log_enabled!(Level::Debug) {
        debug!("log message in {} level", "DEBUG");
    }

    info!("log message in {} level", "INFO");
    warn!("log message in {} level", "WARN");
    error!("log message in {} level", "ERROR");

    let ip_address = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port = 9191_u16;
    let server = PoncuTcpServer::new(ip_address, port);
    let server_handle = thread::spawn(move || {
        server.start();
    });

    thread::sleep(time::Duration::from_secs(5));
    
    let mut client = PoncuTcpClient::new(ip_address, port);
    client.connect();

    server_handle.join();

}
