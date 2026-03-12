use bitflags;

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
