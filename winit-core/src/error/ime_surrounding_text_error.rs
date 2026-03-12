use thiserror;

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone, Hash)]
pub enum ImeSurroundingTextError {
    #[error("text exceeds maximum length")]
    TextTooLong,
    #[error("cursor is not at a valid text index")]
    CursorBadPosition,
    #[error("anchor is not at a valid text index")]
    AnchorBadPosition,
}
