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
