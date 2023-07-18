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
* scaling & nodes
  * support for consistent hashing
* caching
  * support for evictions (LRU)
  * support client-side and server-side caches
* logging
