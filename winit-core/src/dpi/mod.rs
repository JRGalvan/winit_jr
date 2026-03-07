//! Tests and documentation is pending!

pub mod insets;
pub mod pixel;
pub mod position;
pub mod size;
pub mod unit;

pub use insets::{Insets, LogicalInsets, PhysicalInsets};
pub use pixel::Pixel;
pub use position::{LogicalPosition, PhysicalPosition, Position};
pub use size::{LogicalSize, PhysicalSize, Size};
pub use unit::{LogicalUnit, PhysicalUnit, Unit};

#[inline]
pub(crate) fn is_valid_scale_factor(scale_factor: f64) -> bool {
    scale_factor.is_sign_positive() && scale_factor.is_normal()
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_from_vec2_for_ {
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
