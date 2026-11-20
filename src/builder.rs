use base64::prelude::*;
use reqwest::{
    Identity,
    header::{AUTHORIZATION, HeaderMap, HeaderValue},
};

use crate::{RustyDavError, utils::parse_base_url};

pub struct WebDavClientBuilder {
    base_url_raw: String,
    cert_data: Option<(Vec<u8>, String)>,
    auth_data: Option<(String, String)>,
    user_agent: Option<String>,
}

impl WebDavClientBuilder {
    /// Initialization of the client
    ///
    /// Set the base url of the the Webdav server
    pub fn new(base_url: impl AsRef<str>) -> Self {
        Self {
            base_url_raw: base_url.as_ref().to_string(),
            cert_data: None,
            auth_data: None,
            user_agent: None,
        }
    }

    /// Set the user agent for the curent client
    pub fn with_user_agent(mut self, user_agent: impl AsRef<str>) -> Self {
        self.user_agent = Some(user_agent.as_ref().to_string());
        self
    }

    /// Set authentication with username and password
    pub fn with_auth(
        mut self,
        username: impl AsRef<str>,
        password: Option<impl AsRef<str>>,
    ) -> Self {
        let username = username.as_ref().to_string();
        let password = password.map(|p| p.as_ref().to_string()).unwrap_or_default();
        self.auth_data = Some((username, password));
        self
    }

    /// Sets the authentication with a certificate and passphrase
    ///
    /// Takes precedence over basic auth
    pub fn with_certificate(mut self, p12_der: Vec<u8>, passphrase: impl AsRef<str>) -> Self {
        self.cert_data = Some((p12_der, passphrase.as_ref().to_string()));
        self
    }

    /// Factory method for the Default Async Client
    ///
    /// Main function that builds the client, sets the authentication and default headers
    pub fn build(self) -> Result<crate::client::WebDavClient, RustyDavError> {
        // Parse the URL here. If it fails, bubble up the reqwest error.
        let base_url = parse_base_url(&self.base_url_raw)?;

        let mut builder = reqwest::Client::builder();
        let mut headers = HeaderMap::new();

        if let Some((der, pass)) = &self.cert_data {
            builder = builder.identity(Identity::from_pkcs12_der(der, pass)?);
        }
        if let Some((user, pass)) = &self.auth_data {
            let encoded = BASE64_STANDARD.encode(format!("{}:{}", user, pass));
            let mut auth_value = HeaderValue::from_str(&format!("Basic {}", encoded)).unwrap();

            // Best Practice: Mark authorization header as sensitive so it's hidden in logs
            auth_value.set_sensitive(true);
            headers.insert(AUTHORIZATION, auth_value);
        }
        if !headers.is_empty() {
            builder = builder.default_headers(headers);
        }
        if let Some(ua) = self.user_agent {
            builder = builder.user_agent(ua);
        }

        Ok(crate::client::WebDavClient::from_inner(
            builder.build()?,
            base_url,
        ))
    }

    /// Factory method for the Sync Client (Compiles out if feature flag is off)
    #[cfg(feature = "blocking")]
    #[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
    pub fn build_blocking(
        self,
    ) -> Result<crate::blocking_client::WebDavBlockingClient, RustyDavError> {
        // Parse the URL here. If it fails, bubble up the reqwest error.
        let base_url = parse_base_url(&self.base_url_raw)?;

        let mut builder = reqwest::blocking::Client::builder();
        let mut headers = HeaderMap::new();

        if let Some((der, pass)) = &self.cert_data {
            builder = builder.identity(Identity::from_pkcs12_der(der, pass)?);
        }
        if let Some((user, pass)) = &self.auth_data {
            let encoded = BASE64_STANDARD.encode(format!("{}:{}", user, pass));
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Basic {}", encoded)).unwrap(),
            );
        }
        if !headers.is_empty() {
            builder = builder.default_headers(headers);
        }
        if let Some(ua) = self.user_agent {
            builder = builder.user_agent(ua);
        }

        Ok(crate::blocking_client::WebDavBlockingClient::from_inner(
            builder.build()?,
            base_url,
        ))
    }
}
