use crate::tablet::TabletToolAngle;

/// Describes the force of a touch event
#[derive(Debug, Clone, Copy, PartialEq)]
#[doc(alias = "Pressure")]
pub enum Force {
    Calibrated { force: f64, max_possible_force: f64 },
    Normalized(f64),
}

impl Force {
    pub fn normalized(&self, angle: Option<TabletToolAngle>) -> f64 {
        match self {
            Force::Calibrated {
                force,
                max_possible_force,
            } => {
                let force = match angle {
                    Some(TabletToolAngle { altitude, .. }) => force / altitude.sin(),
                    None => *force,
                };
                force / max_possible_force
            }
            Force::Normalized(force) => *force,
        }
    }
}
