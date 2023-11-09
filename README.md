
# Basic Example
```rust
use url::form_urlencoded::byte_serialize;
use webt::header::{ContentDisposition, HeaderKey};
fn main() {
    let file_name: String = byte_serialize("文件file.rs".as_bytes()).collect();

    let dis = ContentDisposition::try_from(format!("attachment; filename={}", file_name)).unwrap();
    assert_eq!(dis.file_name(), Some("文件file.rs"));

    let dis =
        ContentDisposition::try_from(format!("attachment; filename=\"{}\"", file_name)).unwrap();
    assert_eq!(dis.file_name(), Some("文件file.rs"));

    let dis1 = ContentDisposition::new_with_filename("文件file.rs");
    assert_eq!(dis1, dis);
}

#[allow(unused)]
fn header_value_test() -> Result<(), webt::header::HeaderParserError> {
    use hyper::header::{HeaderMap, CONTENT_DISPOSITION};
    use webt::header::HeaderParserError;
    let mut header = HeaderMap::new();
    assert_eq!(
        Err(HeaderParserError::MissingHeaderValue(CONTENT_DISPOSITION)),
        ContentDisposition::try_from(&header)
    );
    let val = ContentDisposition::new(Some("test.rs".into()), None);
    header.insert(val.header_name(), val.header_value());
    let content = ContentDisposition::try_from(&header)?;
    assert_eq!(val, content);
    Ok(())
}



```

# Support [axum](https://crates.io/crates/axum) web framework
```toml
axum = "*"
webt = {version = "*", features = ["axum"]}
tokio = {version = "*", features = ["full"]}
```

```rust
use std::net::SocketAddr;
use axum::{routing::get, Router, Server};
use hyper::{HeaderMap, StatusCode};
use webt::{header::Bearer, BodyFile, ResponseError};
pub async fn download_toml() -> Result<BodyFile, ResponseError> {
    let file = BodyFile::open("Cargo.toml")?;
    Ok(file)
}
pub async fn get_bearer(header_map: HeaderMap) -> (StatusCode, String) {
    let bearer = Bearer::try_from(&header_map).ok();
    (StatusCode::OK, format!("{:?}", bearer))
}
#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/download/toml", get(download_toml))
        .route("/get/bearer", get(get_bearer));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3080));

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

```
