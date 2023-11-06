use hyper::header::{HeaderValue};
fn main() {
    let header = HeaderValue::from_static("hello wor  ");
    println!("{:?}", header.to_str());
}