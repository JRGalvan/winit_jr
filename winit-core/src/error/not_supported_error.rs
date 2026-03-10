use thiserror;

#[derive(Debug, thiserror::Error)]
#[error("Operation is not supported: {reason}")]
pub struct NotSupportedError {
    reason: &'static str,
}

impl NotSupportedError {
    pub fn new(reason: &'static str) -> Self {
        Self { reason }
    }
}
