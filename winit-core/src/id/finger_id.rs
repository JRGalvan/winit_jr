#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FingerId(pub(crate) usize);

impl FingerId {
    pub const fn into_raw(self) -> usize {
        self.0
    }

    pub const fn from_raw(id: usize) -> Self {
        Self(id)
    }
}
