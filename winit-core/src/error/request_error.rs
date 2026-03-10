use crate::error::NotSupportedError;
use crate::error::OsError;
use thiserror;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum RequestError<OE: OsError> {
    #[error(transparent)]
    NotSupported(#[from] NotSupportedError),

    #[error("The request was ignored")]
    Ignored,

    #[error(transparent)]
    Os(#[from] OE),
}
