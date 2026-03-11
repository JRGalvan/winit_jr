use crate::cursor::CursorAnimation;
use crate::cursor::CursorImage;
use crate::error::BadAnimation;
use crate::error::BadImage;
use std::fmt;
use std::hash;
use std::time::Duration;

pub trait CustomCursor: Clone + fmt::Debug + PartialEq + Eq + hash::Hash + Send + Sync {
    fn is_animated(&self) -> bool;
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum CustomCursorSource<CC: CustomCursor> {
    Image(CursorImage),
    Animation(CursorAnimation<CC>),
    Url {
        hotspot_x: u16,
        hotspot_y: u16,
        url: String,
    },
}

impl<CC: CustomCursor> CustomCursorSource<CC> {
    pub fn from_rgba(
        rgba: Vec<u8>,
        width: u16,
        height: u16,
        hotspot_x: u16,
        hotspot_y: u16,
    ) -> Result<Self, BadImage> {
        CursorImage::from_rgba(rgba, width, height, hotspot_x, hotspot_y).map(Self::Image)
    }

    pub fn from_animation(duration: Duration, cursors: Vec<CC>) -> Result<Self, BadAnimation> {
        CursorAnimation::new(duration, cursors).map(Self::Animation)
    }
}
