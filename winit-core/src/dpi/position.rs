use super::pixel::{Pixel, validate_scale_factor};
use super::vec2_from_impls;

/// A position represented in logical pixels.
///
/// The position is stored as floats, so please be careful. Casting floats to integers truncates the
/// fractional part, which can cause noticeable issues. To help with that, an `Into<(i32, i32)>`
/// implementation is provided which does the rounding for you.
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
    pub fn to_physical<X: Pixel>(&self, scale_factor: f64) -> PhysicalPosition<X> {
        assert!(validate_scale_factor(scale_factor));
        let x = self.x.into() * scale_factor;
        let y = self.y.into() * scale_factor;
        PhysicalPosition::new(x, y).cast()
    }

    #[inline]
    pub fn cast<X: Pixel>(&self) -> LogicalPosition<X> {
        LogicalPosition { x: self.x.cast(), y: self.y.cast() }
    }
}

vec2_from_impls!(LogicalPosition, x, y);

/// A position represented in physical pixels.
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
    pub fn to_logical<X: Pixel>(&self, scale_factor: f64) -> LogicalPosition<X> {
        assert!(validate_scale_factor(scale_factor));
        let x = self.x.into() / scale_factor;
        let y = self.y.into() / scale_factor;
        LogicalPosition::new(x, y).cast()
    }

    #[inline]
    pub fn cast<X: Pixel>(&self) -> PhysicalPosition<X> {
        PhysicalPosition { x: self.x.cast(), y: self.y.cast() }
    }
}

vec2_from_impls!(PhysicalPosition, x, y);

/// A position that's either physical or logical.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Position {
    Physical(PhysicalPosition<i32>),
    Logical(LogicalPosition<f64>),
}

impl Position {
    pub fn new<S: Into<Position>>(position: S) -> Position {
        position.into()
    }

    pub fn to_logical<P: Pixel>(&self, scale_factor: f64) -> LogicalPosition<P> {
        match *self {
            Position::Physical(position) => position.to_logical(scale_factor),
            Position::Logical(position) => position.cast(),
        }
    }

    pub fn to_physical<P: Pixel>(&self, scale_factor: f64) -> PhysicalPosition<P> {
        match *self {
            Position::Physical(position) => position.cast(),
            Position::Logical(position) => position.to_physical(scale_factor),
        }
    }
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

/// The logical distance between the edges of two rectangles.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct LogicalInsets<P> {
    /// The distance to the top edge.
    pub top: P,
    /// The distance to the left edge.
    pub left: P,
    /// The distance to the bottom edge.
    pub bottom: P,
    /// The distance to the right edge.
    pub right: P,
}

impl<P> LogicalInsets<P> {
    #[inline]
    pub const fn new(top: P, left: P, bottom: P, right: P) -> Self {
        Self { top, left, bottom, right }
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
    pub fn to_physical<X: Pixel>(&self, scale_factor: f64) -> PhysicalInsets<X> {
        assert!(validate_scale_factor(scale_factor));
        let top = self.top.into() * scale_factor;
        let left = self.left.into() * scale_factor;
        let bottom = self.bottom.into() * scale_factor;
        let right = self.right.into() * scale_factor;
        PhysicalInsets::new(top, left, bottom, right).cast()
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
}

/// The physical distance between the edges of two rectangles.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct PhysicalInsets<P> {
    /// The distance to the top edge.
    pub top: P,
    /// The distance to the left edge.
    pub left: P,
    /// The distance to the bottom edge.
    pub bottom: P,
    /// The distance to the right edge.
    pub right: P,
}

impl<P> PhysicalInsets<P> {
    #[inline]
    pub const fn new(top: P, left: P, bottom: P, right: P) -> Self {
        Self { top, left, bottom, right }
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
    pub fn to_logical<X: Pixel>(&self, scale_factor: f64) -> LogicalInsets<X> {
        assert!(validate_scale_factor(scale_factor));
        let top = self.top.into() / scale_factor;
        let left = self.left.into() / scale_factor;
        let bottom = self.bottom.into() / scale_factor;
        let right = self.right.into() / scale_factor;
        LogicalInsets::new(top, left, bottom, right).cast()
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
}

/// Insets that are either physical or logical.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Insets {
    Physical(PhysicalInsets<u32>),
    Logical(LogicalInsets<f64>),
}

impl Insets {
    pub fn new<S: Into<Self>>(insets: S) -> Self {
        insets.into()
    }

