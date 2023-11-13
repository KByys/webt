use std::fs::File;

use hyper::StatusCode;
use serde::ser::SerializeStruct;
fn main() {
}

#[derive(Debug)]
struct ErrorData {
    code: i32,
    message: String,
}
