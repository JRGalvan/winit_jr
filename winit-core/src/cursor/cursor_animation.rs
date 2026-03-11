use crate::cursor::CustomCursor;
use crate::error::BadAnimation;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CursorAnimation<CC: CustomCursor> {
    pub(crate) duration: Duration,
    pub(crate) cursors: Vec<CC>,
}

impl<CC: CustomCursor> CursorAnimation<CC> {
    pub fn new(duration: Duration, cursors: Vec<CC>) -> Result<Self, BadAnimation> {
        if cursors.is_empty() {
            return Err(BadAnimation::Empty);
        }

        if cursors.iter().any(|cursor| cursor.is_animated()) {
            return Err(BadAnimation::Animation);
        }

        Ok(Self { duration, cursors })
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn cursors(&self) -> &[CC] {
        self.cursors.as_slice()
    }

    pub fn into_raw(self) -> (Duration, Vec<CC>) {
        (self.duration, self.cursors)
    }
}
