use super::Pixel;
use super::is_valid_scale_factor;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct LogicalUnit<P>(pub P);

impl<P> LogicalUnit<P> {
    #[inline]
    pub const fn new(v: P) -> Self {
        LogicalUnit(v)
    }

    pub const MIN: LogicalUnit<f64> = LogicalUnit::new(f64::MIN);
    pub const ZERO: LogicalUnit<f64> = LogicalUnit::new(0.0);
    pub const MAX: LogicalUnit<f64> = LogicalUnit::new(f64::MAX);
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
    pub fn cast<X: Pixel>(&self) -> LogicalUnit<X> {
        LogicalUnit(self.0.cast())
    }

    #[inline]
    pub fn to_physical<X: Pixel>(&self, scale_factor: f64) -> PhysicalUnit<X> {
        debug_assert!(is_valid_scale_factor(scale_factor));
        PhysicalUnit::new(self.0.into() * scale_factor).cast()
    }
}

impl<P: Pixel, X: Pixel> From<X> for LogicalUnit<P> {
    fn from(v: X) -> LogicalUnit<P> {
        LogicalUnit::new(v.cast())
    }
}

macro_rules! impl_from_logical_unit_for_numeric_type {
    ($($t:ty),*) => {
        $(
            impl<P: Pixel> From<LogicalUnit<P>> for $t {
                fn from(v: LogicalUnit<P>) -> $t {
                    v.0.cast()
                }
            }
        )*
    };
}

impl_from_logical_unit_for_numeric_type!(u8, u16, u32, i8, i16, i32, f32, f64);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct PhysicalUnit<P>(pub P);

impl<P> PhysicalUnit<P> {
    #[inline]
    pub const fn new(v: P) -> Self {
        PhysicalUnit(v)
    }

    pub const MIN: LogicalUnit<f64> = LogicalUnit::new(f64::MIN);
    pub const ZERO: LogicalUnit<f64> = LogicalUnit::new(0.0);
    pub const MAX: LogicalUnit<f64> = LogicalUnit::new(f64::MAX);
}

impl<P: Pixel> PhysicalUnit<P> {
    #[inline]
    pub fn from_logical<T: Into<LogicalUnit<X>>, X: Pixel>(logical: T, scale_factor: f64) -> Self {
        logical.into().to_physical(scale_factor)
    }

    #[inline]
    pub fn cast<X: Pixel>(&self) -> PhysicalUnit<X> {
        PhysicalUnit(self.0.cast())
    }

    #[inline]
    pub fn to_logical<X: Pixel>(&self, scale_factor: f64) -> LogicalUnit<X> {
        debug_assert!(is_valid_scale_factor(scale_factor));
        LogicalUnit::new(self.0.into() / scale_factor).cast()
    }
}

impl<P: Pixel, X: Pixel> From<X> for PhysicalUnit<P> {
    fn from(v: X) -> PhysicalUnit<P> {
        PhysicalUnit::new(v.cast())
    }
}

macro_rules! impl_from_physical_unit_for_numeric_type {
    ($($t:ty),*) => {
        $(
            impl<P: Pixel> From<PhysicalUnit<P>> for $t {
                fn from(v: PhysicalUnit<P>) -> $t {
                    v.0.cast()
                }
            }
        )*
    };
}

impl_from_physical_unit_for_numeric_type!(u8, u16, u32, i8, i16, i32, f32, f64);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Unit {
    Physical(PhysicalUnit<i32>),
    Logical(LogicalUnit<f64>),
}

impl<P: Pixel> From<PhysicalUnit<P>> for Unit {
    #[inline]
    fn from(unit: PhysicalUnit<P>) -> Unit {
        Unit::Physical(unit.cast())
    }
}

impl<P: Pixel> From<LogicalUnit<P>> for Unit {
    #[inline]
    fn from(unit: LogicalUnit<P>) -> Unit {
        Unit::Logical(unit.cast())
    }
}

impl Unit {
    pub fn new<S: Into<Unit>>(unit: S) -> Unit {
        unit.into()
    }

    pub const MAX: Unit = Unit::Logical(LogicalUnit::new(f64::MAX));
    pub const MIN: Unit = Unit::Logical(LogicalUnit::new(f64::MIN));
    pub const ZERO: Unit = Unit::Logical(LogicalUnit::new(0.0));

    pub fn to_logical<P: Pixel>(&self, scale_factor: f64) -> LogicalUnit<P> {
        match self {
            Unit::Physical(unit) => unit.to_logical(scale_factor),
            Unit::Logical(unit) => unit.cast(),
        }
    }

    pub fn to_physical<P: Pixel>(&self, scale_factor: f64) -> PhysicalUnit<P> {
        match self {
            Unit::Physical(unit) => unit.cast(),
            Unit::Logical(unit) => unit.to_physical(scale_factor),
        }
    }
}
