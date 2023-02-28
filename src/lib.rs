use std::error::Error;
use std::fmt;
use std::fs::read_dir;
use std::io::ErrorKind::{NotFound, PermissionDenied};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_dir() {
        let lib = list_dir("./")
            .unwrap()
            .iter()
            .any(|file| file.path() == "./Cargo.toml");

        assert!(lib);
    }
}
