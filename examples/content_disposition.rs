use url::form_urlencoded::byte_serialize;
use webt::header::content::ContentDisposition;
fn main() {
    let file_name: String = byte_serialize("文件file.rs".as_bytes()).collect();

    let dis = ContentDisposition::try_from(format!("attachment; filename={}", file_name)).unwrap();
    assert_eq!(dis.file_name(), Some("文件file.rs"));

    let dis =
        ContentDisposition::try_from(format!("attachment; filename=\"{}\"", file_name)).unwrap();
    assert_eq!(dis.file_name(), Some("文件file.rs"));

    let dis1 = ContentDisposition::new("文件file.rs");
    assert_eq!(dis1, dis);
}
#[allow(unused)]
mod test {

    use hyper::header::{HeaderMap, HeaderValue, CONTENT_DISPOSITION};
    use webt::header::{content::ContentDisposition, HeaderParserError};
    #[test]
    fn header_value_test() -> Result<(), HeaderParserError> {
        let mut header = HeaderMap::new();
        assert_eq!(
            Err(HeaderParserError::MissingHeaderValue(CONTENT_DISPOSITION)),
            ContentDisposition::try_from(&header)
        );
        header.insert(
            CONTENT_DISPOSITION,
            HeaderValue::from_static("attachment; filename=test.rs"),
        );
        let content = ContentDisposition::try_from(&header)?;
        assert_eq!(content.file_name(), Some("test.rs"));
        Ok(())
    }
}
