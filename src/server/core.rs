use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, TcpListener};

pub trait TcpServer {
    fn new(ip_address: IpAddr, port: u16) -> Self;
    fn start(&self);
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

    fn start(&self) {
        let socket_address = SocketAddr::new(self.ip_address, self.port);
        let listener = TcpListener::bind(socket_address).unwrap();
        println!("server listening on {}:{} ...", self.ip_address, self.port);

        match listener.accept() {
            Ok((_stream, addr)) => println!("new client: {addr:?}"),
            Err(e) => println!("couldn't get client: {e:?}"),
        }
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
