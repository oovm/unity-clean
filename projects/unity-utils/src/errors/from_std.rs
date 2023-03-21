use crate::{errors::UnityErrorKind, UnityError};

impl From<std::io::Error> for UnityError {
    fn from(error: std::io::Error) -> Self {
        Self { kind: Box::new(UnityErrorKind::IOError { path: "".to_string(), message: error.to_string() }) }
    }
}
