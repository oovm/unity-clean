use fast_walker::utils::to_unix_path;
use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    path::Path,
};
mod from_std;
mod from_trash;
mod from_uuid;

pub type UnityResult<T> = Result<T, UnityError>;

#[derive(Clone)]
pub struct UnityError {
    kind: Box<UnityErrorKind>,
}

#[derive(Debug, Clone)]
pub enum UnityErrorKind {
    CustomError { message: String },
    SyntaxError { text: String, message: String },
    IOError { path: String, message: String },
}

impl Error for UnityError {}

impl Debug for UnityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

impl Display for UnityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for UnityErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnityErrorKind::CustomError { message } => {
                write!(f, "{}", message)
            }
            UnityErrorKind::IOError { path, message } => match path.as_str() {
                "" => write!(f, "{}", message),
                _ => write!(f, "{} at {}", message, path),
            },
            UnityErrorKind::SyntaxError { text, message } => {
                write!(f, "{} at {}", message, text)
            }
        }
    }
}

impl UnityError {
    pub fn custom_error<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self { kind: Box::new(UnityErrorKind::CustomError { message: message.into() }) }
    }
    pub fn io_error<P, S>(path: P, message: S) -> Self
    where
        P: AsRef<Path>,
        S: Into<String>,
    {
        let path = to_unix_path(path.as_ref());
        Self { kind: Box::new(UnityErrorKind::IOError { path, message: message.into() }) }
    }
}
