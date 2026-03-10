use crate::monitor::{MonitorHandler, VideoMode};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fullscreen<MH: MonitorHandler> {
    Exclusive(MH, VideoMode),
    Borderless(Option<MH>),
}
