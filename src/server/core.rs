use std::io::{self, Read, BufReader};
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::{thread, time};
use std::sync::atomic::{AtomicBool, Ordering};
use log;

pub trait TcpServer {
    fn new(ip_address: IpAddr, port: u16) -> Self;
    fn with_socket(socket_addr: &SocketAddr) -> Self;
    fn start(&self, server_shutdown: Arc<AtomicBool>, server_ready: Arc<AtomicBool>) -> JoinHandle<()>;
    fn stop();
    fn set_item(key: String, item: StorageItem) -> bool;
    fn get_item(key: String) -> Option<StorageItem>;
    fn remove_item(key: String) -> bool;
}

pub struct PoncuTcpServer {
    storage: HashMap<String, StorageItem>,
    socket_addr: SocketAddr,
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
    redundancy: u8 // min number of required replications in the claster: 0,1,2, …
}

pub enum ItemComplexType {
    Array(ItemBasicType),
    Set(ItemBasicType),
    Map(ItemBasicType, ItemBasicType),
    Blob,
    Json,
    Xml,
    File,
    Folder,
    Path,
}

pub enum ItemBasicType {
    String,
    Boolean,
    SignedInteger(u8),
    UnsignedInteger(u8),
    Float(u8),
}

/// TBD
pub enum ItemStorageType {
    Memory, // default
    Disk,
}

impl TcpServer for PoncuTcpServer {

    fn new(ip_address:IpAddr, port:u16) -> Self {
        let socket_addr: SocketAddr = SocketAddr::new(ip_address, port);
        PoncuTcpServer::with_socket(&socket_addr)
    }

    fn with_socket(socket_addr: &SocketAddr) -> Self {
        PoncuTcpServer {socket_addr: socket_addr.clone(), storage: HashMap::new()}
    }

    fn start(&self, shutdown: Arc<AtomicBool>, ready: Arc<AtomicBool>) -> JoinHandle<()> {
        let socket_address: SocketAddr = self.socket_addr;

        let handle = thread::spawn(move || {

            let listener = TcpListener::bind(socket_address).unwrap();
            ready.store(true, Ordering::SeqCst);

            log::info!("started listening on {} ...", socket_address);
            // listener.set_nonblocking(true).unwrap();
            
            while !shutdown.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((stream, addr)) => handle_connection(stream, addr, shutdown.clone()),
                    /*
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        // wait until network socket is ready, typically implemented
                        // via platform-specific APIs such as epoll or IOCP
                        thread::sleep(time::Duration::from_millis(1));
                        continue;
                    }                
                    */
                    Err(e) => log::error!("couldn't get client: {e:?}"),
                }
            }
        });
        
        handle
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

fn handle_connection(mut stream: TcpStream, addr: SocketAddr, _shutdowm: Arc<AtomicBool>) {
    use std::io::BufRead;
    log::debug!("client connected: {}", addr);

    let mut reader = BufReader::new(stream);
    let mut msg = String::new();
    reader.read_line(&mut msg).unwrap();
    log::debug!("received message: {}", msg);
}
