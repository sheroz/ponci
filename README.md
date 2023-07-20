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
    * built-in configuration service in node servers
    * auto replication of configuration parameters among server nodes
    * automatic (plug & play) configuration of existing nodes when a new node added into claster
      * new node needs to know only one neigbour to start in the claster
    * any change in configuration that set or discovered at any node, would be automatically replicated to other nodes in the claster (no need for master node)
    * clients receive changes in the configuration automatically
      * each client request includes a param with a config_id
      * server node will analyze the received config_id and may respond to client with updated configuration if there is any change
    * each server node and client builds a weighted graph of nodes to optimize node/peer selection and other network operations.
* caching
  * support for evictions (LRU)
  * support client-side and server-side caches
* logging
