use crate::id::FingerId;
use crate::tablet::TabletToolKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PointerKind {
    Mouse,
    Touch(FingerId),
    TabletTool(TabletToolKind),
    Unknown,
}
