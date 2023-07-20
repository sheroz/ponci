use std::io;
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, TcpListener};
use std::sync::Arc;
use std::{thread, time};
use log;
use std::sync::atomic::{AtomicBool, Ordering};

pub trait TcpServer {
    fn new(ip_address: IpAddr, port: u16) -> Self;
    fn start(&self, server_run: Arc<AtomicBool>, server_ready: Arc<AtomicBool>);
    fn stop();
    fn set_item(key: String, item: StorageItem) -> bool;
    fn get_item(key: String) -> Option<StorageItem>;
    fn remove_item(key: String) -> bool;
}

pub struct PoncuTcpServer {
    storage: HashMap<String, StorageItem>,
    ip_address: IpAddr,
    port: u16
}

pub struct StorageItem {
    item_type: ItemComplexType,
    data: Box<Vec<u8>>,
    description: String,
    tags: Vec<String>,
    metadata: HashMap<String, String>,
    may_expire: bool,
    expires_on: std::time::Instant,
    storage: Vec<ItemStorageType>,
    redundancy: u8 // min number of required replications: 0,1,2, …
}

pub enum ItemComplexType {
    Array(ItemBasicType),
    Blob,
    Json,
    Xml,
    File
}

pub enum ItemBasicType {
    String,
    Boolean,
    SignedInteger(u8),
    UnsignedInteger(u8),
    Float(u8)
}

/// TBD
pub enum ItemStorageType {
    StoreInMemory,
    DoNotStoreInMemory,
    StoreInDisk,
    DoNotStoreInDisk,
}

impl TcpServer for PoncuTcpServer {
    fn new(ip_address: IpAddr, port: u16) -> Self {
        PoncuTcpServer {
            storage: HashMap::new(),
            ip_address,
            port
        }
    }

    fn start(&self, server_run: Arc<AtomicBool>, server_ready: Arc<AtomicBool>) {
        let socket_address = SocketAddr::new(self.ip_address, self.port);
        let listener = TcpListener::bind(socket_address).unwrap();
        server_ready.store(true, Ordering::SeqCst);
        listener.set_nonblocking(true).unwrap();

        log::info!("poncu server listening {}:{} ...", self.ip_address, self.port);

        while server_run.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((_stream, addr)) => log::info!("new client: {addr:?}"),
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // wait until network socket is ready, typically implemented
                    // via platform-specific APIs such as epoll or IOCP
                    thread::sleep(time::Duration::from_millis(20));
                    continue;
                }                
                Err(e) => log::error!("couldn't get client: {e:?}"),
            }
        }
        
        log::info!("poncu server closed.");
    }

    fn stop() {

    }

    fn set_item(key: String, item: StorageItem) -> bool {
        false
    }

    fn get_item(key: String) -> Option<StorageItem> {
        None
    }

    fn remove_item(key: String) -> bool {
        true
    }

}
