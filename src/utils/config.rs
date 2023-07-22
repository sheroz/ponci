use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

#[derive(Debug)]
pub struct Config {
    pub server: Option<Server>,
    pub remote: Option<Remote>,
}

#[derive(Debug)]
pub struct Server {
    pub listen_on: Vec<SocketAddr>,
}

#[derive(Debug)]
pub struct Remote {
    pub nodes: Vec<SocketAddr>,
}

pub fn get_config() -> Arc<Config> {
    let config_file = std::fs::File::open("config.yaml").expect("Could not open config file.");
    let config_map: HashMap<String, HashMap<String, String>> =
        serde_yaml::from_reader(config_file).expect("Could not parse config file.");

    if log::log_enabled!(log::Level::Trace) {
        log::trace!("loaded config:\n{:#?}", config_map);
    }

    let mut config = Config {
        server: None,
        remote: None,
    };

    let map_key = "server";
    if config_map.contains_key(map_key) {
        let config_node = &config_map[map_key];
        let server = parse_server(config_node);
        config.server = Some(server);
    }

    let map_key = "remote";
    if config_map.contains_key(map_key) {
        let config_node = &config_map[map_key];
        let remote = parse_remote(config_node);
        config.remote = Some(remote);
    }

    if log::log_enabled!(log::Level::Trace) {
        log::trace!("parsed config:\n{:#?}", config);
    }

    Arc::new(config)
}

fn parse_server(node: &HashMap<String, String>) -> Server {
    let node_key = "listen_addresses";
    let listen_addresses: Vec<_>;
    if node.contains_key(node_key) {
        listen_addresses = node[node_key]
            .split(',')
            .map(|s| s.trim())
            .collect::<Vec<_>>();
    } else {
        listen_addresses = vec!["127.0.0.1"];
    }

    if log::log_enabled!(log::Level::Trace) {
        log::trace!("config: listen_addresses: {:?}", listen_addresses);
    }

    let node_key = "listen_port";
    let mut port: u16 = 7311;
    if node.contains_key(node_key) {
        port = node[node_key].parse().unwrap();
    }

    if log::log_enabled!(log::Level::Trace) {
        log::trace!("config: listen_port: {}", port);
    }

    let mut listen_on = Vec::<SocketAddr>::with_capacity(listen_addresses.len());
    for listen_addres in listen_addresses {
        let ip_address: IpAddr = listen_addres.parse().unwrap();
        let socket_addres = SocketAddr::new(ip_address, port);
        listen_on.push(socket_addres);
    }

    if log::log_enabled!(log::Level::Trace) {
        log::trace!("parsed: listen_on: {:?}", listen_on);
    }

    Server { listen_on }
}

fn parse_remote(node: &HashMap<String, String>) -> Remote {
    let node_key = "nodes";
    let remote_nodes: Vec<_>;
    if node.contains_key(node_key) {
        remote_nodes = node[node_key]
            .split(',')
            .map(|s| s.trim())
            .collect::<Vec<_>>();
    } else {
        remote_nodes = vec![];
    }

    if log::log_enabled!(log::Level::Trace) {
        log::trace!("config: remote nodes: {:?}", remote_nodes);
    }

    let mut nodes = Vec::<SocketAddr>::with_capacity(remote_nodes.len());
    for node in remote_nodes {
        let socket_addr: SocketAddr = node.parse().unwrap();
        nodes.push(socket_addr);
    }

    if log::log_enabled!(log::Level::Trace) {
        log::trace!("parsed: remote nodes: {:?}", nodes);
    }

    Remote { nodes }
}
