use super::pixel::{Pixel, validate_scale_factor};

/// A logical pixel unit.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct LogicalUnit<P>(pub P);

impl<P> LogicalUnit<P> {
    /// Represents a maximum logical unit that is equal to [`f64::MAX`].
    pub const MAX: LogicalUnit<f64> = LogicalUnit::new(f64::MAX);
    /// Represents a minimum logical unit that is equal to [`f64::MIN`].
    pub const MIN: LogicalUnit<f64> = LogicalUnit::new(f64::MIN);
    /// Represents a logical unit of `0_f64`.
    pub const ZERO: LogicalUnit<f64> = LogicalUnit::new(0.0);

    #[inline]
    pub const fn new(v: P) -> Self {
        LogicalUnit(v)
    }
}

impl<P: Pixel> LogicalUnit<P> {
    #[inline]
    pub fn from_physical<T: Into<PhysicalUnit<X>>, X: Pixel>(
        physical: T,
        scale_factor: f64,
    ) -> Self {
        physical.into().to_logical(scale_factor)
    }

    #[inline]
    pub fn to_physical<X: Pixel>(&self, scale_factor: f64) -> PhysicalUnit<X> {
        assert!(validate_scale_factor(scale_factor));
        PhysicalUnit::new(self.0.into() * scale_factor).cast()
    }

    #[inline]
    pub fn cast<X: Pixel>(&self) -> LogicalUnit<X> {
        LogicalUnit(self.0.cast())
    }
}

impl<P: Pixel, X: Pixel> From<X> for LogicalUnit<P> {
    fn from(v: X) -> LogicalUnit<P> {
        LogicalUnit::new(v.cast())
    }
}

impl<P: Pixel> From<LogicalUnit<P>> for u8 {
    fn from(v: LogicalUnit<P>) -> u8 {
        v.0.cast()
    }
}

impl<P: Pixel> From<LogicalUnit<P>> for u16 {
    fn from(v: LogicalUnit<P>) -> u16 {
        v.0.cast()
    }
}

impl<P: Pixel> From<LogicalUnit<P>> for u32 {
    fn from(v: LogicalUnit<P>) -> u32 {
        v.0.cast()
    }
}

impl<P: Pixel> From<LogicalUnit<P>> for i8 {
    fn from(v: LogicalUnit<P>) -> i8 {
        v.0.cast()
    }
}

impl<P: Pixel> From<LogicalUnit<P>> for i16 {
    fn from(v: LogicalUnit<P>) -> i16 {
        v.0.cast()
    }
}

impl<P: Pixel> From<LogicalUnit<P>> for i32 {
    fn from(v: LogicalUnit<P>) -> i32 {
        v.0.cast()
    }
}

impl<P: Pixel> From<LogicalUnit<P>> for f32 {
    fn from(v: LogicalUnit<P>) -> f32 {
        v.0.cast()
    }
}

impl<P: Pixel> From<LogicalUnit<P>> for f64 {
    fn from(v: LogicalUnit<P>) -> f64 {
        v.0.cast()
    }
}

/// A physical pixel unit.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct PhysicalUnit<P>(pub P);

impl<P> PhysicalUnit<P> {
    /// Represents a maximum physical unit that is equal to [`f64::MAX`].
    pub const MAX: PhysicalUnit<f64> = PhysicalUnit::new(f64::MAX);
    /// Represents a minimum physical unit that is equal to [`f64::MIN`].
    pub const MIN: PhysicalUnit<f64> = PhysicalUnit::new(f64::MIN);
    /// Represents a physical unit of `0_f64`.
    pub const ZERO: PhysicalUnit<f64> = PhysicalUnit::new(0.0);

    #[inline]
    pub const fn new(v: P) -> Self {
        PhysicalUnit(v)
    }
}

impl<P: Pixel> PhysicalUnit<P> {
    #[inline]
    pub fn from_logical<T: Into<LogicalUnit<X>>, X: Pixel>(logical: T, scale_factor: f64) -> Self {
        logical.into().to_physical(scale_factor)
    }

    #[inline]
    pub fn to_logical<X: Pixel>(&self, scale_factor: f64) -> LogicalUnit<X> {
        assert!(validate_scale_factor(scale_factor));
        LogicalUnit::new(self.0.into() / scale_factor).cast()
    }

    #[inline]
    pub fn cast<X: Pixel>(&self) -> PhysicalUnit<X> {
        PhysicalUnit(self.0.cast())
    }
}

impl<P: Pixel, X: Pixel> From<X> for PhysicalUnit<P> {
    fn from(v: X) -> PhysicalUnit<P> {
        PhysicalUnit::new(v.cast())
    }
}

