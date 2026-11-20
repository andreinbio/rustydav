//! # rustydav: A Rust WebDAV Client Library
//!
//! `rustydav` is a comprehensive library for interacting with WebDAV servers, providing both
//! **asynchronous** (`async`) and **synchronous** (`blocking`) client implementations.
//! It leverages the powerful [`reqwest`](https://github.com/seanmonstar/reqwest) HTTP engine under the hood.
//!
//! ## Features
//! - ✅ Support for core WebDAV operations: `GET`, `PUT`, `DELETE`, `MKCOL`, `MOVE`, `COPY`, `PROPFIND` (List), `UNZIP`.
//! - ✅ Dual-mode clients: Async (`tokio`/`async-std`) and Sync (`blocking`).
//! - ✅ Typed responses and robust error handling.
//! - ✅ Supports basic authentication ( user + password ) and with certificate
//! - ✅ Inspired by [hyperdav](https://gitlab.com/Gahr/hyperdav).
//!
//! ## Status & Licensing
//! [![tests](https://github.com/andreinbio/rustydav/workflows/test/badge.svg)](https://github.com/andreinbio/rustydav/actions?query=workflow%3Atest)
//! [![crates.io](https://img.shields.io/crates/v/rustydav.svg)](https://crates.io/crates/rustydav)
//! [![Documentation](https://docs.rs/rustydav/badge.svg)](https://docs.rs/rustydav)
//! [![GPL-3.0 licensed](https://img.shields.io/crates/l/rustydav?style=flat-square)](./LICENSE)
//!
//! ## Installation
//! Add `rustydav` to your `Cargo.toml`. By default, it uses the **async** client. To use the blocking client, enable the `blocking` feature.
//!
//! ### Async (Default)
//! ```toml
//! [dependencies]
//! rustydav = "0.2.0"
//! ```
//!
//! ### Sync/Blocking
//! ```toml
//! [dependencies]
//! rustydav = { version = "0.2.0", features = ["blocking"] }
//! ```
//!
//! ## Usage Examples
//!
//! ### 1. Async Client (Recommended for concurrent applications)
//! ```rust,no_run
//! use rustydav::{WebDavClientBuilder, RustyDavError};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), RustyDavError> {
//!     // Initialize the client with credentials
//!     let client = WebDavClientBuilder::new("https://example.com/")
//!         .with_auth("username", Some("password"))
//!         .build()?;
//!
//!     // Example: List directory contents
//!     match client.list("some/path/", "0").await {
//!         Ok(resources) => println!("List successful!"),
//!         Err(err) => eprintln!("Failed to list directory: {}", err),
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### 2. Sync/Blocking Client (For scripts or non-async contexts)
//! ```rust,no_run
//! use rustydav::{WebDavClientBuilder, RustyDavError};
//!
//! fn main() -> Result<(), RustyDavError> {
//!     // Initialize the client with credentials
//!     let client = WebDavClientBuilder::new("https://example.com/")
//!         .with_auth("username", Some("password"))
//!         .build_blocking()?;
//!
//!     // Example: List directory contents
//!     match client.list("some/path/", "0") {
//!         Ok(resources) => println!("List successful!"),
//!         Err(err) => eprintln!("Failed to list directory: {}", err),
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Supported Operations
//! The client supports the following core WebDAV methods. All paths should be relative to the base url (e.g., `/folder/file.txt`).
//! - `get`: Retrieve file contents.
//! - `put`: Upload/overwrite a file.
//! - `delete`: Remove a file or directory.
//! - `mkcol`: Create a new directory (collection).
//! - `move_item`: Move or rename a resource.
//! - `copy`: Copy a resource and all its members
//! - `list`: List resources in a directory (`PROPFIND`).
//! - `unzip`: Extract zip archives on the server (if supported by the server implementation).
//!
//! ## Error Handling
//! All client methods return a `Result<Response, RustyDavError>`. You can handle failures
//! using pattern matching to inspect specific error variations:
//!
//! ```rust,no_run
//! # use rustydav::{WebDavClientBuilder, RustyDavError};
//! # use reqwest::Response;
//! # async fn example() -> Result<(), RustyDavError> {
//! # let client = WebDavClientBuilder::new("https://example.com").build()?;
//! #
//! match client.get("path/to/file").await {
//!     Ok(response) => println!("Success! Download completed."),
//!     Err(err) => eprintln!("Network or configuration error occurred: {}", err),
//! }
//! # Ok(())
//! # }
//! ```

pub mod builder;
pub mod client;
pub mod error;

// --- Client & Error Types ---
pub use builder::WebDavClientBuilder;
pub use client::WebDavClient;
pub use error::RustyDavError;

// --- Success & Utility Path Types ---
pub use reqwest::{Method, Response};

// --- Error Path Types ---
pub use reqwest::StatusCode;
pub mod header {
    pub use reqwest::header::HeaderMap;
    pub use reqwest::header::HeaderName;
    pub use reqwest::header::HeaderValue;
}

// --- Blocking Client Module ---
#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub mod blocking_client;

#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub use blocking_client::WebDavBlockingClient;

#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub mod blocking {
    pub use reqwest::blocking::Response;
}

pub(crate) mod utils;
