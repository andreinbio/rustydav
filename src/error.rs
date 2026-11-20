use reqwest::StatusCode;
use reqwest::header::HeaderMap;
use std::fmt;

#[derive(Debug)]
pub struct WebDavStatusError {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
}

#[derive(Debug)]
pub enum RustyDavError {
    /// Preserves the full reqwest HTTP/Network error context
    Http(Box<reqwest::Error>),
    /// Preserves the exact reason why an HTTP custom method was invalid
    InvalidMethod(http::method::InvalidMethod),
    /// Explicit URL parsing or joining failure context
    UrlParse(url::ParseError),
    /// Caught when an unroutable or malformed protocol scheme is supplied
    UnsupportedScheme(String),
    /// Caught when the WebDAV server responds with a non-success HTTP status code (4xx or 5xx)
    WebDavStatus(Box<WebDavStatusError>),
}

impl fmt::Display for RustyDavError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustyDavError::Http(e) => write!(f, "HTTP error: {}", e),
            RustyDavError::InvalidMethod(e) => write!(f, "Invalid WebDAV method: {}", e),
            RustyDavError::UrlParse(e) => write!(f, "URL parse error: {}", e),
            RustyDavError::UnsupportedScheme(s) => write!(
                f,
                "Unsupported or malformed URL scheme: '{}'. Expected http or https",
                s
            ),
            RustyDavError::WebDavStatus(err) => {
                writeln!(
                    f,
                    "WebDAV server responded with a failure status: {} ({})",
                    err.status.as_u16(),
                    err.status.canonical_reason().unwrap_or("Unknown Error")
                )?;

                writeln!(f, "Headers: {:?}", err.headers)?;

                if !err.body.is_empty() {
                    write!(f, "Body:\n{}", err.body)?;
                } else {
                    write!(f, "Body: <empty>")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for RustyDavError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RustyDavError::Http(e) => Some(e),
            RustyDavError::InvalidMethod(e) => Some(e),
            RustyDavError::UrlParse(e) => Some(e),
            _ => None,
        }
    }
}

// Provide automatic conversion hooks using the From trait
impl From<reqwest::Error> for RustyDavError {
    fn from(err: reqwest::Error) -> Self {
        RustyDavError::Http(Box::new(err))
    }
}

impl From<http::method::InvalidMethod> for RustyDavError {
    fn from(err: http::method::InvalidMethod) -> Self {
        RustyDavError::InvalidMethod(err)
    }
}

// Auto-convert url::ParseError into RustyDavError
impl From<url::ParseError> for RustyDavError {
    fn from(err: url::ParseError) -> Self {
        RustyDavError::UrlParse(err)
    }
}

impl RustyDavError {
    /// Helper constructor to create a boxed WebDavStatus error cleanly
    pub fn from_status(status: StatusCode, headers: HeaderMap, body: String) -> Self {
        RustyDavError::WebDavStatus(Box::new(WebDavStatusError {
            status,
            headers,
            body,
        }))
    }

    // Convenient method to get the status code for an WebDavStatus error
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            RustyDavError::Http(e) => e.status(),
            RustyDavError::WebDavStatus(err) => Some(err.status),
            _ => None,
        }
    }
}
