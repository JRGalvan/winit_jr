use bitflags;

bitflags::bitflags! {
    #[non_exhaustive]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    pub struct ImeHint: u32 {
        const NONE = 0;
        const COMPLETION = 0x1;
        const SPELLCHECK = 0x2;
        const AUTO_CAPITALIZATION = 0x4;
        const LOWERCASE = 0x8;
        const UPPERCASE = 0x10;
        const TITLECASE = 0x20;
        const HIDDEN_TEXT = 0x40;
        const SENSITIVE_DATA = 0x80;
        const LATIN = 0x100;
        const MULTILINE = 0x200;
    }
}
