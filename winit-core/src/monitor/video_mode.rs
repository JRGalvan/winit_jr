use crate::dpi::PhysicalSize;
use std::fmt;
use std::num::{NonZeroU16, NonZeroU32};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VideoMode {
    pub(crate) size: PhysicalSize<u32>,
    pub(crate) bit_depth: Option<NonZeroU16>,
    pub(crate) refresh_rate_millihertz: Option<NonZeroU32>,
}

impl VideoMode {
    pub fn new(
        size: PhysicalSize<u32>,
        bit_depth: Option<NonZeroU16>,
        refresh_rate_millihertz: Option<NonZeroU32>,
    ) -> Self {
        Self {
            size,
            bit_depth,
            refresh_rate_millihertz,
        }
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn bit_depth(&self) -> Option<NonZeroU16> {
        self.bit_depth
    }

    pub fn refresh_rate_millihertz(&self) -> Option<NonZeroU32> {
        self.refresh_rate_millihertz
    }
}

impl fmt::Display for VideoMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}x{} {}{}",
            self.size.width,
            self.size.height,
            self.refresh_rate_millihertz
                .map(|rate| format!("@ {rate} mHz "))
                .unwrap_or_default(),
            self.bit_depth
                .map(|bit_depth| format!("({bit_depth} bpp)"))
                .unwrap_or_default(),
        )
    }
}
