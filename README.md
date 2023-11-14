
# Example
```rust
use webt::byte_serialize;
use webt::header::{ContentDisposition, HeaderKey};
fn main() {
    let file_name = byte_serialize("文件file.rs");

    let dis = ContentDisposition::try_from(format!("attachment; filename={}", file_name)).unwrap();
    assert_eq!(dis.file_name(), Some("文件file.rs"));

    let dis =
        ContentDisposition::try_from(format!("attachment; filename=\"{}\"", file_name)).unwrap();
    assert_eq!(dis.file_name(), Some("文件file.rs"));

    let dis1 = ContentDisposition::new_with_filename("文件file.rs");
    assert_eq!(dis1, dis);
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

# Support [reqewst](https://crates.io/crates/reqwest)  HTTP client Multipart

```toml
webt = { version = "*", features = ["request"] }
tokio = { version = "*", features = ["full"] }
reqwest = { version = "*", features = ["multipart"]}
axum = {version = "*", features = ["multipart", "headers"]}
tower-http = { version = "0.4.4", features = ["cors"] }
```
```rust

use std::path::PathBuf;
use webt::request::FormData;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut form_data = FormData::new();
    let path = PathBuf::from("Cargo.toml");
    form_data.append("file", path)?;
    form_data.append("text", "hello world").unwrap();

    let response = client
        .post("http://localhost:80/upload")
        .multipart(form_data.into())
        .send()
        .await?
        .text()
        .await?;
    assert_eq!(response.as_str(), "ok");
    Ok(())
}

```

```rust
use axum::{
    extract::{DefaultBodyLimit, Multipart},
    http::Method,
    routing::post,
    Router, Server,
};
use hyper::StatusCode;
use std::net::SocketAddr;
use tower_http::cors::Any;
#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/upload", post(upload))
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST, Method::DELETE]),
        )
        .layer(DefaultBodyLimit::max(70 * 1024 * 1024));
    let addr = SocketAddr::from(([127, 0, 0, 1], 80));

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
async fn upload(mut part: Multipart) -> (StatusCode, String) {
    while let Ok(Some(field)) = part.next_field().await {
        match field.name() {
            Some("text") => {
                let text = field.text().await.unwrap();
                if text.as_str() != "hello world" {
                    return (StatusCode::OK, format!("text: {}", text));
                }
            }
            Some("file") => match field.file_name() {
                Some("Cargo.toml") => (),
                _ => {
                    return (
                        StatusCode::OK,
                        "error, filename is not Cargo.toml".to_string(),
                    )
                }
            },
            _ => return (StatusCode::OK, "Unreadable field".into()),,
        }
    }
    (StatusCode::OK, "ok".into())
}


```