use webt::byte_serialize;
use webt::header::{ContentDisposition, HeaderKey};
fn main() {
    let file_name = byte_serialize("文件file.rs");

    let dis = ContentDisposition::try_from(format!("attachment; filename={}", file_name)).unwrap();
    assert_eq!(dis.file_name(), Some("文件file.rs"));

    let dis =
        ContentDisposition::try_from(format!("attachment; filename=\"{}\"", file_name)).unwrap();
    assert_eq!(dis.file_name(), Some("文件file.rs"));

    // let dis1 = ContentDisposition::new_with_filename("文件file.rs");
    // assert_eq!(dis1, dis);
}

#[allow(unused)]
fn header_value_test() -> Result<(), webt::header::HeaderParserError> {
    use axum::http::header::{HeaderMap, CONTENT_DISPOSITION};
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
