use crate::monitor::{MonitorHandle, VideoMode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fullscreen<MH: MonitorHandle> {
    Exclusive(MH, VideoMode),
    Borderless(Option<MH>),
}
