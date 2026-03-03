pub trait Pixel: Copy + Into<f64> {
    fn from_f64(f: f64) -> Self;
    fn cast<P: Pixel>(self) -> P {
        P::from_f64(self.into())
    }
}

impl Pixel for u8 {
    fn from_f64(f: f64) -> Self {
        f.round() as u8
    }
}

impl Pixel for u16 {
    fn from_f64(f: f64) -> Self {
        f.round() as u16
    }
}

impl Pixel for u32 {
    fn from_f64(f: f64) -> Self {
        f.round() as u32
    }
}

impl Pixel for i8 {
    fn from_f64(f: f64) -> Self {
        f.round() as i8
    }
}

impl Pixel for i16 {
    fn from_f64(f: f64) -> Self {
        f.round() as i16
    }
}

impl Pixel for i32 {
    fn from_f64(f: f64) -> Self {
        f.round() as i32
    }
}

impl Pixel for f32 {
    fn from_f64(f: f64) -> Self {
        f as f32
    }
}

impl Pixel for f64 {
    fn from_f64(f: f64) -> Self {
        f
    }
}

/// Checks that the scale factor is a normal positive `f64`.
///
/// All functions that take a scale factor assert that this will return `true`. If you're sourcing
/// scale factors from anywhere other than winit, it's recommended to validate them using this
/// function before passing them to winit; otherwise, you risk panics.
#[inline]
pub fn validate_scale_factor(scale_factor: f64) -> bool {
    scale_factor.is_sign_positive() && scale_factor.is_normal()
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_pixel_int_impl {
        ($($name:ident => $ty:ty),*) => {$(
            #[test]
            fn $name() {
                assert_eq!(<$ty as Pixel>::from_f64(37.0), 37);
                assert_eq!(<$ty as Pixel>::from_f64(37.4), 37);
                assert_eq!(<$ty as Pixel>::from_f64(37.5), 38);
                assert_eq!(<$ty as Pixel>::from_f64(37.9), 38);

                assert_eq!(<$ty as Pixel>::cast::<u8>(37), 37);
                assert_eq!(<$ty as Pixel>::cast::<u16>(37), 37);
                assert_eq!(<$ty as Pixel>::cast::<u32>(37), 37);
                assert_eq!(<$ty as Pixel>::cast::<i8>(37), 37);
                assert_eq!(<$ty as Pixel>::cast::<i16>(37), 37);
                assert_eq!(<$ty as Pixel>::cast::<i32>(37), 37);
            }
        )*};
    }

    test_pixel_int_impl! {
        test_pixel_int_u8  => u8,
        test_pixel_int_u16 => u16,
        test_pixel_int_u32 => u32,
        test_pixel_int_i8  => i8,
        test_pixel_int_i16 => i16
    }

    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr $(,)?) => {
            assert!(($a - $b).abs() < 0.001, "{} is not approximately equal to {}", $a, $b);
        };
    }

    macro_rules! test_pixel_float_impl {
        ($($name:ident => $ty:ty),*) => {$(
            #[test]
            fn $name() {
                assert_approx_eq!(<$ty as Pixel>::from_f64(37.0), 37.0);
                assert_approx_eq!(<$ty as Pixel>::from_f64(37.4), 37.4);
                assert_approx_eq!(<$ty as Pixel>::from_f64(37.5), 37.5);
                assert_approx_eq!(<$ty as Pixel>::from_f64(37.9), 37.9);

                assert_eq!(<$ty as Pixel>::cast::<u8>(37.0), 37);
                assert_eq!(<$ty as Pixel>::cast::<u8>(37.4), 37);
                assert_eq!(<$ty as Pixel>::cast::<u8>(37.5), 38);

                assert_eq!(<$ty as Pixel>::cast::<u16>(37.0), 37);
                assert_eq!(<$ty as Pixel>::cast::<u16>(37.4), 37);
                assert_eq!(<$ty as Pixel>::cast::<u16>(37.5), 38);

                assert_eq!(<$ty as Pixel>::cast::<u32>(37.0), 37);
                assert_eq!(<$ty as Pixel>::cast::<u32>(37.4), 37);
                assert_eq!(<$ty as Pixel>::cast::<u32>(37.5), 38);

                assert_eq!(<$ty as Pixel>::cast::<i8>(37.0), 37);
                assert_eq!(<$ty as Pixel>::cast::<i8>(37.4), 37);
                assert_eq!(<$ty as Pixel>::cast::<i8>(37.5), 38);

                assert_eq!(<$ty as Pixel>::cast::<i16>(37.0), 37);
                assert_eq!(<$ty as Pixel>::cast::<i16>(37.4), 37);
                assert_eq!(<$ty as Pixel>::cast::<i16>(37.5), 38);
            }
        )*};
    }

    test_pixel_float_impl! {
        test_pixel_float_f32 => f32,
        test_pixel_float_f64 => f64
    }

    #[test]
    fn test_validate_scale_factor() {
        assert!(validate_scale_factor(1.0));
        assert!(validate_scale_factor(2.0));
        assert!(validate_scale_factor(3.0));
        assert!(validate_scale_factor(1.5));
        assert!(validate_scale_factor(0.5));

        assert!(!validate_scale_factor(0.0));
        assert!(!validate_scale_factor(-1.0));
        assert!(!validate_scale_factor(f64::INFINITY));
        assert!(!validate_scale_factor(f64::NAN));
        assert!(!validate_scale_factor(f64::NEG_INFINITY));
    }
}
