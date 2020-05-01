//! # rustydav
//!
//! [![build](https://github.com/andreinbio/rustydav/workflows/build/badge.svg)](https://github.com/andreinbio/rustydav/actions?query=workflow%3Abuild)
//! [![tests](https://github.com/andreinbio/rustydav/workflows/test/badge.svg)](https://github.com/andreinbio/rustydav/actions?query=workflow%3Atest)
//! [![crates.io](https://img.shields.io/crates/v/rustydav.svg)](https://crates.io/crates/rustydav)
//! [![Documentation](https://docs.rs/rustydav/badge.svg)](https://docs.rs/rustydav)
//! [![GPL-3.0 licensed](https://img.shields.io/crates/l/rustydav.svg)](./LICENSE)
//!
//! This is a small library written in rust and inspired by [hyperdav](https://gitlab.com/Gahr/hyperdav) and uses [reqwest](https://github.com/seanmonstar/reqwest) library as the base.
//!
//! This library can be used to make calls to webdav server
//!
//! Supported methods are:
//! - **get**
//! - **put**
//! - **delete**
//! - **unzip**
//! - **mkcol**
//! - **mv**
//! - **list**
//! include **rustydav** as a dependency
//! ```toml
//! [dependencies]
//! rustydav = "0.1.2"
//! ```
//! Then add **rustydav** to your code
//! ```rust
//! extern crate rustydav;
//!
//! use rustydav::client;
//! use rustydav::prelude::*;
//! ```
//!
//! Create a client
//! ```ignore
//! let webdav_client = client::Client::init("username", "password");
//! ```
//! Now you can use the client to call any of supported methods: **get**, **put**, **delete**, **unzip**, **mkcol**, **mv**, **list**.
//! All the paths used by the methods should be absolute on the webdav server to the required file, folder, zip.
//!
//! Every method will return a Result<Response, Error>
//! ```rust
//! # let result: Result<&str, String> = Ok("test");
//! if result.is_ok() {
//!    // the method completed with success
//! } else {
//!    // somenting when wrong
//! }
//! ```

extern crate reqwest;

pub mod prelude;
pub mod client;


