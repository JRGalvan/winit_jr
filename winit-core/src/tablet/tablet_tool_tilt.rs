use crate::tablet::TabletToolAngle;
use std::cell::LazyCell;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TabletToolTilt {
    pub x: i8,
    pub y: i8,
}

impl TabletToolTilt {
    pub fn angle(self) -> TabletToolAngle {
        use std::f64::consts::*;

        const PI_0_5: f64 = FRAC_PI_2;
        const PI_1_5: f64 = 3. * FRAC_PI_2;
        const PI_2: f64 = 2. * PI;

        let x = LazyCell::new(|| f64::from(self.x).to_radians());
        let y = LazyCell::new(|| f64::from(self.y).to_radians());

        let mut azimuth = 0.;

        if self.x == 0 {
            match self.y.cmp(&0) {
                Ordering::Greater => azimuth = PI_0_5,
                Ordering::Less => azimuth = PI_1_5,
                Ordering::Equal => (),
            }
        } else if self.y == 0 {
            if self.x < 0 {
                azimuth = PI;
            }
        } else if self.x.abs() == 90 || self.y.abs() == 90 {
            azimuth = 0.;
        } else {
            azimuth = f64::atan2(y.tan(), x.tan());

            if azimuth < 0. {
                azimuth += PI_2;
            }
        }

        let altitude;

        if self.x.abs() == 90 || self.y.abs() == 90 {
            altitude = 0.;
        } else if self.x == 0 {
            altitude = PI_0_5 - y.abs();
        } else if self.y == 0 {
            altitude = PI_0_5 - x.abs();
        } else {
            altitude = f64::atan(1. / f64::sqrt(x.tan().powi(2) + y.tan().powi(2)));
        }

        TabletToolAngle { altitude, azimuth }
    }
}
