# rustydav

[![build](https://github.com/andreinbio/rustydav/workflows/build/badge.svg)](https://github.com/andreinbio/rustydav/actions?query=workflow%3Abuild)
[![tests](https://github.com/andreinbio/rustydav/workflows/test/badge.svg)](https://github.com/andreinbio/rustydav/actions?query=workflow%3Atest)
[![crates.io](https://img.shields.io/crates/v/rustydav.svg)](https://crates.io/crates/rustydav)
[![Documentation](https://docs.rs/rustydav/badge.svg)](https://docs.rs/rustydav)
[![GPL-3.0 licensed](https://img.shields.io/crates/l/rustydav.svg)](./LICENSE)

Implementation of webdav requests in rust

This is a small library written in rust and inspired by [hyperdav](https://gitlab.com/Gahr/hyperdav) and uses [reqwest](https://github.com/seanmonstar/reqwest) library as the base.

This library can be used to make calls to webdav server.

Supported methods are:
- **get**
- **put**
- **delete**
- **unzip**
- **mkcol**
- **mv**
- **list**

[Changelog](CHANGELOG.md)

# Example
Small example on how to use this library

Include **rustydav** as a dependency
```rust
[dependencies]
rustydav = "0.1.3"
```
Then add this to your code
```rust
extern crate rustydav;

use rustydav::client;
use rustydav::prelude::*;
```
Short examples of call methods
```rust
// Every method will return a Result<Response, Error>

if (result.is_ok() {
    // the method completed with success
} else {
    // somenting when wrong
}

// Create the client
let webdav_client = client::Client::init(/*username*/, /*password*/);

// Get some file from server
// The result will contain the file data
let result = webdav_client.get(/*absolute url to the server file location*/);

// Upload a file to server. It can be any type of file as long as it is transformed to a vector of bytes (Vec<u8>).
// This can be achieved with std::fs::File or zip-rs for sending zip files.
let result = webdav_client.put(/*Vec<u8>*/, /*absolute path to the server file location*/);

// Delete a remote file from the server
let result = webdav_client.delete(/*absolute path to the file on the server*/);

// Unzip a zip archive on the server
let result = webdav_client.unzip(/*absolute path to the zip archive on the server*/);

// Create a new directory on server
let result = webdav_client.mkcol(/*absolute path to the server where to create the new folder*/);

// Rename or move a file / folder / zip on the server
// If the file location changes it will move the file, if only the file name changes it will rename it.
let result = webdav_client.mv(/*absolute path on the server for old file location/name*/, /*absolute on the server for new file location/name*/);

// List files and folders at the given path on the server
// Depth of "0" applies only to the resource, "1" to the resource and it's children, "infinity" to the resource and all it's children recursively
// The result will contain an xml list with the remote folder contents.
let result = webdav_client.list(/*absolute path on the server to list the files*/, /*depth being "0", "1" or "infinity"*/);
```
For some description about them please see the [**client.rs**](src/client.rs) file.
