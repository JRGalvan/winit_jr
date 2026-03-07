pub mod modifiers;
pub mod native;

use keyboard_types::{Code as KeyCode, NamedKey};
use native::{NativeLogicalKey, NativePhysicalKey};
use smol_str::SmolStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalKey<N: NativePhysicalKey> {
    Code(KeyCode),
    Unidentified(N),
}

impl<N: NativePhysicalKey> From<KeyCode> for PhysicalKey<N> {
    #[inline]
    fn from(code: KeyCode) -> Self {
        PhysicalKey::Code(code)
    }
}

impl<N: NativePhysicalKey> From<PhysicalKey<N>> for KeyCode {
    #[inline]
    fn from(key: PhysicalKey<N>) -> Self {
        match key {
            PhysicalKey::Code(code) => code,
            PhysicalKey::Unidentified(_) => KeyCode::Unidentified,
        }
    }
}

impl<N: NativePhysicalKey> PartialEq<KeyCode> for PhysicalKey<N> {
    #[inline]
    fn eq(&self, rhs: &KeyCode) -> bool {
        match self {
            PhysicalKey::Code(code) => code == rhs,
            _ => false,
        }
    }
}

impl<N: NativePhysicalKey> PartialEq<PhysicalKey<N>> for KeyCode {
    #[inline]
    fn eq(&self, rhs: &PhysicalKey<N>) -> bool {
        rhs == self
    }
}

impl<N: NativePhysicalKey> PartialEq<N> for PhysicalKey<N> {
    #[inline]
    fn eq(&self, rhs: &N) -> bool {
        match self {
            PhysicalKey::Unidentified(code) => code == rhs,
            _ => false,
        }
    }
}

// Couldn't implement this
// impl PartialEq<PhysicalKey> for NativeKeyCode {
//     #[inline]
//     fn eq(&self, rhs: &PhysicalKey) -> bool {
//         rhs == self
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Key<N: NativePhysicalKey, Str = SmolStr> {
    Named(NamedKey),
    Character(Str),
    Unidentified(PhysicalKey<N>),
    Dead(Option<char>),
}

impl<N: NativePhysicalKey> From<NamedKey> for Key<N> {
    #[inline]
    fn from(action: NamedKey) -> Self {
        Key::Named(action)
    }
}

impl<N: NativePhysicalKey> From<PhysicalKey<N>> for Key<N> {
    #[inline]
    fn from(code: PhysicalKey<N>) -> Self {
        Key::Unidentified(code)
    }
}

impl<N: NativePhysicalKey, Str> PartialEq<NamedKey> for Key<N, Str> {
    #[inline]
    fn eq(&self, rhs: &NamedKey) -> bool {
        match self {
            Key::Named(a) => a == rhs,
            _ => false,
        }
    }
}

impl<N: NativePhysicalKey, Str: PartialEq<str>> PartialEq<str> for Key<N, Str> {
    #[inline]
    fn eq(&self, rhs: &str) -> bool {
        match self {
            Key::Character(s) => s == rhs,
            _ => false,
        }
    }
}

impl<N: NativePhysicalKey, Str: PartialEq<str>> PartialEq<&str> for Key<N, Str> {
    #[inline]
    fn eq(&self, rhs: &&str) -> bool {
        self == *rhs
    }
}

impl<N: NativePhysicalKey> PartialEq<PhysicalKey<N>> for Key<N> {
    #[inline]
    fn eq(&self, rhs: &PhysicalKey<N>) -> bool {
        match self {
            Key::Unidentified(code) => code == rhs,
            _ => false,
        }
    }
}

// Couldn't implement this
// impl<Str> PartialEq<Key<Str>> for NativeKey {
//     #[inline]
//     fn eq(&self, rhs: &Key<Str>) -> bool {
//         rhs == self
//     }
// }

impl<N: NativePhysicalKey> Key<N, SmolStr> {
    pub fn as_ref(&self) -> Key<N, &str> {
        match self {
            Key::Named(a) => Key::Named(*a),
            Key::Character(ch) => Key::Character(ch.as_str()),
            Key::Dead(d) => Key::Dead(*d),
            Key::Unidentified(u) => Key::Unidentified(u.clone()),
        }
    }
}

impl<N: NativePhysicalKey> Key<N> {
    pub fn to_text(&self) -> Option<&str> {
        match self {
            Key::Named(action) => match action {
                NamedKey::Enter => Some("\r"),
                NamedKey::Backspace => Some("\x08"),
                NamedKey::Tab => Some("\t"),
                NamedKey::Escape => Some("\x1b"),
                _ => None,
            },
            Key::Character(ch) => Some(ch.as_str()),
            _ => None,
        }
    }
}
