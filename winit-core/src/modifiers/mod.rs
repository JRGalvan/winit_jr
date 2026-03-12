mod modifiers_key_state;
mod modifiers_keys;
mod modifiers_state;

pub use modifiers_key_state::ModifiersKeyState;
pub use modifiers_keys::ModifiersKeys;
pub use modifiers_state::ModifiersState;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Modifiers {
    pub(crate) state: ModifiersState,
    pub(crate) pressed_mods: ModifiersKeys,
}

impl Modifiers {
    pub fn new(state: ModifiersState, pressed_mods: ModifiersKeys) -> Self {
        Self {
            state,
            pressed_mods,
        }
    }

    pub fn state(&self) -> ModifiersState {
        self.state
    }

    pub fn lshift_state(&self) -> ModifiersKeyState {
        self.mod_state(ModifiersKeys::LSHIFT)
    }

    pub fn rshift_state(&self) -> ModifiersKeyState {
        self.mod_state(ModifiersKeys::RSHIFT)
    }

    pub fn lalt_state(&self) -> ModifiersKeyState {
        self.mod_state(ModifiersKeys::LALT)
    }

    pub fn ralt_state(&self) -> ModifiersKeyState {
        self.mod_state(ModifiersKeys::RALT)
    }

    pub fn lcontrol_state(&self) -> ModifiersKeyState {
        self.mod_state(ModifiersKeys::LCONTROL)
    }

    pub fn rcontrol_state(&self) -> ModifiersKeyState {
        self.mod_state(ModifiersKeys::RCONTROL)
    }

    pub fn lsuper_state(&self) -> ModifiersKeyState {
        self.mod_state(ModifiersKeys::LMETA)
    }

    pub fn rsuper_state(&self) -> ModifiersKeyState {
        self.mod_state(ModifiersKeys::RMETA)
    }

    fn mod_state(&self, modifier: ModifiersKeys) -> ModifiersKeyState {
        if self.pressed_mods.contains(modifier) {
            ModifiersKeyState::Pressed
        } else {
            ModifiersKeyState::Unknown
        }
    }
}

impl From<ModifiersState> for Modifiers {
    fn from(value: ModifiersState) -> Self {
        Self {
            state: value,
            pressed_mods: Default::default(),
        }
    }
}
