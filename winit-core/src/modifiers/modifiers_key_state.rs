#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ModifiersKeyState {
    Pressed,
    #[default]
    Unknown,
}
