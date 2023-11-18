
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
