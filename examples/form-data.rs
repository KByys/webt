fn main() {
    #[cfg(feature = "request")]
    {
        use reqwest::multipart::Form;
        use std::fs::File;
        use std::path::PathBuf;
        use webt::request::FormData;
        let mut form_data = FormData::new();

        form_data.append("text", "Some text contents").unwrap();

        form_data
            .append("file", File::open("Cargo.toml").unwrap())
            .unwrap();

        form_data
            .append("file", PathBuf::from("Cargo.toml"))
            .unwrap();

        form_data
            .append("file", ("hello.txt", "Hello World".as_bytes()))
            .unwrap();

        let _form: Form = form_data.into();
    }
}
