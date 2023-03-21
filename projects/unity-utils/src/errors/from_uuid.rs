use crate::{errors::UnityErrorKind, UnityError};
use uuid::Error;

impl From<Error> for UnityError {
    fn from(error: Error) -> Self {
        Self { kind: Box::new(UnityErrorKind::SyntaxError { text: "".to_string(), message: error.to_string() }) }
    }
}
