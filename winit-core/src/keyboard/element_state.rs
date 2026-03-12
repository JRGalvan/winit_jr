#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum ElementState {
    Pressed,
    Released,
}

impl ElementState {
    pub fn is_pressed(self) -> bool {
        self == ElementState::Pressed
    }
}
