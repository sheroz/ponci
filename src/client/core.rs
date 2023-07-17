use std::io::prelude::*;
use std::net::{IpAddr, SocketAddr, TcpStream};

pub fn connect(port:u16, ip_address:IpAddr) -> std::io::Result<()> {
    let socket_address = SocketAddr::new(ip_address, port);
    let mut stream = TcpStream::connect(socket_address)?;

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
}