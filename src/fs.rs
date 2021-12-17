use crate::error::FileError;

pub async fn read<S: AsRef<str>>(path: S) -> Result<Vec<u8>, FileError> {
    macroquad::prelude::load_file(path.as_ref()).await.map_err(FileError::Engine)
}

pub async fn read_to_string<S: AsRef<str>>(path: S) -> Result<String, FileError> {
    match read(path).await {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(string) => Ok(string),
            Err(err) => Err(FileError::String(err)),
        },
        Err(err) => Err(err),
    }
}