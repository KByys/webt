use std::{
    borrow::Cow,
    ffi::OsStr,
    path::Path,
};

use reqwest::multipart::*;

#[derive(Default)]
pub struct FormData {
    data: Vec<(String, Part)>,
}

impl From<FormData> for Form {
    fn from(value: FormData) -> Self {
        let mut form_data = Form::new();
        if !value.data.is_empty() {
            for (name, part) in value.data {
                form_data = form_data.part(name, part);
            }
        }
        form_data
    }
}

impl FormData {
    pub fn new() -> FormData {
        Self { data: Vec::new() }
    }
    pub fn append_text(&mut self, name: &str, value: impl Into<Cow<'static, str>>) {
        self.data.push((name.to_owned(), Part::text(value)))
    }
    pub fn append_file<P: AsRef<Path>>(&mut self, name: &str, path: P) -> std::io::Result<()> {
        let path = path.as_ref().to_path_buf();
        let bytes = std::fs::read(&path)?;
        self.append_bytes(name, path.file_name(), bytes);
        Ok(())
    }
    pub fn append_bytes<F, B>(&mut self, name: &str, filename: Option<F>, bytes: B)
    where
        F: AsRef<OsStr>,
        B: Into<Cow<'static, [u8]>>,
    {
        let part = Part::bytes(bytes);
        let part = if let Some(filename) = filename {
            part.file_name(filename.as_ref().to_string_lossy().to_string())
        } else {
            part
        };
        self.data.push((name.into(), part))
    }
}
