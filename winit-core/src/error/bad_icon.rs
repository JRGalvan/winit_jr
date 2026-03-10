use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum BadIcon {
    #[error(
        "The length of the `rgba` argument ({byte_count}) isn't divisible by 4, making it impossible to interpret as 32bpp RGBA pixels."
    )]
    ByteCountNotDivisibleBy4 { byte_count: usize },

    #[error(
        "The specified dimensions ({width}x{height}) don't match the number of pixels supplied by the `rgba` argument ({pixel_count}). For those dimensions, the expected pixel count is {width_x_height}."
    )]
    DimensionsVsPixelCount {
        width: u32,
        height: u32,
        width_x_height: usize,
        pixel_count: usize,
    },
}
