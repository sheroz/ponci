# Poncu

## Distributed Data Storage

[![crates.io](https://img.shields.io/crates/v/poncu)](https://crates.io/crates/poncu)
[![docs](https://img.shields.io/docsrs/poncu)](https://docs.rs/poncu)
[![build & test](https://github.com/sheroz/poncu/actions/workflows/ci.yml/badge.svg)](https://github.com/sheroz/poncu/actions/workflows/ci.yml)
[![MIT](https://img.shields.io/github/license/sheroz/poncu)](https://github.com/sheroz/poncu/tree/main/LICENSE.txt)

* file server
  * fetching files using HTTP (cleint)
  * fetching files using TCP (client)
* data storage (json, xml, blob, file)
  * support for tags
  * support for metadata
  * support for descriptions
  * support for streaming
  * support seek operations (reading data at given position)
* nodes: implement consistent hashing
* caching
  * support for evictions (LRU)
  * support for both client and server parts
* logging
