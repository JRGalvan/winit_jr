use super::pixel::{Pixel, validate_scale_factor};
use super::vec2_from_impls;

/// A size represented in logical pixels.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct LogicalSize<P> {
    pub width: P,
    pub height: P,
}

impl<P> LogicalSize<P> {
    #[inline]
    pub const fn new(width: P, height: P) -> Self {
        LogicalSize { width, height }
    }
}

impl<P: Pixel> LogicalSize<P> {
    #[inline]
    pub fn from_physical<T: Into<PhysicalSize<X>>, X: Pixel>(
        physical: T,
        scale_factor: f64,
    ) -> Self {
        physical.into().to_logical(scale_factor)
    }

    #[inline]
    pub fn to_physical<X: Pixel>(&self, scale_factor: f64) -> PhysicalSize<X> {
        assert!(validate_scale_factor(scale_factor));
        let width = self.width.into() * scale_factor;
        let height = self.height.into() * scale_factor;
        PhysicalSize::new(width, height).cast()
    }

    #[inline]
    pub fn cast<X: Pixel>(&self) -> LogicalSize<X> {
        LogicalSize { width: self.width.cast(), height: self.height.cast() }
    }
}

vec2_from_impls!(LogicalSize, width, height);

/// A size represented in physical pixels.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Hash)]
pub struct PhysicalSize<P> {
    pub width: P,
    pub height: P,
}

impl<P> PhysicalSize<P> {
    #[inline]
    pub const fn new(width: P, height: P) -> Self {
        PhysicalSize { width, height }
    }
}

impl<P: Pixel> PhysicalSize<P> {
    #[inline]
    pub fn from_logical<T: Into<LogicalSize<X>>, X: Pixel>(logical: T, scale_factor: f64) -> Self {
        logical.into().to_physical(scale_factor)
    }

    #[inline]
    pub fn to_logical<X: Pixel>(&self, scale_factor: f64) -> LogicalSize<X> {
        assert!(validate_scale_factor(scale_factor));
        let width = self.width.into() / scale_factor;
        let height = self.height.into() / scale_factor;
        LogicalSize::new(width, height).cast()
    }

    #[inline]
    pub fn cast<X: Pixel>(&self) -> PhysicalSize<X> {
        PhysicalSize { width: self.width.cast(), height: self.height.cast() }
    }
}

vec2_from_impls!(PhysicalSize, width, height);

/// A size that's either physical or logical.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Size {
    Physical(PhysicalSize<u32>),
    Logical(LogicalSize<f64>),
}

impl Size {
    pub fn new<S: Into<Size>>(size: S) -> Size {
        size.into()
    }

    pub fn to_logical<P: Pixel>(&self, scale_factor: f64) -> LogicalSize<P> {
        match *self {
            Size::Physical(size) => size.to_logical(scale_factor),
            Size::Logical(size) => size.cast(),
        }
    }

    pub fn to_physical<P: Pixel>(&self, scale_factor: f64) -> PhysicalSize<P> {
        match *self {
            Size::Physical(size) => size.cast(),
            Size::Logical(size) => size.to_physical(scale_factor),
        }
    }

    pub fn clamp<S: Into<Size>>(input: S, min: S, max: S, scale_factor: f64) -> Size {
        let (input, min, max) = (
            input.into().to_physical::<f64>(scale_factor),
            min.into().to_physical::<f64>(scale_factor),
            max.into().to_physical::<f64>(scale_factor),
        );

        let width = input.width.clamp(min.width, max.width);
        let height = input.height.clamp(min.height, max.height);

        PhysicalSize::new(width, height).into()
    }
}

impl<P: Pixel> From<PhysicalSize<P>> for Size {
    #[inline]
    fn from(size: PhysicalSize<P>) -> Size {
        Size::Physical(size.cast())
    }
}

