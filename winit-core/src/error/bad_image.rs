use crate::cursor::MAX_CURSOR_SIZE;
use thiserror;

#[derive(thiserror::Error, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BadImage {
    #[error(
        "The specified dimensions ({width}x{height}) are too large. The maximum is {MAX_CURSOR_SIZE}x{MAX_CURSOR_SIZE}."
    )]
    TooLarge { width: u16, height: u16 },

    #[error(
        "The length of the `rgba` argument ({byte_count}) isn't divisible by 4, making it impossible to interpret as 32bpp RGBA pixels."
    )]
    ByteCountNotDivisibleBy4 { byte_count: usize },

    #[error(
        "The specified dimensions ({width}x{height}) don't match the number of pixels supplied by the `rgba` argument ({pixel_count}). For those dimensions, the expected pixel count is {width_x_height}."
    )]
    DimensionsVsPixelCount {
        width: u16,
        height: u16,
        width_x_height: u64,
        pixel_count: u64,
    },

    #[error(
        "The specified hotspot ({hotspot_x}, {hotspot_y}) is outside the image bounds ({width}x{height})."
    )]
    HotspotOutOfBounds {
        width: u16,
        height: u16,
        hotspot_x: u16,
        hotspot_y: u16,
    },
}
