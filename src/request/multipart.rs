


use std::path::{Path, PathBuf};

use fdir::{FileInfo, Info};
use reqwest::multipart::*;
pub trait TryIntoMultipart {
    type Error;
    fn part(self) -> Result<Part, Self::Error>;
}

macro_rules! derive_str_part {
    ($($ty:ty), *) => {
        $(
            impl TryIntoMultipart for $ty {
                type Error = ();
                fn part(self) -> Result<Part, Self::Error> {
                    Ok(Part::text(self.to_owned()))
                }
            }
        )*
    };
}

macro_rules! derive_path_part {
    ($($ty:ty), *) => {
        $(
            impl TryIntoMultipart for $ty {
                type Error = std::io::Error;
                fn part(self) -> Result<Part, Self::Error> {
                    let file = std::fs::File::open(self)?;
                    std::fs::File::part(file)
                }
            }
        )*
    };
}
macro_rules! derive_bytes_part {
    ($($ty:ty), *) => {
        $(
            impl TryIntoMultipart for $ty {
                type Error = ();
                fn part(self) -> Result<Part, Self::Error> {
                    let (filename, bytes) = self;
                    Ok(Part::bytes(bytes.to_owned()).file_name(filename.to_owned()))
                }
            }
        )*
    };
}

derive_path_part!(PathBuf, &Path, &PathBuf);
derive_str_part!(&str, String, &String);
derive_bytes_part!(
    (&str, Vec<u8>),
    (&str, &Vec<u8>),
    (&str, &[u8]),
    (&String, Vec<u8>),
    (&String, &Vec<u8>),
    (&String, &[u8]),
    (String, Vec<u8>),
    (String, &Vec<u8>),
    (String, &[u8])
);

impl TryIntoMultipart for std::fs::File {
    type Error = std::io::Error;

    fn part(self) -> Result<Part, Self::Error> {
        let file = FileInfo::try_from(self)?;
        let filename = file
            .file_name()
            .expect("Any file should have a file name")
            .to_str()
            .expect("File name is not valid Unicode")
            .to_owned();

        let part = Part::bytes(std::fs::read(file.as_path())?).file_name(filename);
        Ok(part)
    }
}

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

    /// Append a form data
    ///
    /// # Examples
    /// ```
    /// use webt::request::FormData;
    /// use std::fs::File;
    /// use std::path::PathBuf;
    /// use reqwest::multipart::Form;
    /// let mut form_data = FormData::new();
    ///
    /// form_data.append("text", "Some text contents").unwrap();
    ///
    /// form_data.append("file", File::open("Cargo.toml").unwrap()).unwrap();
    ///
    /// form_data.append("file", PathBuf::from("Cargo.toml")).unwrap();
    ///
    /// form_data.append("file", ("hello.txt", "Hello World".as_bytes())).unwrap();
    ///
    /// let form: Form = form_data.into();
    /// ```
    pub fn append<T: TryIntoMultipart>(&mut self, name: &str, value: T) -> Result<(), T::Error> {
        self.data.push((name.to_owned(), value.part()?));
        Ok(())
    }
}
