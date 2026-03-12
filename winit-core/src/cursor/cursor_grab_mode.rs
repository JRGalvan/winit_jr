#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CursorGrabMode {
    None,
    Confined,
    Locked,
}
