use crate::error::ImeSurroundingTextError;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct ImeSurroundingText {
    text: String,
    cursor: usize,
    anchor: usize,
}

impl ImeSurroundingText {
    pub const MAX_TEXT_BYTES: usize = 4000;

    pub fn new(
        text: String,
        cursor: usize,
        anchor: usize,
    ) -> Result<Self, ImeSurroundingTextError> {
        let text = if text.len() < 4000 {
            text
        } else {
            return Err(ImeSurroundingTextError::TextTooLong);
        };

        let cursor = if text.is_char_boundary(cursor) && cursor <= text.len() {
            cursor
        } else {
            return Err(ImeSurroundingTextError::CursorBadPosition);
        };

        let anchor = if text.is_char_boundary(anchor) && anchor <= text.len() {
            anchor
        } else {
            return Err(ImeSurroundingTextError::AnchorBadPosition);
        };

        Ok(Self {
            text,
            cursor,
            anchor,
        })
    }

    pub fn into_text(self) -> String {
        self.text
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn anchor(&self) -> usize {
        self.anchor
    }
}
