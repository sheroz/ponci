use core::time;
use log::{log_enabled, Level};
use poncu::client::core::{PoncuTcpClient, TcpClient};
use poncu::client::file_client;
use poncu::server::core::{PoncuMutex, PoncuTcpServer, TcpServer};
use poncu::utils::config;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    log4rs::init_file("log.yaml", Default::default()).unwrap();

    log::info!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let config = config::get_config();

    let flag_tcp_server_ready = Arc::new(AtomicBool::new(false));
    let flag_tcp_server_shutdown = Arc::new(AtomicBool::new(false));

    let flag_tcp_server_ready_worker = flag_tcp_server_ready.clone();
    let flag_tcp_server_shutdown_worker = flag_tcp_server_shutdown.clone();

    let server_config = config.clone();
    let handle_tcp_server = thread::spawn(move || {
        let server = PoncuTcpServer::with_config(&server_config);
        let _poncu_mutex: PoncuMutex = Arc::new(Mutex::new(&server));
        server.start(
            &flag_tcp_server_shutdown_worker,
            &flag_tcp_server_ready_worker,
        );
    });

    while !flag_tcp_server_ready.load(Ordering::SeqCst) {
        if log_enabled!(Level::Trace) {
            log::trace!("socket server not ready yet, wait...");
        }
        thread::sleep(time::Duration::from_millis(20));
    }

    let client_config = config.clone();
    let mut client1 = PoncuTcpClient::with_config(&client_config);
    client1.connect().expect("client connection error");

    let msg1 = String::from("Hi there1!");
    client1.set_item(msg1).expect("set item error");
    thread::sleep(Duration::from_millis(20));
    let msg2 = String::from("Hi there2!");
    client1.set_item(msg2).expect("set item error");

    let mut client2 = PoncuTcpClient::with_config(&client_config);
    client2.connect().expect("client connection error");

    let msg1 = String::from("Hi there1!");
    client2.set_item(msg1).expect("set item error");
    thread::sleep(Duration::from_millis(20));
    let msg2 = String::from("Hi there2!");
    client2.set_item(msg2).expect("set item error");

    // shutdown the server
    // server_shutdown.store(false, Ordering::SeqCst);

    // start file server
    let file_server_config = config.clone();
    let flag_file_server_ready = Arc::new(AtomicBool::new(false));
    let flag_file_server_shutdown = Arc::new(AtomicBool::new(false));

    let handle_file_server = poncu::server::file_server::start_file_server(
        &file_server_config,
        flag_file_server_ready.clone(),
        flag_file_server_shutdown.clone(),
    );

    while !flag_file_server_ready.load(Ordering::SeqCst) {
        if log_enabled!(Level::Trace) {
            log::trace!("File server not ready yet, wait...");
        }
        thread::sleep(time::Duration::from_millis(20));
    }

    file_client::get_file_info("http://127.0.0.1:8181/LICENSE");
    file_client::get_file("http://127.0.0.1:8181/LICENSE");

    let range = 48..482;
    file_client::get_file_in_range("http://127.0.0.1:8181/LICENSE", Some(range));

    let range = 2000..2100;
    file_client::get_file_in_range("http://127.0.0.1:8181/LICENSE", Some(range));

    let _ = handle_file_server.join();
    let _ = handle_tcp_server.join();
    log::info!("server closed.");
}
