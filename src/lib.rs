use std::error::Error;
use std::fmt;
use std::fs::{read_dir, File};
use std::io::ErrorKind::{NotFound, PermissionDenied};
use std::io::Read;

#[derive(Debug)]
pub enum ListError {
    NotFound,
    PermDenied,
    Unknown,
}

impl Error for ListError {}

impl fmt::Display for ListError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListError::PermDenied => write!(formatter, "Access Denied"),
            ListError::NotFound => write!(formatter, "Not Found"),
            ListError::Unknown => write!(formatter, "Unknown Error"),
        }
    }
}

pub struct ListedFile {
    is_dir: bool,
    path: String,
}

impl ListedFile {
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    pub fn path(&self) -> String {
        self.path.to_string()
    }
}

pub fn list_dir(path: &str) -> Result<Vec<ListedFile>, ListError> {
    match read_dir(path) {
        Ok(dir) => {
            let mut vec = Vec::new();
            for file in dir {
                let _file = file.unwrap();
                let path = _file.path().into_os_string().into_string().unwrap();
                let is_dir = _file.file_type().unwrap().is_dir();

                vec.push(ListedFile { is_dir, path })
            }

            Ok(vec)
        }
        Err(err) => match err.kind() {
            NotFound => Err(ListError::NotFound),
            PermissionDenied => Err(ListError::PermDenied),
            _ => Err(ListError::Unknown),
        },
    }
}

pub fn read_file(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffers = Vec::new();
    file.read_to_end(&mut buffers)?;

    Ok(buffers)
}

#[cfg(test)]
mod tests {
    use std::str::from_utf8;

    use super::*;

    #[test]
    fn test_list_dir() {
        let lib = list_dir("./")
            .unwrap()
            .iter()
            .any(|file| file.path() == "./Cargo.toml");

        assert!(lib);
    }

    #[test]
    fn test_read_file() {
        let cargo = read_file("./Cargo.toml").unwrap();
        let content = from_utf8(&cargo).unwrap();
        assert!(content.starts_with("[package]"));

        let none = read_file("./not.exists").unwrap_err();
        assert_eq!(none.kind(), NotFound);
    }
}
