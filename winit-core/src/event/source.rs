use crate::event::{Force, MouseButton, PointerKind};
use crate::id::FingerId;
use crate::tablet::{TabletToolButton, TabletToolData, TabletToolKind};

#[derive(Clone, Debug, PartialEq)]
pub enum PointerSource {
    Mouse,
    Touch {
        finger_id: FingerId,
        force: Option<Force>,
    },
    TabletTool {
        kind: TabletToolKind,
        data: TabletToolData,
    },
    Unknown,
}

impl From<PointerSource> for PointerKind {
    fn from(source: PointerSource) -> Self {
        match source {
            PointerSource::Mouse => Self::Mouse,
            PointerSource::Touch { finger_id, .. } => Self::Touch(finger_id),
            PointerSource::TabletTool { kind, .. } => Self::TabletTool(kind),
            PointerSource::Unknown => Self::Unknown,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ButtonSource {
    Mouse(MouseButton),
    Touch {
        finger_id: FingerId,
        force: Option<Force>,
    },
    TabletTool {
        kind: TabletToolKind,
        button: TabletToolButton,
        data: TabletToolData,
    },
    Unknown(u16),
}

impl ButtonSource {
    pub fn mouse_button(self) -> Option<MouseButton> {
        match self {
            ButtonSource::Mouse(mouse) => Some(mouse),
            ButtonSource::Touch { .. } => Some(MouseButton::Left),
            ButtonSource::TabletTool { button, .. } => button.into(),
            ButtonSource::Unknown(_) => None,
        }
    }
}

impl From<MouseButton> for ButtonSource {
    fn from(mouse: MouseButton) -> Self {
        Self::Mouse(mouse)
    }
}
