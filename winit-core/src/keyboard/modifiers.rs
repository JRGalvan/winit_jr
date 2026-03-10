use bitflags;

bitflags::bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ModifiersState: u32 {
        const SHIFT = 0b100;
        const CONTROL = 0b100 << 3;
        const ALT = 0b100 << 6;
        const META = 0b100 << 9;
    }
}

impl ModifiersState {
    pub fn shift_key(&self) -> bool {
        self.intersects(Self::SHIFT)
    }

    pub fn control_key(&self) -> bool {
        self.intersects(Self::CONTROL)
    }

    pub fn alt_key(&self) -> bool {
        self.intersects(Self::ALT)
    }

    pub fn meta_key(&self) -> bool {
        self.intersects(Self::META)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ModifiersKeyState {
    Pressed,
    #[default]
    Unknown,
}

bitflags::bitflags! {
    #[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ModifiersKeys: u8 {
        const LSHIFT   = 0b0000_0001;
        const RSHIFT   = 0b0000_0010;
        const LCONTROL = 0b0000_0100;
        const RCONTROL = 0b0000_1000;
        const LALT     = 0b0001_0000;
        const RALT     = 0b0010_0000;
        const LMETA    = 0b0100_0000;
        const RMETA    = 0b1000_0000;
    }
}
