use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub type UnityResult<T> = Result<T, UnityError>;

#[derive(Clone)]
pub struct UnityError {
    kind: Box<UnityErrorKind>,
}

#[derive(Debug, Clone)]
pub enum UnityErrorKind {
    CustomError {
        message: String,
    }
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
        }
    }
}


impl UnityError {
    pub fn custom_error<S>(message: S) -> Self where S: Into<String> {
        Self {
            kind: Box::new(UnityErrorKind::CustomError {
                message: message.into(),
            })
        }
    }
}