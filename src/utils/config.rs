use std::collections::HashMap;
use std::net::{SocketAddr, IpAddr};

pub type Config = HashMap<String, HashMap<String, String>>;

pub fn read_config() -> Config {
    let config_file = std::fs::File::open("config.yaml").expect("Could not open config file.");
    let config: Config  = serde_yaml::from_reader(config_file).expect("Could not read values from config file.");
    config
}

pub fn get_node_socket_addresses(config: &Config) -> Vec<SocketAddr> {
    let listen_addresses: Vec<_> = config["node"]["listen_addresses"].split(',').map(|s| s.trim()).collect();
    log::trace!("listen_addresses: {:?}", listen_addresses);
    let port: u16 = config["node"]["listen_port"].parse().unwrap();
    log::trace!("listen_port: {}", port);

    let mut socket_addresses = Vec::<SocketAddr>::with_capacity(listen_addresses.len());
    for listen_addres in listen_addresses {
        let ip_address: IpAddr = listen_addres.parse().unwrap();
        let socket_addres = SocketAddr::new(ip_address, port);
        socket_addresses.push(socket_addres);
    }

    log::trace!("node socket addresses: {:?}", socket_addresses);
    socket_addresses
}

pub fn get_remote_nodes(config: &Config) -> Vec<SocketAddr> {
    let remote_nodes: Vec<_> = config["remote"]["nodes"].split(',').map(|s| s.trim()).collect();
    log::trace!("remote nodes: {:?}", remote_nodes);

    let mut socket_addresses = Vec::<SocketAddr>::with_capacity(remote_nodes.len());
    for node in remote_nodes {
        let socket_addr: SocketAddr = node.parse().unwrap();
        socket_addresses.push(socket_addr);
    }

    log::trace!("remote socket addresses: {:?}", socket_addresses);
    socket_addresses
}
