# Notes

* Support for commonly used data types and structures
  * support for storing: file, blob, json, xml, arrays, set, map, primitive types
  * support for metafields: tags, descriptions, metadata
  * support for seek operations for blobs and files (reading data at given position)
  * support for streaming

* File server
  * fetch using HTTP
  * fetch using a connected TCP client

* Authentification and Authorization
  * support for JWT
  * support for Fine-Grained Access Control

* Scaling
  * heartbeat
  * consistent hashing
  * easy scaling
    * built-in configuration service in nodes
    * auto replication of configuration parameters among nodes
    * automatic (plug & play) configuration of existing nodes when a new node added into claster
      * new node needs to know at least one neighbour in the claster. The added node and other nodes would be updated automatically after completing the claster configuration rebuild process.
    * any change in the claster configuration that set or discovered (adding new node, failing existing nodes), would be automatically replicated to other nodes in the claster (no need for master node)
    * configuration updates are received by clients automatically
      * each request of client includes a config_id parameter
      * server node will analyze the received config_id and may respond to client with updated configuration
    * clients and nodes build weighted graphs to optimize node/peer selection and other network operations.

* Caching
  * support for evictions (LRU)
  * support client-side and server-side caches

* WIP: Logging

* Storage Spaces
  * support for folders
  * support for path

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

* WIP: config file with server and client sections
* benchmarks against single thread, pooling and async
* use [Protocol Buffers](https://protobuf.dev/) for wire messages: https://github.com/tokio-rs/prost

* WIP: file server: add support for partial requests (Content-Range)
* file server: add support for If-Modified-Since, If-None-Match
