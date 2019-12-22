use super::prelude::*;
use std::collections::HashMap;

/// Webdav client
///
/// Create a client
/// ```rust
/// let client = client::Client::init(/*username*/, /*password*/);
/// ```
/// Now you can use the client to call any of the methods listed bellow.
///
/// All the paths used by the methods should be absolute on the webdav server to the required file, folder, zip.
///
/// Every method will return a Result<Response, Error>
/// ```rust
/// if (result.ok() {
///    // the method completed with success
/// } else {
///    // somenting when wrong
/// }
/// ```
#[derive(Debug)]
pub struct Client {
    username: String,
    password: String,
    client: reqwest::Client,
}

impl Client {
    /// Initialization of the client
    ///
    /// Initialized client will be stored for future requests
    pub fn init(username: &str, password: &str) -> Self {
        Client {
            username: username.to_owned(),
            password: password.to_owned(),
            client: reqwest::Client::new(),
        }
    }

    fn custom_header(&self, name: &str, value: &str) -> header::HeaderMap {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::HeaderName::from_bytes(name.as_bytes()).unwrap(), header::HeaderValue::from_bytes(value.as_bytes()).unwrap());
        headers
    }

    fn form_params(&self, key: &'static str, value: &'static str) -> HashMap<&str, &str> {
        let mut params = HashMap::new();
        params.insert(key, value);

        params
    }

    /// Main function that creates the RequestBuilder, sets the method, url and the basic_auth
    fn start_request(&self, method: Method, path: &str) -> RequestBuilder {
        self.client
            .request(method, Url::parse(path).unwrap())
            .basic_auth(self.username.as_str(), Some(self.password.as_str()))
    }
    
    /// Get a file from Webdav server
    ///
    /// Use absolute path to the webdav server file location
    pub fn get(&self, path: &str) -> Result<Response, Error> {
        self.start_request(Method::GET, path)
            .send()
    }
    
    /// Upload a file/zip on Webdav server
    ///
    /// It can be any type of file as long as it is transformed to a vector of bytes (Vec<u8>).
    /// This can be achieved with **std::fs::File** or **zip-rs** for sending zip files.
    pub fn put<B: Into<Body>>(&self, body: B, path: &str) -> Result<Response, Error> {
        self.start_request(Method::PUT, path)
            .headers(self.custom_header("content-type", "application/octet-stream"))
            .body(body)
            .send()
    }
    
    /// Deletes the collection, file, folder or zip archive at the given path on Webdav server
    ///
    /// Use absolute path to the webdav server file location
    pub fn delete(&self, path: &str) -> Result<Response, Error> {
        self.start_request(Method::DELETE, path)
            .send()
    }

    /// Unzips the .zip archieve on Webdav server
    ///
    /// Use absolute path to the webdav server file location
    pub fn unzip(&self, path: &str) -> Result<Response, Error> {
        self.start_request(Method::POST, path)
            .form(&self.form_params("method", "UNZIP"))
            .send()
    }

    /// Creates a directory on Webdav server
    ///
    /// Use absolute path to the webdav server file location
    pub fn mkcol(&self, path: &str) -> Result<Response, Error> {
        self.start_request(Method::from_bytes(b"MKCOL").unwrap(), path)
            .send()
    }

    /// Rename or move a collection, file, folder on Webdav server
    ///
    /// If the file location changes it will move the file, if only the file name changes it will rename it.
    /// Use absolute path to the webdav server file location
    pub fn mv(&self, from: &str, to: &str) -> Result<Response, Error> {
        self.start_request(Method::from_bytes(b"MOVE").unwrap(), from)
            .headers(self.custom_header("destination", to))
            .send()
    }

    /// List files and folders at the given path on Webdav server
    ///
    /// Depth of "0" applies only to the resource, "1" to the resource and it's children, "infinity" to the resource and all it's children recursively
    /// The result will contain an xml list with the remote folder contents.
    /// Use absolute path to the webdav server folder location
    pub fn list(&self, path: &str, depth: &str) -> Result<Response, Error> {
        let body = r#"<?xml version="1.0" encoding="utf-8" ?>
            <D:propfind xmlns:D="DAV:">
                <D:allprop/>
            </D:propfind>
        "#;

        self.start_request(Method::from_bytes(b"PROPFIND").unwrap(), path)
            .headers(self.custom_header("depth", depth))
            .body(body)
            .send()
    }
}
