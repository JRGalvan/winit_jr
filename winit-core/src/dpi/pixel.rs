pub trait Pixel: Copy + Into<f64> {
    fn from_f64(f: f64) -> Self;
    fn cast<P: Pixel>(self) -> P {
        P::from_f64(self.into())
    }
}

macro_rules! impl_pixel_for_integer_type {
    ($($t:ty),*) => {
        $(
            impl Pixel for $t {
                fn from_f64(f: f64) -> Self {
                    f.round() as $t
                }
            }
        )*
    };
}

macro_rules! impl_pixel_for_float_type {
    ($($t:ty),*) => {
        $(
            impl Pixel for $t {
                fn from_f64(f: f64) -> Self {
                    f as $t
                }
            }
        )*
    };
}

impl_pixel_for_integer_type!(u8, u16, u32, i8, i16, i32);
impl_pixel_for_float_type!(f32, f64);
