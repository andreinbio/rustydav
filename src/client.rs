use super::prelude::*;
use std::collections::HashMap;
use std::fs::File;

#[derive(Debug)]
pub struct Client {
    username: String,
    password: String,
    client: reqwest::Client,
}

impl Client {
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
    pub fn get(&self, path: &str) -> Result<Response, Error> {
        self.start_request(Method::GET, path)
            .send()
    }

    /// Put a file on Webdav server
    pub fn put(&self, body: File, path: &str) -> Result<Response, Error> {
        self.start_request(Method::PUT, path)
            .headers(self.custom_header("content-type", "application/octet-stream"))
            .body(body)
            .send()
    }

    /// Deletes the collection, file, folder at the given path on Webdav server
    pub fn delete(&self, path: &str) -> Result<Response, Error> {
        self.start_request(Method::DELETE, path)
            .send()
    }

    /// Unzips the .zip archieve on Webdav server
    pub fn unzip(&self, path: &str) -> Result<Response, Error> {
        self.start_request(Method::POST, path)
            .form(&self.form_params("method", "UNZIP"))
            .send()
    }

    /// Creates a directory on Webdav server
    pub fn mkcol(&self, path: &str) -> Result<Response, Error> {
        self.start_request(Method::from_bytes(b"MKCOL").unwrap(), path)
            .send()
    }

    /// Rename or move a collection, file, folder on Webdav server
    pub fn mv(&self, from: &str, to: &str) -> Result<Response, Error> {
        self.start_request(Method::from_bytes(b"MOVE").unwrap(), from)
            .headers(self.custom_header("destination", to))
            .send()
    }

    /// List files and folders at the given path on Webdav server
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