impl<P: Pixel> From<LogicalSize<P>> for Size {
    #[inline]
    fn from(size: LogicalSize<P>) -> Size {
        Size::Logical(size.cast())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logical_size() {
        let log_size = LogicalSize::new(1.0, 2.0);
        assert_eq!(log_size.to_physical::<u32>(1.0), PhysicalSize::new(1, 2));
        assert_eq!(log_size.to_physical::<u32>(2.0), PhysicalSize::new(2, 4));
        assert_eq!(log_size.cast::<u32>(), LogicalSize::new(1, 2));
        assert_eq!(log_size, LogicalSize::from_physical(PhysicalSize::new(1.0, 2.0), 1.0));
        assert_eq!(log_size, LogicalSize::from_physical(PhysicalSize::new(2.0, 4.0), 2.0));
        assert_eq!(LogicalSize::from((2.0, 2.0)), LogicalSize::new(2.0, 2.0));
        assert_eq!(LogicalSize::from([2.0, 3.0]), LogicalSize::new(2.0, 3.0));

        let x: (f64, f64) = log_size.into();
        assert_eq!(x, (1.0, 2.0));
        let x: [f64; 2] = log_size.into();
        assert_eq!(x, [1.0, 2.0]);
    }

    #[test]
    fn test_physical_size() {
        assert_eq!(
            PhysicalSize::from_logical(LogicalSize::new(1.0, 2.0), 1.0),
            PhysicalSize::new(1, 2)
        );
        assert_eq!(
            PhysicalSize::from_logical(LogicalSize::new(2.0, 4.0), 0.5),
            PhysicalSize::new(1, 2)
        );
        assert_eq!(PhysicalSize::from((2.0, 2.0)), PhysicalSize::new(2.0, 2.0));
        assert_eq!(PhysicalSize::from([2.0, 3.0]), PhysicalSize::new(2.0, 3.0));

        let x: (f64, f64) = PhysicalSize::new(1, 2).into();
        assert_eq!(x, (1.0, 2.0));
        let x: [f64; 2] = PhysicalSize::new(1, 2).into();
        assert_eq!(x, [1.0, 2.0]);
    }

    #[test]
    fn test_size() {
        assert_eq!(Size::new(PhysicalSize::new(1, 2)), Size::Physical(PhysicalSize::new(1, 2)));
        assert_eq!(
            Size::new(LogicalSize::new(1.0, 2.0)),
            Size::Logical(LogicalSize::new(1.0, 2.0))
        );

        assert_eq!(
            Size::new(PhysicalSize::new(1, 2)).to_logical::<f64>(1.0),
            LogicalSize::new(1.0, 2.0)
        );
        assert_eq!(
            Size::new(PhysicalSize::new(1, 2)).to_logical::<f64>(2.0),
            LogicalSize::new(0.5, 1.0)
        );
        assert_eq!(
            Size::new(LogicalSize::new(1.0, 2.0)).to_logical::<f64>(1.0),
            LogicalSize::new(1.0, 2.0)
        );

        assert_eq!(
            Size::new(PhysicalSize::new(1, 2)).to_physical::<u32>(1.0),
            PhysicalSize::new(1, 2)
        );
        assert_eq!(
            Size::new(PhysicalSize::new(1, 2)).to_physical::<u32>(2.0),
            PhysicalSize::new(1, 2)
        );
        assert_eq!(
            Size::new(LogicalSize::new(1.0, 2.0)).to_physical::<u32>(1.0),
            PhysicalSize::new(1, 2)
        );
        assert_eq!(
            Size::new(LogicalSize::new(1.0, 2.0)).to_physical::<u32>(2.0),
            PhysicalSize::new(2, 4)
        );

        let small = Size::Physical((1, 2).into());
        let medium = Size::Logical((3, 4).into());
        let medium_physical = Size::new(medium.to_physical::<u32>(1.0));
        let large = Size::Physical((5, 6).into());
        assert_eq!(Size::clamp(medium, small, large, 1.0), medium_physical);
        assert_eq!(Size::clamp(small, medium, large, 1.0), medium_physical);
        assert_eq!(Size::clamp(large, small, medium, 1.0), medium_physical);
    }
}
