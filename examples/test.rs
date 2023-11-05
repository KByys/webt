use wtl::content_type::ContentType;
fn main() {
    let content_type = ContentType::from_extension(Some(".ico")).unwrap();
    println!("{}", content_type);
    println!("{}", ContentType::to_extension(content_type).unwrap());
}
