use crate::cursor::MAX_CURSOR_SIZE;
use crate::error::BadImage;

const PIXEL_SIZE: usize = 4;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct CursorImage {
    pub(crate) rgba: Vec<u8>,
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) hotspot_x: u16,
    pub(crate) hotspot_y: u16,
}

impl CursorImage {
    pub(crate) fn from_rgba(
        rgba: Vec<u8>,
        width: u16,
        height: u16,
        hotspot_x: u16,
        hotspot_y: u16,
    ) -> Result<Self, BadImage> {
        if width > MAX_CURSOR_SIZE || height > MAX_CURSOR_SIZE {
            return Err(BadImage::TooLarge { width, height });
        }

        if rgba.len() % PIXEL_SIZE != 0 {
            return Err(BadImage::ByteCountNotDivisibleBy4 {
                byte_count: rgba.len(),
            });
        }

        let pixel_count = (rgba.len() / PIXEL_SIZE) as u64;
        let width_x_height = width as u64 * height as u64;
        if pixel_count != width_x_height {
            return Err(BadImage::DimensionsVsPixelCount {
                width,
                height,
                width_x_height,
                pixel_count,
            });
        }

        if hotspot_x >= width || hotspot_y >= height {
            return Err(BadImage::HotspotOutOfBounds {
                width,
                height,
                hotspot_x,
                hotspot_y,
            });
        }

        Ok(CursorImage {
            rgba,
            width,
            height,
            hotspot_x,
            hotspot_y,
        })
    }

    pub fn buffer(&self) -> &[u8] {
        self.rgba.as_slice()
    }

    pub fn buffer_mut(&mut self) -> &mut [u8] {
        self.rgba.as_mut_slice()
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn hotspot_x(&self) -> u16 {
        self.hotspot_x
    }

    pub fn hotspot_y(&self) -> u16 {
        self.hotspot_y
    }
}
