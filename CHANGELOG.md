# v0.2.0

### Added
* Full **async** WebDAV client support by default (`get`, `put`, `delete`, `mkcol`, `move_item`, `copy`, `list`, `unzip`).
* `WebDavClientBuilder` for unified client configuration (`with_auth`, `.build()`, and `.build_blocking()`).
* Top-level re-exports (`WebDavClient`, `WebDavBlockingClient`, `WebDavClientBuilder`, `RustyDavError`) for cleaner imports.
* Structured `RustyDavError` enum with detailed HTTP status, header, and response body context.
* Integration test suite using `wiremock` for both async and blocking clients.

### Changed (Breaking)
* **Async by default:** Switched the default client mode from synchronous (`blocking`) to asynchronous. Enable `features = ["blocking"]` in `Cargo.toml` to use `WebDavBlockingClient`.
* Upgraded to **Rust 2024 edition**.
* Updated **reqwest** to `v0.13` (alongside `base64` `v0.23`, `http` `v1.5`, and `url` `v2.5`).
* Replaced direct client initialization with `WebDavClientBuilder::new(url)`.
* Updated all client methods to return `Result<Response, RustyDavError>`.
* Improved base URL validation (`http`/`https` scheme checks) and relative path handling.

# v0.1.3

Update **reqwest** to `v0.11`

# v0.1.2

Update **reqwest** to `v0.10`

Add `blocking` feature

# v0.1.1

Update documentaion.

# v0.1.0

Initial release.
