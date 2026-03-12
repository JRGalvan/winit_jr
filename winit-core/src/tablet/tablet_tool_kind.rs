#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TabletToolKind {
    #[default]
    Pen,
    Eraser,
    Brush,
    Pencil,
    Airbrush,
    Finger,
    Mouse,
    Lens,
}
