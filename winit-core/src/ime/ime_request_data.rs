use crate::dpi::{Position, Size};
use crate::ime::{ImeHint, ImePurpose, ImeSurroundingText};

#[non_exhaustive]
#[derive(Debug, PartialEq, Clone, Default)]
pub struct ImeRequestData {
    pub hint_and_purpose: Option<(ImeHint, ImePurpose)>,
    pub cursor_area: Option<(Position, Size)>,
    pub surrounding_text: Option<ImeSurroundingText>,
}

impl ImeRequestData {
    pub fn with_hint_and_purpose(self, hint: ImeHint, purpose: ImePurpose) -> Self {
        Self {
            hint_and_purpose: Some((hint, purpose)),
            ..self
        }
    }

    pub fn with_cursor_area(self, position: Position, size: Size) -> Self {
        Self {
            cursor_area: Some((position, size)),
            ..self
        }
    }

    pub fn with_surrounding_text(self, surrounding_text: ImeSurroundingText) -> Self {
        Self {
            surrounding_text: Some(surrounding_text),
            ..self
        }
    }
}
