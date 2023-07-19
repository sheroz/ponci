use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, TcpListener};

pub struct StorageServer {
    storage: HashMap<String, StorageItem>,
}

struct StorageItem {
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

impl StorageServer {
    pub fn new() -> Self {
        StorageServer {
            storage: HashMap::new(),
        }
    }

    pub fn start(port: u16, ip_address: IpAddr) {
        let socket_address = SocketAddr::new(ip_address, port);
        let listener = TcpListener::bind(socket_address).unwrap();
        loop {
            match listener.accept() {
                Ok((_stream, addr)) => println!("new client: {addr:?}"),
                Err(e) => println!("couldn't get client: {e:?}"),
            }
        }
    }
}
