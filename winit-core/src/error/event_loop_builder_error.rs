use crate::error::NotSupportedError;
use crate::error::OsError;
use thiserror;

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum EventLoopBuilderError<OE: OsError> {
    #[error(
        "EventLoop can't be recreated, only a single instance of it is supported (for cross-platform compatibility)"
    )]
    RecreationAttempt,

    #[error("Exit Failure: {0}")]
    ExitFailure(i32),

    #[error(transparent)]
    Os(#[from] OE),

    #[error(transparent)]
    NotSupported(#[from] NotSupportedError),
}
