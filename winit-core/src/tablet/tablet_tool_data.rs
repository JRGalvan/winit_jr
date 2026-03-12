use crate::event::Force;
use crate::tablet::TabletToolAngle;
use crate::tablet::TabletToolTilt;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct TabletToolData {
    pub force: Option<Force>,
    pub tangential_force: Option<f32>,
    pub twist: Option<u16>,
    pub tilt: Option<TabletToolTilt>,
    pub angle: Option<TabletToolAngle>,
}

impl TabletToolData {
    pub fn tilt(self) -> Option<TabletToolTilt> {
        if let Some(tilt) = self.tilt {
            Some(tilt)
        } else {
            self.angle.map(TabletToolAngle::tilt)
        }
    }

    pub fn angle(self) -> Option<TabletToolAngle> {
        if let Some(angle) = self.angle {
            Some(angle)
        } else {
            self.tilt.map(TabletToolTilt::angle)
        }
    }
}
