use crate::keyboard::{ElementState, Key, NativePhysicalKey, PhysicalKey};
use keyboard_types::Location as KeyLocation;
use smol_str::SmolStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct RawKeyEvent<N: NativePhysicalKey> {
    pub physical_key: PhysicalKey<N>,
    pub state: ElementState,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct KeyEvent<N: NativePhysicalKey> {
    pub physical_key: PhysicalKey<N>,
    pub logical_key: Key<N>,
    pub text: Option<SmolStr>,
    pub location: KeyLocation,
    pub state: ElementState,
    pub repeat: bool,
    pub text_with_all_modifiers: Option<SmolStr>,
    pub key_without_modifiers: Key<N>,
}
