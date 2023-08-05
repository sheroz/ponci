# Roadmap

## Features

- WIP: Support for commonly used data types and structures
  - support for storing: file, blob, json, xml, arrays, set, map, primitive types
  - support for metafields: tags, descriptions, metadata
  
- WIP: TCP Server
  - basic functionality
  - async support

- WIP: HTTP File Server
  - basic functionality
  - async support
  - streaming (seeking and reading data at given position)

- WIP: Client
  - basic functionality
  - async support

- WIP: Configuration

- WIP: Logging

- Authentification and Authorization
  - support for JWT
  - support for Fine-Grained Access Control

- Scaling
  - easy scaling
    - support for built-in configuration service in nodes
    - support for auto replicating of configuration parameters in claster nodes
    - support for plug & play in adding a new node into claster and re-configurating of existing nodes
      - new node needs to know at least one neighbour in the claster. The added node and other nodes would be updated after completing the re-configuration and re-building process of claster.
    - any change in the claster configuration (adding new node, failing existing nodes), would be auto replicated to other nodes in the claster (no need for master node)
    - clients would be auto-updated after re-configuration
      - each client request includes a config_id parameter
      - server node will analyze the received config_id and may respond with updated configuration settings
    - clients and nodes use weighted graphs to optimize node/peer selection and other network operations.
  - consistent hashing
  - claster heartbeat

- Caching
  - support for evictions (LRU)
  - support client-side and server-side caches

- Storage Spaces
  - support for folders
  - support for path

## Notes

Storage space samples:

    path_id1 = path1/sub-path1
    path_id2 = path_id1/other_path2/item_id1

    get_item_type(path_id1) => PathItem
    parse_path(path_id1) => [
        path1 : FolderItem,
        subpath1 : FolderItem,
    ]

    get_item_type(path_id2) => PathItem
    parse_path(path_id2) => [
        path_id1 : PathItem,
        other_path2 : FolderItem,
        item_id1 : FileItem
    ]

    parse_full_path(path_id2) => [
        path1 : FolderItem,
        subpath1 : FolderItem,
        other_path2 : FolderItem,
        item_id1 : FileItem
    ]

- WIP: config file with server and client sections
- benchmarks against single thread, pooling and async
- use [Protocol Buffers](https://protobuf.dev/) for [wire messages](https://github.com/tokio-rs/prost)

- WIP: file server: add support for partial requests (Content-Range)
- file server: add support for If-Modified-Since, If-None-Match

file server: http if range,etags
client: send file http
client: get file http
client: send file tcp
client: get file tcp

authorization: node_token XOR …
client_access_token = …
node_token = …
path_access_token = …
item_access_token = …
