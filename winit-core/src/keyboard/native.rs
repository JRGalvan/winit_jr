use crate::BackendType;
use std::fmt::Debug;
use std::hash::Hash;

/// Maps to NativeKeyCode
pub trait NativePhysicalKey:
    Clone + Copy + Debug + PartialEq + Eq + PartialOrd + Ord + Hash
{
    fn is_unidentified(&self) -> bool;

    fn backend(&self) -> BackendType;
}

/// Maps to NativeKey
pub trait NativeLogicalKey:
    Clone + Copy + Debug + PartialEq + Eq + PartialOrd + Ord + Hash
{
    type PhysicalKey: NativePhysicalKey;

    fn is_unidentified(&self) -> bool;

    fn backend(&self) -> BackendType;

    fn to_physical_key(self) -> Option<Self::PhysicalKey>;
}
