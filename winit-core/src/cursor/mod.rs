mod cursor_animation;
mod cursor_image;
mod custom_cursor;

pub const MAX_CURSOR_SIZE: u16 = 2048;

pub use cursor_animation::CursorAnimation;
pub use cursor_image::CursorImage;
pub use custom_cursor::{CustomCursor, CustomCursorSource};
