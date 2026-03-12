use crate::tablet::TabletToolTilt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TabletToolAngle {
    pub altitude: f64,
    pub azimuth: f64,
}

impl Default for TabletToolAngle {
    fn default() -> Self {
        Self {
            altitude: std::f64::consts::FRAC_2_PI,
            azimuth: 0.,
        }
    }
}

impl TabletToolAngle {
    pub fn tilt(self) -> TabletToolTilt {
        // See <https://www.w3.org/TR/2024/WD-pointerevents3-20240326/#converting-between-tiltx-tilty-and-altitudeangle-azimuthangle>.

        use std::f64::consts::*;

        const PI_0_5: f64 = FRAC_PI_2;
        const PI_1_5: f64 = 3. * FRAC_PI_2;
        const PI_2: f64 = 2. * PI;

        let mut x = 0.;
        let mut y = 0.;

        if self.altitude == 0. {
            if self.azimuth == 0. || self.azimuth == PI_2 {
                x = FRAC_PI_2;
            } else if self.azimuth == PI_0_5 {
                y = FRAC_PI_2;
            } else if self.azimuth == PI {
                x = -FRAC_PI_2;
            } else if self.azimuth == PI_1_5 {
                y = -FRAC_PI_2;
            } else if self.azimuth > 0. && self.azimuth < PI_0_5 {
                x = FRAC_PI_2;
                y = FRAC_PI_2;
            } else if self.azimuth > PI_0_5 && self.azimuth < PI {
                x = -FRAC_PI_2;
                y = FRAC_PI_2;
            } else if self.azimuth > PI && self.azimuth < PI_1_5 {
                x = -FRAC_PI_2;
                y = -FRAC_PI_2;
            } else if self.azimuth > PI_1_5 && self.azimuth < PI_2 {
                x = FRAC_PI_2;
                y = -FRAC_PI_2;
            }
        }

        if self.altitude != 0. {
            let altitude = self.altitude.tan();

            x = f64::atan(f64::cos(self.azimuth) / altitude);
            y = f64::atan(f64::sin(self.azimuth) / altitude);
        }

        TabletToolTilt {
            x: x.to_degrees().round() as i8,
            y: y.to_degrees().round() as i8,
        }
    }
}
