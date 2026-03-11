use thiserror;

#[derive(thiserror::Error, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BadAnimation {
    #[error("No cursors supplied")]
    Empty,

    #[error("A supplied cursor is an animation")]
    Animation,
}
