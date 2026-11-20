use rustydav::{WebDavClient, WebDavClientBuilder};
use wiremock::matchers::{basic_auth, body_bytes, body_string, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn basic_setup() -> (MockServer, WebDavClientBuilder) {
    let server = MockServer::start().await;
    let client_builder = WebDavClientBuilder::new(server.uri());

    (server, client_builder)
}

async fn setup_with_auth() -> (MockServer, WebDavClient) {
    let (server, client_builder) = basic_setup().await;
    let client = client_builder
        .with_auth("user", Some("password"))
        .build()
        .unwrap();

    (server, client)
}

/// Helper to spin up a MockServer and a WebDavClient pointed at it
async fn setup() -> (MockServer, WebDavClient) {
    let (server, client_builder) = basic_setup().await;
    let client = client_builder.build().unwrap();

    (server, client)
}

#[tokio::test]
async fn calls_should_use_basic_auth() {
    let (server, client) = setup_with_auth().await;

    Mock::given(method("GET"))
        .and(basic_auth("user", "password"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let res = client.get("some_path/").await;
    assert!(res.is_ok());
    assert_eq!(res.unwrap().status(), 200);
}

#[tokio::test]
async fn get_existing_resource() {
    let (server, client) = setup().await;

    Mock::given(method("GET"))
        .and(path("/notes.txt"))
        .respond_with(ResponseTemplate::new(200).set_body_string("hello"))
        .expect(1)
        .mount(&server)
        .await;

    let result = client.get("notes.txt").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().text().await.unwrap(), "hello");
}

#[tokio::test]
async fn get_not_existing_resource() {
    let (server, client) = setup().await;

    Mock::given(method("GET"))
        .and(path("/missing.txt"))
        .respond_with(ResponseTemplate::new(404))
        .expect(1)
        .mount(&server)
        .await;

    let res = client.get("missing.txt").await;
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().status().unwrap().as_u16(), 404);
}

#[tokio::test]
async fn put_uploads_bytes() {
    let (server, client) = setup().await;
    let payload = b"file content".to_vec();

    Mock::given(method("PUT"))
        .and(path("/archive.zip"))
        .and(header("content-type", "application/octet-stream"))
        .and(body_bytes(payload.clone()))
        .respond_with(ResponseTemplate::new(201)) // 201 Created is standard for new WebDAV files
        .mount(&server)
        .await;

    let res = client.put("archive.zip", payload).await.unwrap();
    assert_eq!(res.status(), 201);
}

#[tokio::test]
async fn delete_resource() {
    let (server, client) = setup().await;

    Mock::given(method("DELETE"))
        .and(path("/old.txt"))
        .respond_with(ResponseTemplate::new(204)) // 204 No Content is standard for WebDAV DELETE
        .mount(&server)
        .await;

    let res = client.delete("old.txt").await.unwrap();
    assert_eq!(res.status(), 204);
}

#[tokio::test]
async fn unzip_posts_from_param() {
    let (server, client) = setup().await;

    Mock::given(method("POST"))
        .and(path("/archive.zip"))
        .and(header("content-type", "application/x-www-form-urlencoded"))
        .and(body_string("method=UNZIP"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let res = client.unzip("archive.zip").await.unwrap();
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn mkcol_creates_directory() {
    let (server, client) = setup().await;

    Mock::given(method("MKCOL"))
        .and(path("/new_folder"))
        .respond_with(ResponseTemplate::new(201))
        .mount(&server)
        .await;

    let res = client.mkcol("/new_folder").await.unwrap();
    assert_eq!(res.status(), 201);
}

#[tokio::test]
async fn move_item_sets_destination_header() {
    let (server, client) = setup().await;
    let expected_dest = format!("{}/dest.txt", server.uri());

    Mock::given(method("MOVE"))
        .and(path("/src.txt"))
        .and(header("destination", expected_dest.as_str()))
        .respond_with(ResponseTemplate::new(201))
        .mount(&server)
        .await;

    let res = client.move_item("src.txt", "dest.txt").await.unwrap();
    assert_eq!(res.status(), 201);
}

#[tokio::test]
async fn copy_sets_destination_header() {
    let (server, client) = setup().await;
    let expected_dest = format!("{}/copy.txt", server.uri());

    Mock::given(method("COPY"))
        .and(path("/orig.txt"))
        .and(header("destination", expected_dest.as_str()))
        .respond_with(ResponseTemplate::new(201))
        .mount(&server)
        .await;

    let res = client.copy("orig.txt", "copy.txt").await.unwrap();
    assert_eq!(res.status(), 201);
}

#[tokio::test]
async fn list_sends_propfind_and_depth() {
    let (server, client) = setup().await;
    let sample_multistatus = r#"<?xml version="1.0" encoding="utf-8"?>
        <D:multistatus xmlns:D="DAV:">
            <D:response><D:href>/docts/file.txt</D:href></D:response>
        </D:multistatus>"#;

    Mock::given(method("PROPFIND"))
        .and(path("/docs"))
        .and(header("depth", "1"))
        .respond_with(ResponseTemplate::new(207).set_body_string(sample_multistatus))
        .mount(&server)
        .await;

    let res = client.list("docs", "1").await.unwrap();
    assert_eq!(res.status(), 207);
    assert!(res.text().await.unwrap().contains("file.txt"));
}
