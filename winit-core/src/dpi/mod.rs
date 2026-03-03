//! # DPI
//!
//! ## Why should I care about UI scaling?
//!
//! Modern computer screens don't have a consistent relationship between resolution and size.
//! 1920x1080 is a common resolution for both desktop and mobile screens, despite mobile screens
//! typically being less than a quarter the size of their desktop counterparts. Moreover, neither
//! desktop nor mobile screens have consistent resolutions within their own size classes - common
//! mobile screens range from below 720p to above 1440p, and desktop screens range from 720p to 5K
//! and beyond.
//!
//! Given that, it's a mistake to assume that 2D content will only be displayed on screens with
//! a consistent pixel density. If you were to render a 96-pixel-square image on a 1080p screen and
//! then render the same image on a similarly-sized 4K screen, the 4K rendition would only take up
//! about a quarter of the physical space as it did on the 1080p screen. That issue is especially
//! problematic with text rendering, where quarter-sized text becomes a significant legibility
//! problem.
//!
//! Failure to account for the scale factor can create a significantly degraded user experience.
//! Most notably, it can make users feel like they have bad eyesight, which will potentially cause
//! them to think about growing elderly, resulting in them having an existential crisis. Once users
//! enter that state, they will no longer be focused on your application.
//!
//! ## How should I handle it?
//!
//! The solution to this problem is to account for the device's *scale factor*. The scale factor is
//! the factor UI elements should be scaled by to be consistent with the rest of the user's system -
//! for example, a button that's usually 50 pixels across would be 100 pixels across on a device
//! with a scale factor of `2.0`, or 75 pixels across with a scale factor of `1.5`.
//!
//! Many UI systems, such as CSS, expose DPI-dependent units like [points] or [picas]. That's
//! usually a mistake since there's no consistent mapping between the scale factor and the screen's
//! actual DPI. Unless printing to a physical medium, you should work in scaled pixels rather
//! than any DPI-dependent units.
//!
//! ### Position and Size types
//!
//! The [`PhysicalPosition`] / [`PhysicalSize`] / [`PhysicalUnit`] types correspond with the actual
//! pixels on the device, and the [`LogicalPosition`] / [`LogicalSize`] / [`LogicalUnit`] types
//! correspond to the physical pixels divided by the scale factor.
//!
//! The position and size types are generic over their exact pixel type, `P`, to allow the
//! API to have integer precision where appropriate (e.g. most window manipulation functions) and
//! floating precision when necessary (e.g. logical sizes for fractional scale factors and touch
//! input). If `P` is a floating-point type, please do not cast the values with `as {int}`. Doing so
//! will truncate the fractional part of the float rather than properly round to the nearest
//! integer. Use the provided `cast` function or [`From`]/[`Into`] conversions, which handle the
//! rounding properly. Note that precision loss will still occur when rounding from a float to an
//! int, although rounding lessens the problem.
//!
//! [points]: https://en.wikipedia.org/wiki/Point_(typography)
//! [picas]: https://en.wikipedia.org/wiki/Pica_(typography)

pub mod pixel;
pub mod position;
pub mod size;
pub mod unit;

pub use pixel::{Pixel, validate_scale_factor};
pub use position::{Insets, LogicalInsets, LogicalPosition, PhysicalInsets, PhysicalPosition, Position};
pub use size::{LogicalSize, PhysicalSize, Size};
pub use unit::{LogicalUnit, PhysicalUnit, PixelUnit};

macro_rules! vec2_from_impls {
    ($t:ident, $a:ident, $b:ident) => {
        impl<P: Pixel, X: Pixel> From<(X, X)> for $t<P> {
            fn from(($a, $b): (X, X)) -> Self {
                Self::new($a.cast(), $b.cast())
            }
        }

        impl<P: Pixel, X: Pixel> From<$t<P>> for (X, X) {
            fn from(p: $t<P>) -> Self {
                (p.$a.cast(), p.$b.cast())
            }
        }

        impl<P: Pixel, X: Pixel> From<[X; 2]> for $t<P> {
            fn from([$a, $b]: [X; 2]) -> Self {
                Self::new($a.cast(), $b.cast())
            }
        }

        impl<P: Pixel, X: Pixel> From<$t<P>> for [X; 2] {
            fn from(p: $t<P>) -> Self {
                [p.$a.cast(), p.$b.cast()]
            }
        }
    };
}

pub(crate) use vec2_from_impls;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn ensure_attrs_do_not_panic() {
        let _ = format!("{:?}", LogicalPosition::<u32>::default().clone());
        HashSet::new().insert(LogicalPosition::<u32>::default());

        let _ = format!("{:?}", PhysicalPosition::<u32>::default().clone());
        HashSet::new().insert(PhysicalPosition::<u32>::default());

        let _ = format!("{:?}", LogicalSize::<u32>::default().clone());
        HashSet::new().insert(LogicalSize::<u32>::default());

        let _ = format!("{:?}", PhysicalSize::<u32>::default().clone());
        HashSet::new().insert(PhysicalSize::<u32>::default());

        let _ = format!("{:?}", Size::Physical((1, 2).into()).clone());
        let _ = format!("{:?}", Position::Physical((1, 2).into()).clone());
    }

    #[test]
    fn ensure_copy_trait() {
        fn is_copy<T: Copy>() {}

        is_copy::<LogicalUnit<i32>>();
        is_copy::<PhysicalUnit<f64>>();
        is_copy::<PixelUnit>();

        is_copy::<LogicalSize<i32>>();
        is_copy::<PhysicalSize<f64>>();
        is_copy::<Size>();

        is_copy::<LogicalPosition<i32>>();
        is_copy::<PhysicalPosition<f64>>();
        is_copy::<Position>();
    }

    #[test]
    fn ensure_partial_eq_trait() {
        fn is_partial_eq<T: PartialEq>() {}

        is_partial_eq::<LogicalUnit<i32>>();
        is_partial_eq::<PhysicalUnit<f64>>();
        is_partial_eq::<PixelUnit>();

        is_partial_eq::<LogicalSize<i32>>();
        is_partial_eq::<PhysicalSize<f64>>();
        is_partial_eq::<Size>();

        is_partial_eq::<LogicalPosition<i32>>();
        is_partial_eq::<PhysicalPosition<f64>>();
        is_partial_eq::<Position>();
    }
}
