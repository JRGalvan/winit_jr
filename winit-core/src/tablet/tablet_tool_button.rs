use crate::event::MouseButton;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TabletToolButton {
    Contact,
    Barrel,
    Other(u16),
}

impl From<TabletToolButton> for Option<MouseButton> {
    fn from(tool: TabletToolButton) -> Self {
        Some(match tool {
            TabletToolButton::Contact => MouseButton::Left,
            TabletToolButton::Barrel => MouseButton::Right,
            TabletToolButton::Other(1) => MouseButton::Middle,
            TabletToolButton::Other(3) => MouseButton::Back,
            TabletToolButton::Other(4) => MouseButton::Forward,
            TabletToolButton::Other(_) => return None,
        })
    }
}
