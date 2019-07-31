# rustydav
Implementation of webdav requests in rust

This is a small library written in rust and inspired by [hyperdav](https://gitlab.com/Gahr/hyperdav)
It uses [reqwest](https://github.com/seanmonstar/reqwest) library as the base.

# Example
Small example how to use this library

include **rustydav** as a dependency
```rust
[dependencies]
rustydav = { git = "https://github.com/andreinbio/rustydav", branch = "master"}
```
Then add this to your code
```rust
extern crate rustydav;

use rustydav::client;
use rustydav::prelude::*;

let client = client::Client::init(/*username*/, /*password*/);

// get some file from server
// it will return a Result<Response, Error>
let result = client.get(/*absolute url to the server file location*/);

if result.is_ok() {
    // do somenthing with received data
} else {
    // do somenting else for the error case
}
```
Supported methods are: **get**, **put**, **delete**, **unzip**, **mkcol**, **mv**, **list**.
For some description about them please see the **client.rs** file.

Hope to update the Description with more relevant information in the future.