impl<P: Pixel> From<PhysicalUnit<P>> for u8 {
    fn from(v: PhysicalUnit<P>) -> u8 {
        v.0.cast()
    }
}

impl<P: Pixel> From<PhysicalUnit<P>> for u16 {
    fn from(v: PhysicalUnit<P>) -> u16 {
        v.0.cast()
    }
}

impl<P: Pixel> From<PhysicalUnit<P>> for u32 {
    fn from(v: PhysicalUnit<P>) -> u32 {
        v.0.cast()
    }
}

impl<P: Pixel> From<PhysicalUnit<P>> for i8 {
    fn from(v: PhysicalUnit<P>) -> i8 {
        v.0.cast()
    }
}

impl<P: Pixel> From<PhysicalUnit<P>> for i16 {
    fn from(v: PhysicalUnit<P>) -> i16 {
        v.0.cast()
    }
}

impl<P: Pixel> From<PhysicalUnit<P>> for i32 {
    fn from(v: PhysicalUnit<P>) -> i32 {
        v.0.cast()
    }
}

impl<P: Pixel> From<PhysicalUnit<P>> for f32 {
    fn from(v: PhysicalUnit<P>) -> f32 {
        v.0.cast()
    }
}

impl<P: Pixel> From<PhysicalUnit<P>> for f64 {
    fn from(v: PhysicalUnit<P>) -> f64 {
        v.0.cast()
    }
}

/// A pixel unit that's either physical or logical.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PixelUnit {
    Physical(PhysicalUnit<i32>),
    Logical(LogicalUnit<f64>),
}

impl PixelUnit {
    /// Represents a maximum logical unit that is equal to [`f64::MAX`].
    pub const MAX: PixelUnit = PixelUnit::Logical(LogicalUnit::new(f64::MAX));
    /// Represents a minimum logical unit that is equal to [`f64::MIN`].
    pub const MIN: PixelUnit = PixelUnit::Logical(LogicalUnit::new(f64::MIN));
    /// Represents a logical unit of `0_f64`.
    pub const ZERO: PixelUnit = PixelUnit::Logical(LogicalUnit::new(0.0));

    pub fn new<S: Into<PixelUnit>>(unit: S) -> PixelUnit {
        unit.into()
    }

    pub fn to_logical<P: Pixel>(&self, scale_factor: f64) -> LogicalUnit<P> {
        match *self {
            PixelUnit::Physical(unit) => unit.to_logical(scale_factor),
            PixelUnit::Logical(unit) => unit.cast(),
        }
    }

    pub fn to_physical<P: Pixel>(&self, scale_factor: f64) -> PhysicalUnit<P> {
        match *self {
            PixelUnit::Physical(unit) => unit.cast(),
            PixelUnit::Logical(unit) => unit.to_physical(scale_factor),
        }
    }
}

impl<P: Pixel> From<PhysicalUnit<P>> for PixelUnit {
    #[inline]
    fn from(unit: PhysicalUnit<P>) -> PixelUnit {
        PixelUnit::Physical(unit.cast())
    }
}

impl<P: Pixel> From<LogicalUnit<P>> for PixelUnit {
    #[inline]
    fn from(unit: LogicalUnit<P>) -> PixelUnit {
        PixelUnit::Logical(unit.cast())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logical_unit() {
        let log_unit = LogicalUnit::new(1.0);
        assert_eq!(log_unit.to_physical::<u32>(1.0), PhysicalUnit::new(1));
        assert_eq!(log_unit.to_physical::<u32>(2.0), PhysicalUnit::new(2));
        assert_eq!(log_unit.cast::<u32>(), LogicalUnit::new(1));
        assert_eq!(log_unit, LogicalUnit::from_physical(PhysicalUnit::new(1.0), 1.0));
        assert_eq!(log_unit, LogicalUnit::from_physical(PhysicalUnit::new(2.0), 2.0));
        assert_eq!(LogicalUnit::from(2.0), LogicalUnit::new(2.0));

        let x: f64 = log_unit.into();
        assert_eq!(x, 1.0);
    }

    #[test]
    fn test_physical_unit() {
        assert_eq!(PhysicalUnit::from_logical(LogicalUnit::new(1.0), 1.0), PhysicalUnit::new(1));
        assert_eq!(PhysicalUnit::from_logical(LogicalUnit::new(2.0), 0.5), PhysicalUnit::new(1));
        assert_eq!(PhysicalUnit::from(2.0), PhysicalUnit::new(2.0));

        let x: f64 = PhysicalUnit::new(1).into();
        assert_eq!(x, 1.0);
    }
}