    pub fn to_logical<P: Pixel>(&self, scale_factor: f64) -> LogicalInsets<P> {
        match *self {
            Self::Physical(insets) => insets.to_logical(scale_factor),
            Self::Logical(insets) => insets.cast(),
        }
    }

    pub fn to_physical<P: Pixel>(&self, scale_factor: f64) -> PhysicalInsets<P> {
        match *self {
            Self::Physical(insets) => insets.cast(),
            Self::Logical(insets) => insets.to_physical(scale_factor),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logical_position() {
        let log_pos = LogicalPosition::new(1.0, 2.0);
        assert_eq!(log_pos.to_physical::<u32>(1.0), PhysicalPosition::new(1, 2));
        assert_eq!(log_pos.to_physical::<u32>(2.0), PhysicalPosition::new(2, 4));
        assert_eq!(log_pos.cast::<u32>(), LogicalPosition::new(1, 2));
        assert_eq!(log_pos, LogicalPosition::from_physical(PhysicalPosition::new(1.0, 2.0), 1.0));
        assert_eq!(log_pos, LogicalPosition::from_physical(PhysicalPosition::new(2.0, 4.0), 2.0));
        assert_eq!(LogicalPosition::from((2.0, 2.0)), LogicalPosition::new(2.0, 2.0));
        assert_eq!(LogicalPosition::from([2.0, 3.0]), LogicalPosition::new(2.0, 3.0));

        let x: (f64, f64) = log_pos.into();
        assert_eq!(x, (1.0, 2.0));
        let x: [f64; 2] = log_pos.into();
        assert_eq!(x, [1.0, 2.0]);
    }

    #[test]
    fn test_physical_position() {
        assert_eq!(
            PhysicalPosition::from_logical(LogicalPosition::new(1.0, 2.0), 1.0),
            PhysicalPosition::new(1, 2)
        );
        assert_eq!(
            PhysicalPosition::from_logical(LogicalPosition::new(2.0, 4.0), 0.5),
            PhysicalPosition::new(1, 2)
        );
        assert_eq!(PhysicalPosition::from((2.0, 2.0)), PhysicalPosition::new(2.0, 2.0));
        assert_eq!(PhysicalPosition::from([2.0, 3.0]), PhysicalPosition::new(2.0, 3.0));

        let x: (f64, f64) = PhysicalPosition::new(1, 2).into();
        assert_eq!(x, (1.0, 2.0));
        let x: [f64; 2] = PhysicalPosition::new(1, 2).into();
        assert_eq!(x, [1.0, 2.0]);
    }

    #[test]
    fn test_position() {
        assert_eq!(
            Position::new(PhysicalPosition::new(1, 2)),
            Position::Physical(PhysicalPosition::new(1, 2))
        );
        assert_eq!(
            Position::new(LogicalPosition::new(1.0, 2.0)),
            Position::Logical(LogicalPosition::new(1.0, 2.0))
        );

        assert_eq!(
            Position::new(PhysicalPosition::new(1, 2)).to_logical::<f64>(1.0),
            LogicalPosition::new(1.0, 2.0)
        );
        assert_eq!(
            Position::new(PhysicalPosition::new(1, 2)).to_logical::<f64>(2.0),
            LogicalPosition::new(0.5, 1.0)
        );
        assert_eq!(
            Position::new(LogicalPosition::new(1.0, 2.0)).to_logical::<f64>(1.0),
            LogicalPosition::new(1.0, 2.0)
        );

        assert_eq!(
            Position::new(PhysicalPosition::new(1, 2)).to_physical::<u32>(1.0),
            PhysicalPosition::new(1, 2)
        );
        assert_eq!(
            Position::new(PhysicalPosition::new(1, 2)).to_physical::<u32>(2.0),
            PhysicalPosition::new(1, 2)
        );
        assert_eq!(
            Position::new(LogicalPosition::new(1.0, 2.0)).to_physical::<u32>(1.0),
            PhysicalPosition::new(1, 2)
        );
        assert_eq!(
            Position::new(LogicalPosition::new(1.0, 2.0)).to_physical::<u32>(2.0),
            PhysicalPosition::new(2, 4)
        );
    }
}
