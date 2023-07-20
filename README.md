# Poncu

## Distributed Data Storage

[![crates.io](https://img.shields.io/crates/v/poncu)](https://crates.io/crates/poncu)
[![docs](https://img.shields.io/docsrs/poncu)](https://docs.rs/poncu)
[![build & test](https://github.com/sheroz/poncu/actions/workflows/ci.yml/badge.svg)](https://github.com/sheroz/poncu/actions/workflows/ci.yml)
[![MIT](https://img.shields.io/github/license/sheroz/poncu)](https://github.com/sheroz/poncu/tree/main/LICENSE.txt)

* data storage (json, xml, blob, file)
  * support for metafields: tags, descriptions, metadata
  * support seek operations for blobs and files (reading data at given position)
  * support for streaming
* file server
  * fetch using HTTP
  * fetch using a connected TCP client
* authorization
* scaling
  * support for consistent hashing
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
* caching
  * support for evictions (LRU)
  * support client-side and server-side caches
* logging
