# rustydav: A Rust WebDAV Client Library

`rustydav` is a comprehensive library for interacting with WebDAV servers, providing both
**asynchronous** (`async`) and **synchronous** (`blocking`) client implementations.
It leverages the powerful [`reqwest`](https://github.com/seanmonstar/reqwest) HTTP engine under the hood.

## Features
- ✅ Support for core WebDAV operations: `GET`, `PUT`, `DELETE`, `MKCOL`, `MOVE`, `COPY`, `PROPFIND` (List), `UNZIP`.
- ✅ Dual-mode clients: Async (`tokio`/`async-std`) and Sync (`blocking`).
- ✅ Typed responses and robust error handling.
- ✅ Supports basic authentication ( user + password ) and with certificate
- ✅ Inspired by [hyperdav](https://gitlab.com/Gahr/hyperdav).

## Status & Licensing
[![Rust Tests](https://github.com/andreinbio/rustydav/actions/workflows/rust.yml/badge.svg)](https://github.com/andreinbio/rustydav/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/rustydav.svg)](https://crates.io/crates/rustydav)
[![Documentation](https://docs.rs/rustydav/badge.svg)](https://docs.rs/rustydav)
[![GPL-3.0 licensed](https://img.shields.io/crates/l/rustydav?style=flat-square)](./LICENSE)

## Installation
Add `rustydav` to your `Cargo.toml`. By default, it uses the **async** client. To use the blocking client, enable the `blocking` feature.

### Async (Default)
```toml
[dependencies]
rustydav = "0.2.0"
```

### Sync/Blocking
```toml
[dependencies]
rustydav = { version = "0.2.0", features = ["blocking"] }
```

## Usage Examples

### 1. Async Client (Recommended for concurrent applications)
```rust
use rustydav::{WebDavClientBuilder, RustyDavError};

#[tokio::main]
async fn main() -> Result<(), RustyDavError> {
    // Initialize the client with credentials
    let client = WebDavClientBuilder::new("https://example.com/")
        .with_auth("username", Some("password"))
        .build()?;

    // Example: List directory contents
    match client.list("some/path/", "0").await {
        Ok(resources) => println!("List successful!"),
        Err(err) => eprintln!("Failed to list directory: {}", err),
    }

    Ok(())
}
```

### 2. Sync/Blocking Client (For scripts or non-async contexts)
```rust
use rustydav::{WebDavClientBuilder, RustyDavError};

fn main() -> Result<(), RustyDavError> {
    // Initialize the client with credentials
    let client = WebDavClientBuilder::new("https://example.com/")
        .with_auth("username", Some("password"))
        .build_blocking()?;

    // Example: List directory contents
    match client.list("some/path/", "0") {
        Ok(resources) => println!("List successful!"),
        Err(err) => eprintln!("Failed to list directory: {}", err),
    }

    Ok(())
}
```

## Supported Operations
The client supports the following core WebDAV methods. All paths should be relative to the base url (e.g., `/folder/file.txt`).
- `get`: Retrieve file contents.
- `put`: Upload/overwrite a file.
- `delete`: Remove a file or directory.
- `mkcol`: Create a new directory (collection).
- `move_item`: Move or rename a resource.
- `copy`: Copy a resource and all its members
- `list`: List resources in a directory (`PROPFIND`).
- `unzip`: Extract zip archives on the server (if supported by the server implementation).

## Error Handling
All client methods return a `Result<Response, RustyDavError>`. You can handle failures
using pattern matching to inspect specific error variations:

```rust
use rustydav::RustyDavError;

match client.get("path/to/file").await {
    Ok(response) => println!("Download completed: {}.", response.status()),
    Err(RustyDavError::WebDaveStatus(err)) => {
        eprintln!("Server error ({}): {}", err.status, err.body);
    },
    Err(RustyDavError::Http(err)) => {
        eprintln!("Network or connection error: {}", err);
    },
    Err(err) => eprintln!("Configuration error: {}", err),
}
```
Available `RustyDavError` variants:
* **`WebDavStatus`** — Non-2xx HTTP response from the server (includes `status`, `headers`, and `body`).
* **`Http`** — Underlying `reqwest` network, TLS, or timeout failure.
* **`UrlParse` / `UnsupportedScheme`** — Malformed URL or non-`http(s)` scheme.
* **`InvalidMethod`** — Invalid custom HTTP/WebDAV verb.
