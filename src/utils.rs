use crate::RustyDavError;
use http::header;
use reqwest::Url;
use std::collections::HashMap;

/// Parses, validates, and formats a raw string into a valid HTTP/HTTPS WebDAV base URL
/// with a guaranteed trailing slash for safe path joining.
pub fn parse_base_url(base_url_raw: &str) -> Result<Url, RustyDavError> {
    // Trim all trailing slashes from the raw string slice, then append exactly one
    let sanitized_str = format!("{}/", base_url_raw.trim_end_matches('/'));

    // Parse the clean string
    let base_url = Url::parse(&sanitized_str)?;

    // Validate url scheme
    if base_url.scheme() != "http" && base_url.scheme() != "https" {
        return Err(RustyDavError::UnsupportedScheme(
            "WebDav requires an http or https base URL".to_string(),
        ));
    }

    Ok(base_url)
}

/// Joins a subpath onto a base URL after stripping any leading slashes to ensure
/// the path is evaluated relative to the base URL directory.
pub fn join_path(base_url: &Url, path: &str) -> Result<Url, RustyDavError> {
    let sanitized_path = path.trim_start_matches('/');

    Ok(base_url.join(sanitized_path)?)
}

pub fn form_params<'a>(key: &'a str, value: &'a str) -> HashMap<&'a str, &'a str> {
    let mut params = HashMap::new();
    params.insert(key, value);
    params
}

pub fn custom_header(name: &[u8], value: &str) -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::HeaderName::from_bytes(name).unwrap(),
        header::HeaderValue::from_str(value).unwrap(),
    );
    headers
}

/// Evaluates an async reqwest response.
#[inline]
pub async fn evaluate_response_async(
    response: reqwest::Response,
) -> Result<reqwest::Response, RustyDavError> {
    if response.status().is_success() {
        Ok(response)
    } else {
        let status = response.status();
        let headers = response.headers().clone(); // Clone the headers map
        let body = response.text().await.unwrap_or_default();

        Err(RustyDavError::from_status(status, headers, body))
    }
}

/// Evaluates a blocking reqwest response.
#[cfg(feature = "blocking")]
#[inline]
pub fn evaluate_response_blocking(
    response: reqwest::blocking::Response,
) -> Result<reqwest::blocking::Response, RustyDavError> {
    if response.status().is_success() {
        Ok(response)
    } else {
        let status = response.status();
        let headers = response.headers().clone(); // Clone the headers map
        let body = response.text().unwrap_or_default();

        Err(RustyDavError::from_status(status, headers, body))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================
    // Group 1: parse_base_url
    // ========================================
    mod parse_base_url {
        use super::*;

        #[test]
        fn rejects_missing_scheme() {
            // "example.com" and "//example.com" lack a base protocol entirely
            assert!(parse_base_url("example.com").is_err());
            assert!(parse_base_url("//example.com").is_err());
        }

        #[test]
        fn rejects_unsupported_or_typo_schemes() {
            // "hps://" is a typo, and "ftp://" is unroutable for a WebDAV library
            assert!(parse_base_url("hps://example.com").is_err());
            assert!(parse_base_url("ftp://example.com").is_err());
        }

        #[test]
        fn rejects_empty_host_with_scheme() {
            // "https://" parses a valid scheme but has an illegal, empty authority
            assert!(parse_base_url("https://").is_err());
        }

        #[test]
        fn successfully_parses_valid_base_urls() {
            // base url with or without ending slash
            assert!(parse_base_url("https://example.com").is_ok());
            assert!(parse_base_url("https://example.com/").is_ok());
        }

        #[test]
        fn adds_missing_trailing_slash() {
            let result = parse_base_url("https://example.com").unwrap();
            assert_eq!(result.as_str(), "https://example.com/");
        }

        #[test]
        fn keeps_only_one_slash_at_the_end() {
            let result = parse_base_url("https://example.com////").unwrap();
            assert_eq!(result.as_str(), "https://example.com/");
        }
    }

    // ===========================
    // Group 2: join_path
    // ===========================
    mod join_path {
        use super::*;

        fn get_base_url() -> Url {
            parse_base_url("https://example.com").unwrap()
        }

        #[test]
        fn standard_subpath() {
            let base = get_base_url();
            let result = join_path(&base, "documents/notes.txt").unwrap();
            assert_eq!(result.as_str(), "https://example.com/documents/notes.txt");
        }

        #[test]
        fn subpath_for_folder() {
            let base = get_base_url();
            let result = join_path(&base, "documents/biography/").unwrap();
            assert_eq!(result.as_str(), "https://example.com/documents/biography/");
        }

        #[test]
        fn strips_single_leading_slash() {
            let base = get_base_url();
            let result = join_path(&base, "/documents/notes.txt").unwrap();
            assert_eq!(result.as_str(), "https://example.com/documents/notes.txt");
        }

        #[test]
        fn strips_multiple_leading_slashes() {
            let base = get_base_url();
            let result = join_path(&base, "///documents/notes.txt").unwrap();
            assert_eq!(result.as_str(), "https://example.com/documents/notes.txt");
        }
    }

    // ===========================
    // Group 3: custom_header
    // ===========================
    mod custom_header {
        use super::*;

        #[test]
        fn test_custom_header() {
            let header = custom_header(b"name", "value");
            assert_eq!(header.get("name").unwrap(), "value")
        }
    }
}
