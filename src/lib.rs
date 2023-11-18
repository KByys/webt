pub mod header;
// #[cfg(feature = "request")]
pub mod request;
pub mod response;

pub use response::*;

/// Change `' '` to `"%20"` not `'+'`
pub fn byte_serialize(input: impl AsRef<[u8]>) -> String {
    url::form_urlencoded::byte_serialize(input.as_ref())
        .collect::<String>()
        .replace('+', "%20")
}
