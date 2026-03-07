use super::Pixel;
use super::is_valid_scale_factor;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct LogicalInsets<P> {
    pub top: P,
    pub left: P,
    pub bottom: P,
    pub right: P,
}

impl<P> LogicalInsets<P> {
    #[inline]
    pub const fn new(top: P, left: P, bottom: P, right: P) -> Self {
        Self {
            top,
            left,
            bottom,
            right,
        }
    }
}

impl<P: Pixel> LogicalInsets<P> {
    #[inline]
    pub fn from_physical<T: Into<PhysicalInsets<X>>, X: Pixel>(
        physical: T,
        scale_factor: f64,
    ) -> Self {
        physical.into().to_logical(scale_factor)
    }

    #[inline]
    pub fn cast<X: Pixel>(&self) -> LogicalInsets<X> {
        LogicalInsets {
            top: self.top.cast(),
            left: self.left.cast(),
            bottom: self.bottom.cast(),
            right: self.right.cast(),
        }
    }

    #[inline]
    pub fn to_physical<X: Pixel>(&self, scale_factor: f64) -> PhysicalInsets<X> {
        debug_assert!(is_valid_scale_factor(scale_factor));
        let top = self.top.into() * scale_factor;
        let left = self.left.into() * scale_factor;
        let bottom = self.bottom.into() * scale_factor;
        let right = self.right.into() * scale_factor;
        PhysicalInsets::new(top, left, bottom, right).cast()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct PhysicalInsets<P> {
    pub top: P,
    pub left: P,
    pub bottom: P,
    pub right: P,
}

impl<P> PhysicalInsets<P> {
    #[inline]
    pub const fn new(top: P, left: P, bottom: P, right: P) -> Self {
        Self {
            top,
            left,
            bottom,
            right,
        }
    }
}

impl<P: Pixel> PhysicalInsets<P> {
    #[inline]
    pub fn from_logical<T: Into<LogicalInsets<X>>, X: Pixel>(
        logical: T,
        scale_factor: f64,
    ) -> Self {
        logical.into().to_physical(scale_factor)
    }

    #[inline]
    pub fn cast<X: Pixel>(&self) -> PhysicalInsets<X> {
        PhysicalInsets {
            top: self.top.cast(),
            left: self.left.cast(),
            bottom: self.bottom.cast(),
            right: self.right.cast(),
        }
    }

    #[inline]
    pub fn to_logical<X: Pixel>(&self, scale_factor: f64) -> LogicalInsets<X> {
        debug_assert!(is_valid_scale_factor(scale_factor));
        let top = self.top.into() / scale_factor;
        let left = self.left.into() / scale_factor;
        let bottom = self.bottom.into() / scale_factor;
        let right = self.right.into() / scale_factor;
        LogicalInsets::new(top, left, bottom, right).cast()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Insets {
    Physical(PhysicalInsets<u32>),
    Logical(LogicalInsets<f64>),
}

impl<P: Pixel> From<PhysicalInsets<P>> for Insets {
    #[inline]
    fn from(insets: PhysicalInsets<P>) -> Self {
        Self::Physical(insets.cast())
    }
}

impl<P: Pixel> From<LogicalInsets<P>> for Insets {
    #[inline]
    fn from(insets: LogicalInsets<P>) -> Self {
        Self::Logical(insets.cast())
    }
}

impl Insets {
    pub fn new<S: Into<Self>>(insets: S) -> Self {
        insets.into()
    }

    pub fn to_logical<P: Pixel>(&self, scale_factor: f64) -> LogicalInsets<P> {
        match self {
            Self::Physical(insets) => insets.to_logical(scale_factor),
            Self::Logical(insets) => insets.cast(),
        }
    }

    pub fn to_physical<P: Pixel>(&self, scale_factor: f64) -> PhysicalInsets<P> {
        match self {
            Self::Physical(insets) => insets.cast(),
            Self::Logical(insets) => insets.to_physical(scale_factor),
        }
    }
}
