use thiserror;

#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ImeRequestError {
    #[error("ime is not enabled.")]
    NotEnabled,
    #[error("ime is already enabled.")]
    AlreadyEnabled,
    #[error("ime is not supported.")]
    NotSupported,
}
