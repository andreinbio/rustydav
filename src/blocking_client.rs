#![cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
use crate::RustyDavError;
use crate::utils::{custom_header, evaluate_response_blocking, form_params, join_path};
use reqwest::{
    Method, Url,
    blocking::{Client, Response},
};

pub struct WebDavBlockingClient {
    pub(crate) inner: Client,
    pub(crate) base_url: Url,
}

impl WebDavBlockingClient {
    // Package-visible constructor used only by the builder
    pub(crate) fn from_inner(inner: Client, base_url: Url) -> Self {
        Self { inner, base_url }
    }

    // --- WebDAV Synchronous Implementation ---

    /// Get a file from WebDav server
    ///
    /// Use relative path to the WebDav server file location
    pub fn get(&self, path: &str) -> Result<Response, RustyDavError> {
        let url = join_path(&self.base_url, path)?;
        let response = self.inner.get(url).send()?;

        evaluate_response_blocking(response)
    }

    /// Upload a file/zip on WebDav server
    ///
    /// It can be any type of file as long as it is transformed to a vector of bytes (`Vec<u8>`).
    /// This can be achieved with **std::fs::File** or **zip-rs** for sending zip files.
    ///
    /// Use relative path to the WebDav server folder location
    pub fn put(&self, path: &str, body: Vec<u8>) -> Result<Response, RustyDavError> {
        let url = join_path(&self.base_url, path)?;
        let header = custom_header(b"content-type", "application/octet-stream");
        let response = self.inner.put(url).headers(header).body(body).send()?;

        evaluate_response_blocking(response)
    }

    /// Deletes the collection, file, folder or zip archive at the given path on WebDav server
    ///
    /// Use relative path to the WebDav server resource location
    pub fn delete(&self, path: &str) -> Result<Response, RustyDavError> {
        let url = join_path(&self.base_url, path)?;
        let response = self.inner.delete(url).send()?;

        evaluate_response_blocking(response)
    }

    /// Unzip the .zip archieve on WebDav server
    ///
    /// Use relative path to the WebDav server zip locaiton
    pub fn unzip(&self, path: &str) -> Result<Response, RustyDavError> {
        let url = join_path(&self.base_url, path)?;
        let response = self
            .inner
            .post(url)
            .form(&form_params("method", "UNZIP"))
            .send()?;

        evaluate_response_blocking(response)
    }

    /// Creates a directory on WebDav server
    ///
    /// Use relative path to the WebDav server folder location
    pub fn mkcol(&self, path: &str) -> Result<Response, RustyDavError> {
        let url = join_path(&self.base_url, path)?;
        let method = Method::from_bytes(b"MKCOL")?;
        let response = self.inner.request(method, url).send()?;

        evaluate_response_blocking(response)
    }

    /// Rename or move a collection, file, folder on WebDav server
    ///
    /// If the file location changes it will move the file, if only the file name changes it will rename it.
    ///
    /// Use relative path to the WebDav server resource location
    pub fn move_item(&self, from: &str, to: &str) -> Result<Response, RustyDavError> {
        let from_url = join_path(&self.base_url, from)?;
        let to_url = join_path(&self.base_url, to)?;
        let method = Method::from_bytes(b"MOVE")?;
        let header = custom_header(b"destination", to_url.as_str());
        let response = self
            .inner
            .request(method, from_url)
            .headers(header)
            .send()?;

        evaluate_response_blocking(response)
    }

    /// Copy a resource ( file, folder and all its members ) on WebDav server
    ///
    /// Use relative path to the WebDav resource locaiton
    pub fn copy(&self, from: &str, to: &str) -> Result<Response, RustyDavError> {
        let from_path = join_path(&self.base_url, from)?;
        let to_path = join_path(&self.base_url, to)?;
        let method = Method::from_bytes(b"COPY")?;
        let header = custom_header(b"destination", to_path.as_str());
        let response = self
            .inner
            .request(method, from_path)
            .headers(header)
            .send()?;

        evaluate_response_blocking(response)
    }

    /// List files and folders at the given path on WevDav server
    ///
    /// Depth of "0" applies only to the resource, "1" to the resource and it's children, "infinity" to the resource and all it's children recursively
    ///
    /// Use relative path to the WebDav server folder location
    pub fn list(&self, path: &str, depth: &str) -> Result<Response, RustyDavError> {
        let url = join_path(&self.base_url, path)?;
        let body = r#"<?xml version="1.0" encoding="utf-8" ?>
            <D:propfind xmlns:D="DAV:">
                <D:allprop/>
            </D:propfind"#;
        let method = Method::from_bytes(b"PROPFIND")?;
        let header = custom_header(b"depth", depth);
        let response = self
            .inner
            .request(method, url)
            .headers(header)
            .body(body)
            .send()?;

        evaluate_response_blocking(response)
    }
}
