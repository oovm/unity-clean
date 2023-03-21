use crate::{errors::UnityErrorKind, UnityError};
use trash::Error;

impl From<Error> for UnityError {
    fn from(error: Error) -> Self {
        Self { kind: Box::new(UnityErrorKind::from(error)) }
    }
}

impl From<Error> for UnityErrorKind {
    fn from(error: Error) -> Self {
        match error {
            Error::Unknown { description } => UnityErrorKind::CustomError { message: description },
            #[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
            Error::FileSystem { .. } => {}
            Error::TargetedRoot => {
                unimplemented!()
            }
            Error::CouldNotAccess { .. } => {
                unimplemented!()
            }
            Error::CanonicalizePath { .. } => {
                unimplemented!()
            }
            Error::ConvertOsString { .. } => {
                unimplemented!()
            }
            Error::RestoreCollision { .. } => {
                unimplemented!()
            }
            Error::RestoreTwins { .. } => {
                unimplemented!()
            }
        }
    }
}
