use super::Pixel;
use super::is_valid_scale_factor;
use crate::impl_from_vec2_for_;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct LogicalPosition<P> {
    pub x: P,
    pub y: P,
}

impl<P> LogicalPosition<P> {
    #[inline]
    pub const fn new(x: P, y: P) -> Self {
        LogicalPosition { x, y }
    }
}

impl<P: Pixel> LogicalPosition<P> {
    #[inline]
    pub fn from_physical<T: Into<PhysicalPosition<X>>, X: Pixel>(
        physical: T,
        scale_factor: f64,
    ) -> Self {
        physical.into().to_logical(scale_factor)
    }

    #[inline]
    pub fn cast<X: Pixel>(&self) -> LogicalPosition<X> {
        LogicalPosition {
            x: self.x.cast(),
            y: self.y.cast(),
        }
    }

    #[inline]
    pub fn to_physical<X: Pixel>(&self, scale_factor: f64) -> PhysicalPosition<X> {
        debug_assert!(is_valid_scale_factor(scale_factor));
        let x = self.x.into() * scale_factor;
        let y = self.y.into() * scale_factor;
        PhysicalPosition::new(x, y).cast()
    }
}

impl_from_vec2_for_!(LogicalPosition, x, y);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct PhysicalPosition<P> {
    pub x: P,
    pub y: P,
}

impl<P> PhysicalPosition<P> {
    #[inline]
    pub const fn new(x: P, y: P) -> Self {
        PhysicalPosition { x, y }
    }
}

impl<P: Pixel> PhysicalPosition<P> {
    #[inline]
    pub fn from_logical<T: Into<LogicalPosition<X>>, X: Pixel>(
        logical: T,
        scale_factor: f64,
    ) -> Self {
        logical.into().to_physical(scale_factor)
    }

    #[inline]
    pub fn cast<X: Pixel>(&self) -> PhysicalPosition<X> {
        PhysicalPosition {
            x: self.x.cast(),
            y: self.y.cast(),
        }
    }

    #[inline]
    pub fn to_logical<X: Pixel>(&self, scale_factor: f64) -> LogicalPosition<X> {
        debug_assert!(is_valid_scale_factor(scale_factor));
        let x = self.x.into() / scale_factor;
        let y = self.y.into() / scale_factor;
        LogicalPosition::new(x, y).cast()
    }
}

impl_from_vec2_for_!(PhysicalPosition, x, y);

/// A position that's either physical or logical.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Position {
    Physical(PhysicalPosition<i32>),
    Logical(LogicalPosition<f64>),
}

impl<P: Pixel> From<PhysicalPosition<P>> for Position {
    #[inline]
    fn from(position: PhysicalPosition<P>) -> Position {
        Position::Physical(position.cast())
    }
}

impl<P: Pixel> From<LogicalPosition<P>> for Position {
    #[inline]
    fn from(position: LogicalPosition<P>) -> Position {
        Position::Logical(position.cast())
    }
}

impl Position {
    pub fn new<S: Into<Position>>(position: S) -> Position {
        position.into()
    }

    pub fn to_logical<P: Pixel>(&self, scale_factor: f64) -> LogicalPosition<P> {
        match self {
            Position::Physical(position) => position.to_logical(scale_factor),
            Position::Logical(position) => position.cast(),
        }
    }

    pub fn to_physical<P: Pixel>(&self, scale_factor: f64) -> PhysicalPosition<P> {
        match self {
            Position::Physical(position) => position.cast(),
            Position::Logical(position) => position.to_physical(scale_factor),
        }
    }
}
