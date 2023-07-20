use std::io::prelude::*;
use std::net::{IpAddr, SocketAddr, TcpStream};

pub trait PoncuClient {
    fn connect(port: u16, ip_address: IpAddr) -> std::io::Result<()>;
    fn disconnect();
    fn set_item(key: String);
    fn get_item(key: String);
    fn remove_item(key: String) -> bool;
}
struct Client;

impl PoncuClient for Client {

    fn connect(port:u16, ip_address:IpAddr) -> std::io::Result<()> {
        let socket_address = SocketAddr::new(ip_address, port);
        let mut stream = TcpStream::connect(socket_address)?;
    
        stream.write(&[1])?;
        stream.read(&mut [0; 128])?;
        Ok(())
    }

    fn disconnect() {

    }

    fn set_item(key: String) {

    }

    fn get_item(key: String) {

    }

    fn remove_item(key: String) -> bool {
        true
    }


}
