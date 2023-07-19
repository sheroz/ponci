use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, TcpListener};

pub trait PoncuServer {
    fn new() -> PoncuStorage;
    fn start(port: u16, ip_address: IpAddr);
    fn stop();
    fn set_item(key: String, item: StorageItem) -> bool;
    fn get_item(key: String) -> Option<StorageItem>;
    fn remove_item(key: String) -> bool;
}

pub struct PoncuStorage {
    storage: HashMap<String, StorageItem>,
}

pub struct StorageItem {
    item_type: ItemComplexType,
    data: Box<Vec<u8>>,
    description: String,
    tags: Vec<String>,
    metadata: HashMap<String, String>,
    expirable: bool,
    expires_on: std::time::Instant,
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

impl PoncuServer for PoncuStorage {
    fn new() -> Self {
        PoncuStorage {
            storage: HashMap::new(),
        }
    }

    fn start(port: u16, ip_address: IpAddr) {
        let socket_address = SocketAddr::new(ip_address, port);
        let listener = TcpListener::bind(socket_address).unwrap();
        loop {
            match listener.accept() {
                Ok((_stream, addr)) => println!("new client: {addr:?}"),
                Err(e) => println!("couldn't get client: {e:?}"),
            }
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
