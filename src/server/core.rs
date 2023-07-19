use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, TcpListener};

pub struct StorageServer {
    storage: HashMap<String, StorageItem>,
}

struct StorageItem {
    item_type: ItemType,
    item_data: Box<Vec<u8>>,
    item_metadata: String
}

pub enum ItemType {
    String,
    Boolean,
    SignedInteger(u8),
    UnsignedInteger(u8),
    Float(u8),
    Blob,
    Json,
    Xml,
    File
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
